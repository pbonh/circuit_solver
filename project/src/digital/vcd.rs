//! VCD parser into the event model (interchange only).
//!
//! # Spec traceability
//!
//! - Scenario: `digital-equivalence#vcd-interchange-only`
//! - Task #21: VCD parser into the event model (interchange only; no acceptance
//!   depends on VCD bytes).
//!
//! # Design
//!
//! This module parses Value Change Dump (VCD) files (IEEE 1364) into the
//! [`EventTrace`] model defined in [`super::equivalence`].  VCD is treated
//! purely as an **interchange format** — the parser extracts the semantic
//! content (time, net, value) and discards everything else (date, version,
//! comments, exact whitespace).  No acceptance criterion depends on VCD byte
//! layout; the event model is the authoritative representation.
//!
//! # Supported VCD features
//!
//! - Four-state VCD: 0, 1, X, Z
//! - Single-bit scalar value changes
//! - Multi-bit bus value changes (each bit produces a separate event)
//! - `$timescale` parsing with SI-unit conversion to seconds
//! - `$var` declarations with arbitrary-length short identifiers
//! - Nested `$scope` ... `$upscope` hierarchies (signal names use `.` separator)
//! - `$dumpvars` initial value section (parsed as value changes at the
//!   current time point)
//!
//! # Limitations (by design)
//!
//! - Real-value VCD (`$var real ...`) is not supported — the event model only
//!   carries [`LogicValue`].
//! - Port direction, parameter, and comment sections are skipped.
//! - No write/dump support — VCD is read-only interchange.

use std::collections::HashMap;

use super::equivalence::{Event, EventTrace, LogicValue};

// ---------------------------------------------------------------------------
// Parse errors
// ---------------------------------------------------------------------------

/// Errors that can occur during VCD parsing.
#[derive(Clone, Debug, PartialEq)]
pub enum VcdParseError {
    /// The VCD input is empty or contains no header.
    EmptyInput,
    /// A required header section is missing.
    MissingHeader(String),
    /// The `$timescale` value could not be parsed.
    InvalidTimescale(String),
    /// A `$var` declaration is malformed.
    InvalidVarDecl(String),
    /// A time command (`#N`) could not be parsed as an integer.
    InvalidTime(String),
    /// An unknown short identifier appeared in a value change.
    UnknownIdentifier(String),
}

impl std::fmt::Display for VcdParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VcdParseError::EmptyInput => write!(f, "empty VCD input"),
            VcdParseError::MissingHeader(section) => {
                write!(f, "missing required VCD header section: {}", section)
            }
            VcdParseError::InvalidTimescale(ts) => {
                write!(f, "invalid VCD timescale: {}", ts)
            }
            VcdParseError::InvalidVarDecl(decl) => {
                write!(f, "invalid VCD $var declaration: {}", decl)
            }
            VcdParseError::InvalidTime(t) => {
                write!(f, "invalid VCD time command: {}", t)
            }
            VcdParseError::UnknownIdentifier(id) => {
                write!(f, "unknown VCD signal identifier: {}", id)
            }
        }
    }
}

impl std::error::Error for VcdParseError {}

// ---------------------------------------------------------------------------
// Timescale parsing
// ---------------------------------------------------------------------------

/// Supported VCD timescale units.
#[derive(Clone, Copy, Debug, PartialEq)]
enum TimescaleUnit {
    Femtosecond,
    Picosecond,
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
}

impl TimescaleUnit {
    /// Convert to seconds multiplier.
    fn to_seconds(self) -> f64 {
        match self {
            TimescaleUnit::Femtosecond => 1e-15,
            TimescaleUnit::Picosecond => 1e-12,
            TimescaleUnit::Nanosecond => 1e-9,
            TimescaleUnit::Microsecond => 1e-6,
            TimescaleUnit::Millisecond => 1e-3,
            TimescaleUnit::Second => 1.0,
        }
    }

    /// Parse a unit string like "fs", "ps", "ns", "us", "ms", "s".
    fn from_str_unit(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "fs" | "femtosecond" | "femtoseconds" => Some(TimescaleUnit::Femtosecond),
            "ps" | "picosecond" | "picoseconds" => Some(TimescaleUnit::Picosecond),
            "ns" | "nanosecond" | "nanoseconds" => Some(TimescaleUnit::Nanosecond),
            "us" | "µs" | "microsecond" | "microseconds" => Some(TimescaleUnit::Microsecond),
            "ms" | "millisecond" | "milliseconds" => Some(TimescaleUnit::Millisecond),
            "s" | "second" | "seconds" => Some(TimescaleUnit::Second),
            _ => None,
        }
    }
}

/// Parse a VCD `$timescale` value like "1 ns", "10ps", "100 us".
///
/// Returns the number of seconds per VCD time unit.
fn parse_timescale(ts: &str) -> Result<f64, VcdParseError> {
    let ts = ts.trim();
    if ts.is_empty() {
        return Err(VcdParseError::InvalidTimescale("empty timescale".into()));
    }

    let unit_start = ts
        .find(|c: char| c.is_ascii_alphabetic())
        .ok_or_else(|| VcdParseError::InvalidTimescale(ts.to_string()))?;

    let num_str = ts[..unit_start].trim();
    let unit_str = &ts[unit_start..];

    let multiplier: f64 = num_str
        .parse()
        .map_err(|_| VcdParseError::InvalidTimescale(ts.to_string()))?;

    let unit = TimescaleUnit::from_str_unit(unit_str)
        .ok_or_else(|| VcdParseError::InvalidTimescale(ts.to_string()))?;

    Ok(multiplier * unit.to_seconds())
}

// ---------------------------------------------------------------------------
// Signal registry
// ---------------------------------------------------------------------------

/// Maps a VCD short identifier (e.g., "!", "aB") to the full hierarchical
/// signal name and its bit width.
#[derive(Clone, Debug)]
struct SignalInfo {
    /// Full hierarchical name (e.g., "top.clk").
    name: String,
    /// Bit width (1 for scalar, >1 for bus).
    width: usize,
}

// ---------------------------------------------------------------------------
// VCD parser
// ---------------------------------------------------------------------------

/// A VCD parser that produces an [`EventTrace`].
///
/// Usage:
/// ```ignore
/// let trace = VcdParser::parse(&vcd_text)?;
/// ```
pub struct VcdParser;

impl VcdParser {
    /// Parse a VCD string into an [`EventTrace`].
    ///
    /// The parser performs two logical passes:
    /// 1. **Header pass**: extract `$timescale`, `$var` declarations, and
    ///    build the signal registry (short-id → full name).
    /// 2. **Value-change pass**: walk the value-change section, converting
    ///    each value change into an [`Event`].
    ///
    /// The returned `EventTrace` is sorted by (time, net) and deduplicated,
    /// ready for equivalence checking.
    pub fn parse(input: &str) -> Result<EventTrace, VcdParseError> {
        if input.trim().is_empty() {
            return Err(VcdParseError::EmptyInput);
        }

        let tokens = tokenize(input);
        let (timescale_seconds, signals, value_start) = parse_header(&tokens)?;
        let events = parse_value_changes(&tokens, value_start, &signals, timescale_seconds)?;

        Ok(EventTrace::from_unsorted(events))
    }
}

// ---------------------------------------------------------------------------
// Tokenizer
// ---------------------------------------------------------------------------

/// A VCD token.
#[derive(Clone, Debug, PartialEq)]
enum Token {
    /// A `$keyword` command.
    Command(String),
    /// A plain word.
    Word(String),
}

/// Tokenize a VCD string into a sequence of [`Token`] values.
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for line in input.lines() {
        let line = line.split("//").next().unwrap_or(line);
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        for word in line.split_whitespace() {
            if word.starts_with('$') {
                tokens.push(Token::Command(word.to_string()));
            } else {
                tokens.push(Token::Word(word.to_string()));
            }
        }
    }

    tokens
}

// ---------------------------------------------------------------------------
// Header parser
// ---------------------------------------------------------------------------

/// Parse the VCD header, returning:
/// - `timescale_seconds`: seconds per VCD time unit
/// - `signals`: short-id → SignalInfo mapping
/// - `value_start`: index into `tokens` where value changes begin
fn parse_header(
    tokens: &[Token],
) -> Result<(f64, HashMap<String, SignalInfo>, usize), VcdParseError> {
    let mut timescale_seconds: Option<f64> = None;
    let mut signals: HashMap<String, SignalInfo> = HashMap::new();
    let mut scope_stack: Vec<String> = Vec::new();
    let mut value_start = tokens.len();

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Command(cmd) => match cmd.as_str() {
                "$timescale" => {
                    let mut ts_parts = Vec::new();
                    i += 1;
                    while i < tokens.len() {
                        match &tokens[i] {
                            Token::Command(c) if c == "$end" => break,
                            Token::Word(w) => ts_parts.push(w.clone()),
                            _ => {}
                        }
                        i += 1;
                    }
                    let ts_str = ts_parts.join(" ");
                    timescale_seconds = Some(parse_timescale(&ts_str)?);
                }
                "$scope" => {
                    i += 1;
                    if i < tokens.len() {
                        i += 1; // skip type
                    }
                    if i < tokens.len() {
                        if let Token::Word(name) = &tokens[i] {
                            scope_stack.push(name.clone());
                        }
                    }
                    while i < tokens.len() {
                        if matches!(&tokens[i], Token::Command(c) if c == "$end") {
                            break;
                        }
                        i += 1;
                    }
                }
                "$upscope" => {
                    scope_stack.pop();
                    i += 1;
                    while i < tokens.len() {
                        if matches!(&tokens[i], Token::Command(c) if c == "$end") {
                            break;
                        }
                        i += 1;
                    }
                }
                "$var" => {
                    i += 1;
                    let mut var_tokens = Vec::new();
                    while i < tokens.len() {
                        match &tokens[i] {
                            Token::Command(c) if c == "$end" => break,
                            Token::Word(w) => var_tokens.push(w.clone()),
                            _ => {}
                        }
                        i += 1;
                    }
                    if var_tokens.len() < 4 {
                        return Err(VcdParseError::InvalidVarDecl(var_tokens.join(" ")));
                    }
                    let width: usize = var_tokens[1]
                        .parse()
                        .map_err(|_| VcdParseError::InvalidVarDecl(var_tokens.join(" ")))?;
                    let short_id = &var_tokens[2];
                    let local_name = &var_tokens[3];
                    let full_name = if scope_stack.is_empty() {
                        local_name.clone()
                    } else {
                        format!("{}.{}", scope_stack.join("."), local_name)
                    };
                    signals.insert(short_id.clone(), SignalInfo { name: full_name, width });
                }
                "$enddefinition" => {
                    i += 1;
                    if i < tokens.len()
                        && matches!(&tokens[i], Token::Command(c) if c == "$end")
                    {
                        i += 1;
                    }
                    value_start = i;
                    break;
                }
                // Skip other header sections ($date, $version, $comment, etc.)
                _ => {
                    i += 1;
                    while i < tokens.len() {
                        if matches!(&tokens[i], Token::Command(c) if c == "$end") {
                            break;
                        }
                        i += 1;
                    }
                }
            },
            Token::Word(w) => {
                if w.starts_with('#') {
                    value_start = i;
                    break;
                }
            }
        }
        i += 1;
    }

    let ts = timescale_seconds
        .ok_or_else(|| VcdParseError::MissingHeader("$timescale".into()))?;

    if signals.is_empty() {
        return Err(VcdParseError::MissingHeader("$var".into()));
    }

    Ok((ts, signals, value_start))
}

// ---------------------------------------------------------------------------
// Value-change parser (single pass)
// ---------------------------------------------------------------------------

/// Commands that contain value changes and should be parsed (not skipped).
fn is_value_change_command(cmd: &str) -> bool {
    cmd == "$dumpvars"
}

/// Commands that should be skipped entirely (content + $end).
fn is_skip_command(cmd: &str) -> bool {
    cmd == "$dumpon" || cmd == "$dumpoff" || cmd == "$dumpall" || cmd == "$comment"
}

/// Parse value changes starting from `value_start` in `tokens`.
fn parse_value_changes(
    tokens: &[Token],
    value_start: usize,
    signals: &HashMap<String, SignalInfo>,
    timescale_seconds: f64,
) -> Result<Vec<Event>, VcdParseError> {
    let mut events = Vec::new();
    let mut current_time_vcd: u64 = 0;
    let mut i = value_start;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Word(w) => {
                if let Some(time_str) = w.strip_prefix('#') {
                    current_time_vcd = time_str
                        .parse()
                        .map_err(|_| VcdParseError::InvalidTime(w.clone()))?;
                } else if let Some(bus_value) = w.strip_prefix('b').or_else(|| w.strip_prefix('B')) {
                    // Bus value change: b<binary> followed by short_id
                    i += 1;
                    if i < tokens.len() {
                        if let Token::Word(short_id) = &tokens[i] {
                            let sig = signals
                                .get(short_id)
                                .ok_or_else(|| VcdParseError::UnknownIdentifier(short_id.to_string()))?;
                            let time_seconds = current_time_vcd as f64 * timescale_seconds;
                            let padded = if bus_value.len() < sig.width {
                                let pad = sig.width - bus_value.len();
                                "0".repeat(pad) + bus_value
                            } else {
                                bus_value.to_string()
                            };
                            for (bit_idx, ch) in padded.chars().enumerate() {
                                if let Some(value) = LogicValue::from_char(ch) {
                                    let bit_name = if sig.width > 1 {
                                        format!("{}[{}]", sig.name, sig.width - 1 - bit_idx)
                                    } else {
                                        sig.name.clone()
                                    };
                                    events.push(Event::new(time_seconds, bit_name, value));
                                }
                            }
                        }
                    }
                } else if w.len() >= 2 {
                    let value_char = w.chars().next().unwrap();
                    let short_id = &w[1..];
                    if let Some(value) = LogicValue::from_char(value_char) {
                        let sig = signals
                            .get(short_id)
                            .ok_or_else(|| VcdParseError::UnknownIdentifier(short_id.to_string()))?;
                        let time_seconds = current_time_vcd as f64 * timescale_seconds;
                        events.push(Event::new(time_seconds, &sig.name, value));
                    }
                }
            }
            Token::Command(cmd) => {
                if is_value_change_command(cmd) {
                    // $dumpvars body is parsed as value changes.
                } else if is_skip_command(cmd) {
                    i += 1;
                    while i < tokens.len() {
                        if matches!(&tokens[i], Token::Command(c) if c == "$end") {
                            break;
                        }
                        i += 1;
                    }
                }
            }
        }
        i += 1;
    }

    Ok(events)
}

// ---------------------------------------------------------------------------
// VCD writer (event model → VCD string)
// ---------------------------------------------------------------------------

/// Split a hierarchical net name into scope components and leaf name.
///
/// "top.sub.clk"  → (["top", "sub"], "clk")
/// "clk"           → ([], "clk")
/// "top.data[0]"   → (["top"], "data[0]")
fn split_net_hierarchy(net: &str) -> (Vec<&str>, &str) {
    let mut last_dot: Option<usize> = None;
    let mut in_bracket = false;
    for (i, ch) in net.char_indices() {
        match ch {
            '[' => in_bracket = true,
            ']' => in_bracket = false,
            '.' if !in_bracket => last_dot = Some(i),
            _ => {}
        }
    }
    match last_dot {
        Some(dot) => {
            let scope_part = &net[..dot];
            let leaf = &net[dot + 1..];
            let scopes: Vec<&str> = scope_part.split('.').collect();
            (scopes, leaf)
        }
        None => (vec![], net),
    }
}

/// Convert a [`LogicValue`] to a char for VCD output.
fn logic_value_to_char(v: LogicValue) -> char {
    match v {
        LogicValue::Zero => '0',
        LogicValue::One => '1',
        LogicValue::X => 'x',
        LogicValue::Z => 'z',
    }
}

/// Convert an [`EventTrace`] to a VCD string.
///
/// The produced VCD is semantically equivalent to the input event trace, but
/// no guarantee is made about byte-level identity.  This is consistent with
/// the spec: "VCD is treated as interchange only; no acceptance depends on
/// VCD bytes."
pub fn trace_to_vcd(trace: &EventTrace, timescale: &str) -> String {
    let mut out = String::new();

    // Collect unique net names.
    let mut net_names: Vec<String> = trace.iter().map(|e| e.net.clone()).collect();
    net_names.sort();
    net_names.dedup();

    // Group bus bits: "data[0]", "data[1]" → bus group "data".
    let mut bus_groups: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    let mut scalar_nets: Vec<String> = Vec::new();

    for name in &net_names {
        if let Some(idx) = name.rfind('[') {
            if let Some(be) = name.rfind(']') {
                if be == name.len() - 1 {
                    let base = &name[..idx];
                    let bit_str = &name[idx + 1..be];
                    if let Ok(bit) = bit_str.parse::<usize>() {
                        bus_groups
                            .entry(base.to_string())
                            .or_default()
                            .push((bit, name.clone()));
                        continue;
                    }
                }
            }
        }
        scalar_nets.push(name.clone());
    }

    // Sort bus bits descending (MSB first).
    for bits in bus_groups.values_mut() {
        bits.sort_by_key(|b| std::cmp::Reverse(b.0));
    }

    // Build var entries list.
    struct VarEntry {
        full_name: String,
        width: usize,
        leaf_name: String,
    }

    let mut var_entries: Vec<VarEntry> = Vec::new();

    for name in &scalar_nets {
        let (_, leaf) = split_net_hierarchy(name);
        var_entries.push(VarEntry {
            full_name: name.clone(),
            width: 1,
            leaf_name: leaf.to_string(),
        });
    }

    let mut bus_bases: Vec<String> = bus_groups.keys().cloned().collect();
    bus_bases.sort();
    for base in &bus_bases {
        let (_, leaf) = split_net_hierarchy(base);
        let bits = &bus_groups[base];
        var_entries.push(VarEntry {
            full_name: base.clone(),
            width: bits.len(),
            leaf_name: leaf.to_string(),
        });
    }

    // Assign short identifiers.
    let id_chars: Vec<char> = (33..=126).map(|c| c as u8 as char).collect();
    let mut net_to_id: HashMap<String, String> = HashMap::new();

    for (next_id_idx, entry) in var_entries.iter().enumerate() {
        let id = if next_id_idx < id_chars.len() {
            id_chars[next_id_idx].to_string()
        } else {
            let hi = next_id_idx / id_chars.len();
            let lo = next_id_idx % id_chars.len();
            format!("{}{}", id_chars[hi], id_chars[lo])
        };
        net_to_id.insert(entry.full_name.clone(), id);
    }

    // Determine common scope hierarchy.
    let mut all_scopes: Vec<Vec<&str>> = Vec::new();
    for name in &net_names {
        let (scopes, _) = split_net_hierarchy(name);
        if !scopes.is_empty() {
            all_scopes.push(scopes);
        }
    }

    let common_depth = if all_scopes.is_empty() {
        0
    } else if all_scopes.len() == 1 {
        all_scopes[0].len()
    } else {
        let mut d = 0;
        loop {
            let mut all_match = true;
            let mut any_have = false;
            for scopes in &all_scopes {
                if scopes.len() > d {
                    any_have = true;
                    if scopes[d] != *all_scopes[0].get(d).unwrap_or(&"") {
                        all_match = false;
                        break;
                    }
                }
            }
            if !all_match || !any_have {
                break;
            }
            d += 1;
        }
        d
    };

    // Emit header.
    out.push_str("$date\n   Interchange output\n$end\n");
    out.push_str("$version\n   circuit-solver VCD interchange\n$end\n");
    out.push_str(&format!("$timescale\n   {}\n$end\n", timescale));

    if common_depth > 0 {
        for scope_name in all_scopes[0].iter().take(common_depth) {
            out.push_str(&format!("$scope module {} $end\n", scope_name));
        }
    } else if !net_names.is_empty() {
        // If there's no common scope, check whether any nets have scope prefixes.
        // If some nets are scoped but share no common prefix, create a "top" scope.
        // If all nets are flat (no scope), emit without $scope — flat VCD.
        let any_scoped = net_names.iter().any(|n| {
            let (s, _) = split_net_hierarchy(n);
            !s.is_empty()
        });
        if any_scoped {
            out.push_str("$scope module top $end\n");
        }
    }

    for entry in &var_entries {
        let id = net_to_id.get(&entry.full_name).unwrap();
        out.push_str(&format!(
            "$var wire {} {} {} $end\n",
            entry.width, id, entry.leaf_name
        ));
    }

    let any_scoped = net_names.iter().any(|n| {
        let (s, _) = split_net_hierarchy(n);
        !s.is_empty()
    });
    let n_scopes = if common_depth > 0 {
        common_depth
    } else if any_scoped {
        1 // we added a "top" scope
    } else {
        0 // flat VCD, no scope emitted
    };
    for _ in 0..n_scopes {
        out.push_str("$upscope $end\n");
    }
    out.push_str("$enddefinition $end\n");

    let ts_seconds = parse_timescale(timescale).unwrap_or(1e-9);

    // Value changes.
    let events = trace.as_slice();
    let mut i = 0;
    let mut last_time: Option<f64> = None;

    while i < events.len() {
        let event_time = events[i].time;
        if last_time != Some(event_time) {
            let time_int = (event_time / ts_seconds).round() as u64;
            out.push_str(&format!("#{}\n", time_int));
            last_time = Some(event_time);
        }

        // Check if this event is a bus bit.
        let bus_base: Option<String> = if let Some(idx) = events[i].net.rfind('[') {
            if let Some(be) = events[i].net.rfind(']') {
                if be == events[i].net.len() - 1 {
                    Some(events[i].net[..idx].to_string())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(ref base) = bus_base {
            if let Some(bits) = bus_groups.get(base) {
                let width = bits.len();
                let mut bit_values = vec![LogicValue::X; width];

                while i < events.len()
                    && events[i].time == event_time
                    && events[i].net.starts_with(base.as_str())
                    && events[i].net.len() > base.len()
                    && events[i].net.as_bytes()[base.len()] == b'['
                {
                    let net = &events[i].net;
                    if let Some(bs) = net.rfind('[') {
                        if let Some(be) = net.rfind(']') {
                            if be == net.len() - 1 {
                                if let Ok(bit_idx) = net[bs + 1..be].parse::<usize>() {
                                    if bit_idx < width {
                                        bit_values[width - 1 - bit_idx] = events[i].value;
                                    }
                                }
                            }
                        }
                    }
                    i += 1;
                }

                if let Some(id) = net_to_id.get(base) {
                    out.push('b');
                    for v in &bit_values {
                        out.push(logic_value_to_char(*v));
                    }
                    out.push(' ');
                    out.push_str(id);
                    out.push('\n');
                }
            } else {
                // Not in bus_groups (shouldn't happen), skip.
                i += 1;
            }
        } else {
            let id = net_to_id.get(&events[i].net);
            if let Some(sid) = id {
                out.push_str(&format!("{}{}\n", events[i].value, sid));
            }
            i += 1;
        }
    }

    out
}

// ---------------------------------------------------------------------------
// Unit tests (inline)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digital::equivalence::{check_equivalence, EquivalenceTolerance};

    #[test]
    fn timescale_1ns() {
        assert!((parse_timescale("1 ns").unwrap() - 1e-9).abs() < 1e-15);
    }

    #[test]
    fn timescale_10ps() {
        assert!((parse_timescale("10ps").unwrap() - 10e-12).abs() < 1e-15);
    }

    #[test]
    fn timescale_100us() {
        assert!((parse_timescale("100 us").unwrap() - 100e-6).abs() < 1e-9);
    }

    #[test]
    fn timescale_1s() {
        assert!((parse_timescale("1 s").unwrap() - 1.0).abs() < 1e-15);
    }

    #[test]
    fn timescale_invalid() {
        assert!(parse_timescale("abc").is_err());
        assert!(parse_timescale("").is_err());
        assert!(parse_timescale("5 xyz").is_err());
    }

    #[test]
    fn parse_minimal_vcd() {
        let vcd = "\
$date
   2026-01-01
$end
$version
   test
$end
$timescale
   1 ns
$end
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinition $end
#0
0!
#5
1!
#10
0!
";
        let trace = VcdParser::parse(vcd).unwrap();
        assert_eq!(trace.len(), 3);

        let events = trace.as_slice();
        assert_eq!(events[0].time, 0.0);
        assert_eq!(events[0].net, "top.clk");
        assert_eq!(events[0].value, LogicValue::Zero);
        assert_eq!(events[1].time, 5e-9);
        assert_eq!(events[1].net, "top.clk");
        assert_eq!(events[1].value, LogicValue::One);
        assert_eq!(events[2].time, 10e-9);
        assert_eq!(events[2].net, "top.clk");
        assert_eq!(events[2].value, LogicValue::Zero);
    }

    #[test]
    fn parse_empty_input() {
        let result = VcdParser::parse("");
        assert!(matches!(result, Err(VcdParseError::EmptyInput)));
    }

    #[test]
    fn parse_missing_timescale() {
        let vcd = "\
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinition $end
#0
0!
";
        let result = VcdParser::parse(vcd);
        assert!(matches!(result, Err(VcdParseError::MissingHeader(_))));
    }

    #[test]
    fn parse_missing_vars() {
        let vcd = "\
$timescale
   1 ns
$end
$scope module top $end
$upscope $end
$enddefinition $end
#0
";
        let result = VcdParser::parse(vcd);
        assert!(matches!(result, Err(VcdParseError::MissingHeader(_))));
    }

    #[test]
    fn parse_x_and_z_values() {
        let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! sig $end
$upscope $end
$enddefinition $end
#0
x!
#5
z!
#10
1!
";
        let trace = VcdParser::parse(vcd).unwrap();
        assert_eq!(trace.len(), 3);

        let events = trace.as_slice();
        assert_eq!(events[0].value, LogicValue::X);
        assert_eq!(events[1].value, LogicValue::Z);
        assert_eq!(events[2].value, LogicValue::One);
    }

    #[test]
    fn parse_hierarchical_scope() {
        let vcd = "\
$timescale 1 ns $end
$scope module top $end
$scope module sub $end
$var wire 1 ! clk $end
$upscope $end
$upscope $end
$enddefinition $end
#0
1!
";
        let trace = VcdParser::parse(vcd).unwrap();
        assert_eq!(trace.len(), 1);
        assert_eq!(trace.as_slice()[0].net, "top.sub.clk");
    }

    #[test]
    fn parse_unknown_identifier() {
        let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinition $end
#0
1@
";
        let result = VcdParser::parse(vcd);
        assert!(matches!(result, Err(VcdParseError::UnknownIdentifier(_))));
    }

    #[test]
    fn parse_bus_value_change() {
        let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 4 ! data $end
$upscope $end
$enddefinition $end
#0
b0101 !
#5
b1111 !
";
        let trace = VcdParser::parse(vcd).unwrap();
        assert_eq!(trace.len(), 8);

        let events = trace.as_slice();
        assert_eq!(events[0].net, "top.data[0]");
        assert_eq!(events[0].value, LogicValue::One);
        assert_eq!(events[1].net, "top.data[1]");
        assert_eq!(events[1].value, LogicValue::Zero);
        assert_eq!(events[2].net, "top.data[2]");
        assert_eq!(events[2].value, LogicValue::One);
        assert_eq!(events[3].net, "top.data[3]");
        assert_eq!(events[3].value, LogicValue::Zero);
    }

    #[test]
    fn vcd_round_trip_semantic() {
        let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$var wire 1 \" rst $end
$upscope $end
$enddefinition $end
#0
1!
0\"
#10
0!
1\"
#20
1!
0\"
";
        let trace = VcdParser::parse(vcd).unwrap();
        let vcd_out = trace_to_vcd(&trace, "1 ns");
        let trace2 = VcdParser::parse(&vcd_out).unwrap();

        let result = check_equivalence(&trace, &trace2, &EquivalenceTolerance::exact());
        assert!(result.equivalent, "round-trip failed: {}", result);
    }

    #[test]
    fn parse_invalid_time() {
        let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinition $end
#abc
1!
";
        let result = VcdParser::parse(vcd);
        assert!(matches!(result, Err(VcdParseError::InvalidTime(_))));
    }

    #[test]
    fn parse_with_comments() {
        let vcd = "\
// This is a comment
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinition $end
#0  // initial time
0!
#5
1!
";
        let trace = VcdParser::parse(vcd).unwrap();
        assert_eq!(trace.len(), 2);
    }

    #[test]
    fn parse_multiple_signals_same_time() {
        let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$var wire 1 \" data $end
$upscope $end
$enddefinition $end
#0
0!
1\"
#5
1!
0\"
";
        let trace = VcdParser::parse(vcd).unwrap();
        assert_eq!(trace.len(), 4);

        let events = trace.as_slice();
        assert_eq!(events[0].net, "top.clk");
        assert_eq!(events[0].value, LogicValue::Zero);
        assert_eq!(events[1].net, "top.data");
        assert_eq!(events[1].value, LogicValue::One);
    }

    #[test]
    fn parse_dumpvars_contains_values() {
        let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinition $end
$dumpvars
0!
$end
#5
1!
$dumpon $end
#10
0!
$dumpoff $end
";
        let trace = VcdParser::parse(vcd).unwrap();
        assert_eq!(trace.len(), 3);

        let events = trace.as_slice();
        assert_eq!(events[0].value, LogicValue::Zero);
        assert_eq!(events[0].time, 0.0);
        assert_eq!(events[1].value, LogicValue::One);
        assert_eq!(events[1].time, 5e-9);
        assert_eq!(events[2].value, LogicValue::Zero);
        assert_eq!(events[2].time, 10e-9);
    }

    #[test]
    fn parse_no_scope_flat_vcd() {
        let vcd = "\
$timescale 1 ns $end
$var wire 1 ! clk $end
$enddefinition $end
#0
0!
#5
1!
";
        let trace = VcdParser::parse(vcd).unwrap();
        assert_eq!(trace.len(), 2);
        assert_eq!(trace.as_slice()[0].net, "clk");
    }
}
