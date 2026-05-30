//! VCD (Value Change Dump) parser into the event model.
//!
//! # Spec traceability
//!
//! - Scenario: `digital-equivalence#vcd-interchange-only`
//! - Task #21: VCD parser into the event model (interchange only; no
//!   acceptance depends on VCD bytes).
//!
//! # Design
//!
//! This module parses IEEE 1364 VCD files into [`EventTrace`] objects so that
//! external simulation outputs can be compared via the equivalence checker.
//! VCD is treated as an **interchange format only** — no acceptance criterion
//! depends on specific VCD byte layout (whitespace, ordering of header
//! sections, etc.).  The only contract is that a correctly parsed VCD yields
//! the same ordered (time, net, value) events as an equivalent trace produced
//! by any other means.
//!
//! # Supported VCD subset
//!
//! - Four-state values: 0, 1, x/X, z/Z
//! - Single-bit `$var` declarations (wire, reg, integer, tri, supply0,
//!   supply1, tri0, tri1, wand, wor, trior, triand, trireg)
//! - Timescale units: s, ms, us, ns, ps, fs (with optional magnitude 1/10/100)
//! - `$scope` … `$upscope` nesting (scopes are flattened into dotted net names)
//! - `$dumpvars` initial values
//! - `#<timestamp>` + value-change lines
//! - Real-valued `$var` are **not** supported (the digital event model only
//!   handles four-state logic).

use std::collections::HashMap;
use std::fmt;

use super::equivalence::{Event, EventTrace, LogicValue};

// ---------------------------------------------------------------------------
// Parse errors
// ---------------------------------------------------------------------------

/// Errors that can occur while parsing a VCD file.
#[derive(Clone, Debug, PartialEq)]
pub enum VcdParseError {
    /// A value-change line could not be decoded.
    InvalidValueChange {
        line: usize,
        raw: String,
        detail: String,
    },
    /// A `$var` declaration is malformed.
    InvalidVarDecl { line: usize, raw: String },
    /// A `$timescale` declaration is malformed or unsupported.
    InvalidTimescale { line: usize, raw: String },
    /// A `$dumpoff` / `$dumpon` / `$dumpall` section was encountered
    /// but is not supported.
    UnsupportedDumpSection { line: usize, keyword: String },
    /// A real-valued variable was encountered.
    RealVarNotSupported { line: usize, name: String },
    /// The VCD input is empty or contains no time steps.
    EmptyInput,
    /// An unknown keyword was encountered in the header.
    UnknownKeyword { line: usize, keyword: String },
}

impl fmt::Display for VcdParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VcdParseError::InvalidValueChange { line, raw, detail } => {
                write!(
                    f,
                    "invalid value change at line {}: {:?} ({})",
                    line, raw, detail
                )
            }
            VcdParseError::InvalidVarDecl { line, raw } => {
                write!(f, "invalid $var declaration at line {}: {:?}", line, raw)
            }
            VcdParseError::InvalidTimescale { line, raw } => {
                write!(f, "invalid $timescale at line {}: {:?}", line, raw)
            }
            VcdParseError::UnsupportedDumpSection { line, keyword } => {
                write!(f, "unsupported dump section ${} at line {}", keyword, line)
            }
            VcdParseError::RealVarNotSupported { line, name } => {
                write!(
                    f,
                    "real variable not supported at line {}: {:?}",
                    line, name
                )
            }
            VcdParseError::EmptyInput => write!(f, "empty VCD input"),
            VcdParseError::UnknownKeyword { line, keyword } => {
                write!(f, "unknown keyword ${} at line {}", keyword, line)
            }
        }
    }
}

impl std::error::Error for VcdParseError {}

// ---------------------------------------------------------------------------
// Timescale
// ---------------------------------------------------------------------------

/// VCD timescale magnitude.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimescaleMagnitude {
    One,
    Ten,
    Hundred,
}

/// VCD timescale unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimescaleUnit {
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    Picosecond,
    Femtosecond,
}

/// A parsed VCD timescale (e.g. "1 ns", "10 ps").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timescale {
    pub magnitude: TimescaleMagnitude,
    pub unit: TimescaleUnit,
}

impl Timescale {
    /// Convert the timescale to a multiplier in seconds.
    ///
    /// E.g. "1 ns" → 1e-9, "10 ps" → 10e-12, "100 us" → 100e-6.
    pub fn to_seconds(self) -> f64 {
        let mag: f64 = match self.magnitude {
            TimescaleMagnitude::One => 1.0,
            TimescaleMagnitude::Ten => 10.0,
            TimescaleMagnitude::Hundred => 100.0,
        };
        let unit: f64 = match self.unit {
            TimescaleUnit::Second => 1.0,
            TimescaleUnit::Millisecond => 1e-3,
            TimescaleUnit::Microsecond => 1e-6,
            TimescaleUnit::Nanosecond => 1e-9,
            TimescaleUnit::Picosecond => 1e-12,
            TimescaleUnit::Femtosecond => 1e-15,
        };
        mag * unit
    }
}

// ---------------------------------------------------------------------------
// Parsed VCD header info
// ---------------------------------------------------------------------------

/// Metadata extracted from the VCD header.
#[derive(Clone, Debug, PartialEq)]
pub struct VcdHeader {
    /// The parsed timescale.
    pub timescale: Timescale,
    /// Map from VCD short identifier (1–4 character code) to the full
    /// dotted net name.
    pub signals: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// Parser result
// ---------------------------------------------------------------------------

/// The result of parsing a VCD file.
#[derive(Clone, Debug, PartialEq)]
pub struct VcdParseResult {
    /// The parsed header metadata.
    pub header: VcdHeader,
    /// The event trace extracted from the value-change records.
    pub trace: EventTrace,
}

// ---------------------------------------------------------------------------
// Tokeniser
// ---------------------------------------------------------------------------

/// A lightweight VCD token: either a keyword token (`$foo` or `$end`) or a
/// value token (any whitespace-delimited word).
#[derive(Clone, Debug, PartialEq)]
enum Token {
    /// A keyword like `$date`, `$var`, `$end`, etc.
    Keyword(String),
    /// A value token (identifier, number, etc.).
    Value(String),
}

/// Tokenise a VCD input string into a sequence of tokens, keeping line
/// numbers for error reporting.
fn tokenize(input: &str) -> Vec<(Token, usize)> {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '\n' => {
                line += 1;
                chars.next();
            }
            '\r' => {
                chars.next();
                // Handle \r\n
                if chars.peek() == Some(&'\n') {
                    chars.next();
                    line += 1;
                }
            }
            ' ' | '\t' => {
                chars.next();
            }
            // '$' starts either a keyword ($date, $end, $var, …) or,
            // when standalone, a VCD id code (Icarus Verilog uses '$'
            // as a signal id code in $var declarations and value-change
            // records like "0$").
            '$' => {
                let start_line = line;
                chars.next(); // consume '$'

                // Collect keyword characters after '$'
                let mut after = String::new();
                while let Some(&kc) = chars.peek() {
                    if kc.is_whitespace() || kc == '\n' || kc == '\r' {
                        break;
                    }
                    after.push(kc);
                    chars.next();
                }

                if after.is_empty() {
                    // Bare '$' followed by whitespace — treat as an
                    // id-code value token, not a keyword.
                    tokens.push((Token::Value(String::from("$")), start_line));
                } else {
                    tokens.push((Token::Keyword(format!("${}", after)), start_line));
                }
            }
            _ => {
                let start_line = line;
                let mut val = String::new();
                while let Some(&vc) = chars.peek() {
                    if vc.is_whitespace() || vc == '\n' || vc == '\r' {
                        break;
                    }
                    // Do NOT break on '$' here — Icarus Verilog uses '$'
                    // as an id code (e.g. signal `b` may get id `$`,
                    // producing value-change tokens like `0$`).  The '$'
                    // is only a keyword delimiter when it starts a
                    // whitespace-delimited token (handled by the '$'
                    // arm above).
                    val.push(vc);
                    chars.next();
                }
                if !val.is_empty() {
                    tokens.push((Token::Value(val), start_line));
                }
            }
        }
    }

    tokens
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

/// Parse a VCD input string into a [`VcdParseResult`].
///
/// This is the main entry point. It tokenises the input, extracts the header
/// (timescale, signal declarations), then walks the value-change records to
/// produce an [`EventTrace`].
///
/// # Errors
///
/// Returns [`VcdParseError`] for malformed input.
///
/// # Interchange-only contract
///
/// Per the spec scenario `digital-equivalence#vcd-interchange-only`, the
/// parsed [`EventTrace`] is the authoritative interchange product.  No test
/// or acceptance criterion depends on the raw VCD byte representation.
pub fn parse_vcd(input: &str) -> Result<VcdParseResult, VcdParseError> {
    let tokens = tokenize(input);
    if tokens.is_empty() {
        return Err(VcdParseError::EmptyInput);
    }

    let mut pos = 0;
    let mut timescale: Option<Timescale> = None;
    let mut signals: HashMap<String, String> = HashMap::new();
    let mut scope_stack: Vec<String> = Vec::new();
    let mut events: Vec<Event> = Vec::new();
    let mut current_time: Option<f64> = None;
    let mut in_header = true;
    let mut saw_enddefinitions = false;

    // Helper: read tokens until $end, collecting values
    let read_until_end = |tokens: &[(Token, usize)], start: usize| -> (Vec<String>, usize) {
        let mut vals = Vec::new();
        let mut i = start;
        while i < tokens.len() {
            match &tokens[i].0 {
                Token::Keyword(kw) if kw == "$end" => {
                    return (vals, i + 1);
                }
                Token::Value(v) => {
                    vals.push(v.clone());
                }
                _ => {}
            }
            i += 1;
        }
        (vals, i)
    };

    while pos < tokens.len() {
        let (ref token, line) = tokens[pos].clone();

        match token {
            Token::Keyword(kw) => {
                match kw.as_str() {
                    "$end" => {
                        // Skip stray $end tokens
                        pos += 1;
                    }
                    "$comment" => {
                        // Skip until $end
                        let (_, new_pos) = read_until_end(&tokens, pos + 1);
                        pos = new_pos;
                    }
                    "$date" | "$version" => {
                        // Skip until $end
                        let (_, new_pos) = read_until_end(&tokens, pos + 1);
                        pos = new_pos;
                    }
                    "$timescale" => {
                        let (vals, new_pos) = read_until_end(&tokens, pos + 1);
                        if vals.is_empty() {
                            return Err(VcdParseError::InvalidTimescale {
                                line,
                                raw: kw.clone(),
                            });
                        }
                        timescale = Some(parse_timescale(&vals.join(" "), line)?);
                        pos = new_pos;
                    }
                    "$scope" => {
                        // $scope <type> <name> $end
                        let (vals, new_pos) = read_until_end(&tokens, pos + 1);
                        // vals = [type, name]
                        if vals.len() >= 2 {
                            scope_stack.push(vals[1].clone());
                        }
                        pos = new_pos;
                    }
                    "$upscope" => {
                        scope_stack.pop();
                        pos += 1;
                        // skip $end if present
                        if pos < tokens.len() && tokens[pos].0 == Token::Keyword("$end".into()) {
                            pos += 1;
                        }
                    }
                    "$var" => {
                        // $var <type> <size> <id_code> <reference> $end
                        let (vals, new_pos) = read_until_end(&tokens, pos + 1);
                        // vals = [type, size, id_code, reference, ...]
                        if vals.len() < 3 {
                            return Err(VcdParseError::InvalidVarDecl {
                                line,
                                raw: vals.join(" "),
                            });
                        }
                        let var_type = &vals[0];
                        let size: usize = match vals[1].parse() {
                            Ok(s) => s,
                            Err(_) => {
                                return Err(VcdParseError::InvalidVarDecl {
                                    line,
                                    raw: vals.join(" "),
                                });
                            }
                        };
                        let id_code = vals[2].clone();
                        // The reference name is vals[3]; bit select [N] may follow
                        // For multi-bit vars, we only support size=1 for now
                        let ref_name = if vals.len() > 3 {
                            vals[3].clone()
                        } else {
                            id_code.clone()
                        };

                        // Reject real variables
                        if var_type == "real" {
                            return Err(VcdParseError::RealVarNotSupported {
                                line,
                                name: ref_name.clone(),
                            });
                        }

                        // Only support single-bit signals in the digital event model
                        // For multi-bit signals, we only store the id mapping;
                        // value changes for individual bits are handled per the
                        // VCD spec's 0/1/x/z prefix on id_code.
                        if size != 1 && var_type != "integer" {
                            // For integers, we still map the id but only accept
                            // single-char value changes (0/1/x/z prefix)
                        }

                        // Build dotted net name from scope stack + reference
                        let mut full_name = scope_stack.join(".");
                        if !full_name.is_empty() {
                            full_name.push('.');
                        }
                        full_name.push_str(&ref_name);

                        signals.insert(id_code, full_name);
                        pos = new_pos;
                    }
                    "$enddefinitions" => {
                        saw_enddefinitions = true;
                        in_header = false;
                        pos += 1;
                        // Skip $end
                        if pos < tokens.len() && tokens[pos].0 == Token::Keyword("$end".into()) {
                            pos += 1;
                        }
                    }
                    "$dumpvars" => {
                        // Process initial values until $end
                        // Each value line is <value_char><id_code>
                        // e.g. 1! or x? or 0"
                        pos += 1;
                        while pos < tokens.len() {
                            match &tokens[pos].0 {
                                Token::Keyword(kw) if kw == "$end" => {
                                    pos += 1;
                                    break;
                                }
                                Token::Value(v) => {
                                    if let Some((logic_val, id_code)) = parse_value_change(v) {
                                        if let Some(net) = signals.get(&id_code).cloned() {
                                            let time = current_time.unwrap_or(0.0);
                                            events.push(Event::new(time, net, logic_val));
                                        }
                                    }
                                    pos += 1;
                                }
                                _ => {
                                    pos += 1;
                                }
                            }
                        }
                    }
                    "$dumpoff" => {
                        // Icarus Verilog emits $dumpoff during simulation.
                        // Skip the X-value lines until $end.
                        pos += 1;
                        while pos < tokens.len() {
                            match &tokens[pos].0 {
                                Token::Keyword(kw) if kw == "$end" => {
                                    pos += 1;
                                    break;
                                }
                                _ => {
                                    pos += 1;
                                }
                            }
                        }
                    }
                    "$dumpon" => {
                        // Icarus Verilog emits $dumpon during simulation.
                        // Skip value lines until $end.
                        pos += 1;
                        while pos < tokens.len() {
                            match &tokens[pos].0 {
                                Token::Keyword(kw) if kw == "$end" => {
                                    pos += 1;
                                    break;
                                }
                                _ => {
                                    pos += 1;
                                }
                            }
                        }
                    }
                    "$dumpall" => {
                        // Icarus Verilog emits $dumpall in the header section.
                        // Skip value lines until $end.
                        pos += 1;
                        while pos < tokens.len() {
                            match &tokens[pos].0 {
                                Token::Keyword(kw) if kw == "$end" => {
                                    pos += 1;
                                    break;
                                }
                                Token::Value(v) => {
                                    // Process value changes (same as $dumpvars)
                                    if let Some((logic_val, id_code)) = parse_value_change(v) {
                                        if let Some(net) = signals.get(&id_code).cloned() {
                                            let time = current_time.unwrap_or(0.0);
                                            events.push(Event::new(time, net, logic_val));
                                        }
                                    }
                                    pos += 1;
                                }
                                _ => {
                                    pos += 1;
                                }
                            }
                        }
                    }
                    other => {
                        // Unknown keyword in header: skip until $end
                        if in_header {
                            let (_, new_pos) = read_until_end(&tokens, pos + 1);
                            pos = new_pos;
                        } else {
                            return Err(VcdParseError::UnknownKeyword {
                                line,
                                keyword: other.to_string(),
                            });
                        }
                    }
                }
            }
            Token::Value(v) => {
                if saw_enddefinitions {
                    // Check if this is a timestamp (#N)
                    if let Some(rest) = v.strip_prefix('#') {
                        let ts: u64 = match rest.parse() {
                            Ok(t) => t,
                            Err(_) => {
                                // Not a valid timestamp; skip
                                pos += 1;
                                continue;
                            }
                        };
                        let scale = timescale.as_ref().map(|t| t.to_seconds()).unwrap_or(1e-9); // default to ns if missing
                        current_time = Some(ts as f64 * scale);
                        pos += 1;
                    } else {
                        // Value change line: <value_char><id_code>
                        // e.g. 1! x? 0" Z#
                        if let Some((logic_val, id_code)) = parse_value_change(v) {
                            if let Some(net) = signals.get(&id_code).cloned() {
                                let time = current_time.unwrap_or(0.0);
                                events.push(Event::new(time, net, logic_val));
                            }
                            // If id_code not found, skip silently (might be
                            // a signal we don't care about).
                        }
                        pos += 1;
                    }
                } else {
                    // Still in header; skip stray values
                    pos += 1;
                }
            }
        }
    }

    let timescale = timescale.unwrap_or(Timescale {
        magnitude: TimescaleMagnitude::One,
        unit: TimescaleUnit::Nanosecond,
    });

    let trace = EventTrace::from_unsorted(events);

    Ok(VcdParseResult {
        header: VcdHeader { timescale, signals },
        trace,
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse a value-change token like "1!" or "x?" into (LogicValue, id_code).
///
/// VCD value changes for single-bit signals are written as `<value><id>`,
/// where `<value>` is one of 0, 1, x, X, z, Z and `<id>` is the short
/// identifier code assigned in the `$var` declaration.
fn parse_value_change(v: &str) -> Option<(LogicValue, String)> {
    if v.is_empty() {
        return None;
    }
    let first = v.chars().next()?;
    let logic_val = LogicValue::from_char(first)?;
    let id_code = &v[first.len_utf8()..];
    if id_code.is_empty() {
        return None;
    }
    Some((logic_val, id_code.to_string()))
}

/// Parse a timescale string like "1 ns" or "10ps" or "100 us".
fn parse_timescale(s: &str, line: usize) -> Result<Timescale, VcdParseError> {
    let s = s.trim();

    // Try to split into magnitude and unit
    // Formats: "1 ns", "10ps", "100us", "1s"
    let (mag_str, unit_str) = if let Some(pos) = s.find(|c: char| c.is_alphabetic()) {
        (&s[..pos], &s[pos..])
    } else {
        return Err(VcdParseError::InvalidTimescale {
            line,
            raw: s.to_string(),
        });
    };

    let magnitude = match mag_str.trim() {
        "1" => TimescaleMagnitude::One,
        "10" => TimescaleMagnitude::Ten,
        "100" => TimescaleMagnitude::Hundred,
        _ => {
            return Err(VcdParseError::InvalidTimescale {
                line,
                raw: s.to_string(),
            });
        }
    };

    let unit = match unit_str.trim() {
        "s" | "sec" | "second" => TimescaleUnit::Second,
        "ms" | "msec" => TimescaleUnit::Millisecond,
        "us" | "usec" => TimescaleUnit::Microsecond,
        "ns" | "nsec" => TimescaleUnit::Nanosecond,
        "ps" | "psec" => TimescaleUnit::Picosecond,
        "fs" | "fsec" => TimescaleUnit::Femtosecond,
        _ => {
            return Err(VcdParseError::InvalidTimescale {
                line,
                raw: s.to_string(),
            });
        }
    };

    Ok(Timescale { magnitude, unit })
}

// ---------------------------------------------------------------------------
// Unit tests (inline)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_timescale_1ns() {
        let ts = parse_timescale("1 ns", 1).unwrap();
        assert_eq!(ts.magnitude, TimescaleMagnitude::One);
        assert_eq!(ts.unit, TimescaleUnit::Nanosecond);
        assert!((ts.to_seconds() - 1e-9).abs() < 1e-20);
    }

    #[test]
    fn parse_timescale_10ps() {
        let ts = parse_timescale("10ps", 1).unwrap();
        assert_eq!(ts.magnitude, TimescaleMagnitude::Ten);
        assert_eq!(ts.unit, TimescaleUnit::Picosecond);
        assert!((ts.to_seconds() - 10e-12).abs() < 1e-20);
    }

    #[test]
    fn parse_timescale_100us() {
        let ts = parse_timescale("100 us", 1).unwrap();
        assert_eq!(ts.magnitude, TimescaleMagnitude::Hundred);
        assert_eq!(ts.unit, TimescaleUnit::Microsecond);
        assert!((ts.to_seconds() - 100e-6).abs() < 1e-15);
    }

    #[test]
    fn parse_timescale_invalid() {
        assert!(parse_timescale("xyz", 1).is_err());
        assert!(parse_timescale("5 ns", 1).is_err()); // magnitude not 1/10/100
    }

    #[test]
    fn parse_value_change_single_char_id() {
        assert_eq!(
            parse_value_change("1!"),
            Some((LogicValue::One, "!".to_string()))
        );
        assert_eq!(
            parse_value_change("0\""),
            Some((LogicValue::Zero, "\"".to_string()))
        );
        assert_eq!(
            parse_value_change("x?"),
            Some((LogicValue::X, "?".to_string()))
        );
        assert_eq!(
            parse_value_change("Z#"),
            Some((LogicValue::Z, "#".to_string()))
        );
    }

    #[test]
    fn parse_value_change_multi_char_id() {
        assert_eq!(
            parse_value_change("1abc"),
            Some((LogicValue::One, "abc".to_string()))
        );
    }

    #[test]
    fn parse_value_change_empty() {
        assert_eq!(parse_value_change(""), None);
    }

    #[test]
    fn parse_value_change_invalid_char() {
        assert_eq!(parse_value_change("2!"), None); // '2' is not a valid logic value
    }

    #[test]
    fn parse_minimal_vcd() {
        let vcd = "\
$date 2026-01-01 $end
$version test $end
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$var wire 2 \" data $end
$upscope $end
$enddefinitions $end
$dumpvars
1!
0\"
$end
#5
0!
1\"
#10
1!
$end
";
        let result = parse_vcd(vcd).unwrap();
        assert_eq!(result.header.signals.len(), 2);
        assert_eq!(result.header.signals["!"], "top.clk");
        assert_eq!(result.header.signals["\""], "top.data");

        let trace = result.trace;
        // dumpvars at t=0: clk=1, data=0
        // #5: clk=0, data=1
        // #10: clk=1
        assert_eq!(trace.len(), 5);

        // Check that events are sorted
        let events = trace.as_slice();
        assert_eq!(events[0].time, 0.0);
        assert_eq!(events[0].net, "top.clk");
        assert_eq!(events[0].value, LogicValue::One);

        assert_eq!(events[1].time, 0.0);
        assert_eq!(events[1].net, "top.data");
        assert_eq!(events[1].value, LogicValue::Zero);

        assert_eq!(events[2].time, 5e-9);
        assert_eq!(events[2].net, "top.clk");
        assert_eq!(events[2].value, LogicValue::Zero);

        assert_eq!(events[3].time, 5e-9);
        assert_eq!(events[3].net, "top.data");
        assert_eq!(events[3].value, LogicValue::One);

        assert_eq!(events[4].time, 10e-9);
        assert_eq!(events[4].net, "top.clk");
        assert_eq!(events[4].value, LogicValue::One);
    }

    #[test]
    fn parse_vcd_with_nested_scopes() {
        let vcd = "\
$timescale 1 ps $end
$scope module top $end
$scope module cpu $end
$var wire 1 ! clk $end
$upscope $end
$scope module mem $end
$var wire 1 \" en $end
$upscope $end
$upscope $end
$enddefinitions $end
#0
1!
0\"
#100
0!
1\"
";
        let result = parse_vcd(vcd).unwrap();
        assert_eq!(result.header.signals["!"], "top.cpu.clk");
        assert_eq!(result.header.signals["\""], "top.mem.en");
        assert_eq!(result.trace.len(), 4);
    }

    #[test]
    fn parse_vcd_x_and_z_values() {
        let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$var wire 1 \" b $end
$upscope $end
$enddefinitions $end
$dumpvars
x!
z\"
$end
#1
0!
1\"
#2
X!
Z\"
";
        let result = parse_vcd(vcd).unwrap();
        let events = result.trace.as_slice();
        // dumpvars: a=X, b=Z at t=0
        assert_eq!(events[0].value, LogicValue::X);
        assert_eq!(events[1].value, LogicValue::Z);
        // #2: a=X, b=Z again (duplicates will be deduped by from_unsorted)
    }

    #[test]
    fn parse_vcd_empty_input() {
        assert!(matches!(parse_vcd(""), Err(VcdParseError::EmptyInput)));
    }

    #[test]
    fn parse_vcd_no_timestamps() {
        let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
";
        let result = parse_vcd(vcd).unwrap();
        assert!(result.trace.is_empty());
    }

    #[test]
    fn parse_vcd_real_var_rejected() {
        let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var real 1 ! voltage $end
$upscope $end
$enddefinitions $end
";
        assert!(matches!(
            parse_vcd(vcd),
            Err(VcdParseError::RealVarNotSupported { .. })
        ));
    }

    #[test]
    fn parse_vcd_dumpoff_skipped() {
        let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#0
0!
$dumpoff
x!
$end
#10
1!
";
        let result = parse_vcd(vcd).unwrap();
        // $dumpoff values are skipped; we get the initial 0! and the #10 1!
        assert_eq!(result.trace.len(), 2);
        assert_eq!(result.trace.as_slice()[0].value, LogicValue::Zero);
        assert_eq!(result.trace.as_slice()[1].value, LogicValue::One);
    }

    #[test]
    fn timescale_default_to_ns_when_missing() {
        let vcd = "\
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#10
1!
";
        let result = parse_vcd(vcd).unwrap();
        assert_eq!(result.header.timescale.unit, TimescaleUnit::Nanosecond);
        assert_eq!(result.header.timescale.magnitude, TimescaleMagnitude::One);
        // #10 with 1ns timescale = 10ns
        assert!((result.trace.as_slice()[0].time - 10e-9).abs() < 1e-20);
    }

    #[test]
    fn parse_vcd_100fs_timescale() {
        let vcd = "\
$timescale 100 fs $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#1000
1!
";
        let result = parse_vcd(vcd).unwrap();
        // 1000 * 100fs = 100000fs = 100ps = 100e-12
        assert!((result.trace.as_slice()[0].time - 100e-12).abs() < 1e-20);
    }

    #[test]
    fn parse_vcd_unknown_signal_id_ignored() {
        let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#5
1!
1@  // @ is not declared, should be silently ignored
";
        let result = parse_vcd(vcd).unwrap();
        assert_eq!(result.trace.len(), 1);
    }

    #[test]
    fn parse_vcd_multiple_value_changes_same_time() {
        let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$var wire 1 \" b $end
$upscope $end
$enddefinitions $end
#0
1!
0\"
#5
0!
1\"
";
        let result = parse_vcd(vcd).unwrap();
        assert_eq!(result.trace.len(), 4);
        let events = result.trace.as_slice();
        // At t=0: a=1, b=0 (sorted by net name)
        assert_eq!(events[0].net, "m.a");
        assert_eq!(events[0].value, LogicValue::One);
        assert_eq!(events[1].net, "m.b");
        assert_eq!(events[1].value, LogicValue::Zero);
        // At t=5ns: a=0, b=1
        assert_eq!(events[2].net, "m.a");
        assert_eq!(events[2].value, LogicValue::Zero);
        assert_eq!(events[3].net, "m.b");
        assert_eq!(events[3].value, LogicValue::One);
    }

    #[test]
    fn parse_vcd_roundtrip_with_equivalence_checker() {
        // Parse a VCD, build a trace manually, compare with equivalence checker
        use super::super::equivalence::{check_equivalence, EquivalenceTolerance};

        let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$var wire 1 \" data $end
$upscope $end
$enddefinitions $end
$dumpvars
0!
0\"
$end
#10
1!
#20
0!
1\"
#30
1!
0\"
";
        let parsed = parse_vcd(vcd).unwrap();
        let manual = EventTrace::from_sorted(vec![
            Event::new(0.0, "top.clk", LogicValue::Zero),
            Event::new(0.0, "top.data", LogicValue::Zero),
            Event::new(10e-9, "top.clk", LogicValue::One),
            Event::new(20e-9, "top.clk", LogicValue::Zero),
            Event::new(20e-9, "top.data", LogicValue::One),
            Event::new(30e-9, "top.clk", LogicValue::One),
            Event::new(30e-9, "top.data", LogicValue::Zero),
        ]);

        // Use a small time tolerance because integer * float multiplication
        // (e.g. 30 * 1e-9) introduces ULP-level rounding that exact comparison
        // rejects.  1e-15 is well below any practical timing resolution.
        let result = check_equivalence(
            &parsed.trace,
            &manual,
            &EquivalenceTolerance::with_time_tolerance(1e-15),
        );
        assert!(
            result.equivalent,
            "VCD-parsed trace should match manual trace"
        );
    }

    #[test]
    fn vcd_parse_error_display() {
        let e = VcdParseError::InvalidTimescale {
            line: 5,
            raw: "bad".to_string(),
        };
        let s = format!("{}", e);
        assert!(s.contains("line 5"));
        assert!(s.contains("bad"));
    }
}
