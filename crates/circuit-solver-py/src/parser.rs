//! SPICE netlist parser — tasks.md item #60.
//!
//! Implements the SPICE-netlist-file-parsing capability of the
//! `python-frontend` bounded context. The public entry point is the
//! free function [`parse_file`], called by the Python-facing
//! `parse_netlist_py` `PyO3` binding in `crate::lib`. The parser
//! walks the netlist *text* and replays the recognized
//! declarations against a fresh
//! [`netlist_graph::CircuitBuilder`], then calls `build()` to
//! return the immutable [`netlist_graph::CircuitGraph`] the
//! `PyO3` layer wraps as `PyCircuitGraph`.
//!
//! ## Why text → builder replay (not a separate AST)
//!
//! The Gherkin scenario
//! `python-frontend#spice-netlist-file-parsing` asserts that
//!
//! > the `CircuitGraph` is identical to one built incrementally with
//! > the same topology.
//!
//! The cleanest way to honor that assertion is to make the parser
//! use the *same* builder API a Python caller would: any divergence
//! between "parsed graph" and "incrementally-built graph" can only
//! arise from the parser sending different `add_*` calls, not from
//! a parallel construction path. The per-test equivalence harness
//! in this module's `tests` submodule asserts this directly.
//!
//! ## SPICE subset (v1)
//!
//! The supported subset is intentionally minimal — enough to light
//! up the spec scenario and to feed the existing
//! `netlist_graph::CircuitBuilder` API. Per ADR-0010 the surface is
//! unstable, so the grammar can grow as later tasks demand it.
//!
//! - **Title line.** The first non-empty line of a SPICE deck is
//!   conventionally a title and is skipped.
//! - **Comments.** Lines whose first non-blank character is `*`, `;`,
//!   or `$` are ignored. Inline `;` comments on element lines are
//!   stripped before tokenization.
//! - **Continuation lines.** A line starting with `+` (after leading
//!   whitespace) is appended to the previous logical line.
//! - **Element cards.** First-letter dispatch:
//!   - `R`, `C`, `L`, `V`, `I` — two-terminal linear element with a
//!     trailing numeric value. SPICE suffixes (`k`, `meg`, `m`, `u`,
//!     `n`, `p`, `f`, `g`, `t`, `mil`) are honored.
//!   - `D` — two-terminal semiconductor referencing a model name.
//!   - `Q` — three-terminal BJT referencing a model name (collector,
//!     base, emitter).
//!   - `M` — four-terminal MOSFET referencing a model name (drain,
//!     gate, source, bulk).
//!   - `X` — subcircuit instance. Token layout is
//!     `Xname node1 node2 ... subname` (subname is the trailing
//!     token; everything between is the port-binding list).
//! - **Directives.**
//!   - `.MODEL <name> <type> [...]` — registers the model name with
//!     the builder. The model `type` and parameter list are
//!     accepted but currently discarded (the device-modeling crate
//!     resolves model names; type details are not surfaced via
//!     `circuit_solver.parse_netlist` at v1).
//!   - `.SUBCKT <name> <p1> [p2 ...]` … `.ENDS` — defines a
//!     subcircuit. Body lines inside the block are parsed as if
//!     they were top-level element cards but accumulated into the
//!     definition's body (`add_subcircuit` on the Python side).
//!     Nested `.SUBCKT` blocks are not supported in v1.
//!   - `.END` — terminator; remaining lines are ignored.
//!   - Any other `.` directive (`.OPTIONS`, `.DC`, `.AC`, `.TRAN`,
//!     `.PRINT`, `.NODESET`, …) is silently skipped at v1 — analysis
//!     cards are owned by `AnalysisRequest` (tasks.md #54+) not the
//!     graph builder.
//!
//! ## What this parser does *not* do (yet)
//!
//! - SPICE expressions, `.PARAM` substitution, `.INCLUDE` /
//!   `.LIB` resolution, parametrized models. The minimal subset
//!   above is the v1 contract.
//!
//! ## Unrecognised device letters → `NetlistParseError` (tasks.md #61)
//!
//! When `parse_element_card` encounters a card whose leading
//! character is not one of the supported device letters
//! (`R`, `C`, `L`, `V`, `I`, `D`, `Q`, `M`, `X`), it raises a
//! [`crate::errors::NetlistParseError`] carrying the unrecognised
//! token. The 1-indexed source line number is then prepended by
//! `annotate_with_line` as the error bubbles out. This lights up
//! the `python-frontend#error-on-malformed-netlist` Gherkin scenario.
//! Other parse-time failures (arity, malformed numeric value,
//! unterminated `.SUBCKT`, …) continue to surface as `PyValueError`
//! pending the broader Python-error-mapping pass in tasks.md #58.

use std::fs;
use std::path::Path;

use netlist_graph::{CircuitBuilder, CircuitGraph, ElementDecl, ElementKind, SubcircuitDefinition};
use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::PyErr;

use crate::errors::{netlist_parse_error_unrecognised_device, to_py_err};

/// Parse a SPICE netlist file at `path` and return the resulting
/// [`CircuitGraph`].
///
/// This is the Rust core of the Python `circuit_solver.parse_netlist`
/// entry point. The function reads the file, parses the SPICE subset
/// documented at the module level, and replays the declarations
/// against a fresh [`CircuitBuilder`]. The returned `CircuitGraph` is
/// the same value `CircuitBuilder::build()` would produce had the
/// caller issued the equivalent `add_*` calls from Python — the
/// `python-frontend#spice-netlist-file-parsing` Gherkin scenario's
/// "identical to one built incrementally" assertion follows directly.
///
/// # Errors
///
/// - [`PyIOError`] if the file cannot be read.
/// - [`crate::errors::NetlistParseError`] if a card's leading character
///   is not one of the recognised SPICE device letters
///   (`R`, `C`, `L`, `V`, `I`, `D`, `Q`, `M`, `X`). The message
///   identifies the 1-indexed line number and the unrecognised token,
///   per the `python-frontend#error-on-malformed-netlist` Gherkin
///   scenario (tasks.md #61).
/// - [`PyValueError`] if a line cannot be tokenized into a recognized
///   card shape (wrong arity, missing model name, malformed numeric
///   value, unterminated `.SUBCKT`, etc.). The broader Python-error
///   mapping pass that may migrate these onto a structured taxonomy
///   is tasks.md #58.
/// - The `CircuitBuilderError` Python exception (via
///   [`to_py_err`]) if the underlying [`CircuitBuilder`] rejects a
///   replayed `add_*` call (duplicate name, terminal arity mismatch,
///   unknown subcircuit, port arity mismatch, expansion cycle).
pub fn parse_file(path: &Path) -> Result<CircuitGraph, PyErr> {
    let text = fs::read_to_string(path).map_err(|e| {
        PyIOError::new_err(format!(
            "circuit_solver.parse_netlist: cannot read {}: {e}",
            path.display()
        ))
    })?;
    parse_text(&text)
}

/// Parse SPICE netlist `text` and return the resulting
/// [`CircuitGraph`]. Exposed separately from [`parse_file`] so unit
/// tests can drive the parser with in-memory strings without
/// touching the filesystem.
///
/// # Title-line convention
///
/// Per SPICE convention the very first physical line of a deck is a
/// title and is unconditionally skipped — even if it happens to look
/// like a valid element card. Tools that emit "headless" decks
/// (just card lines, no title) must prepend a leading title line
/// (e.g. `* generated by ...`) or accept that their first card will
/// be dropped. This matches every mainstream SPICE dialect
/// (ngspice, hspice, ltspice, xyce).
///
/// # Errors
///
/// See [`parse_file`].
///
/// # Panics
///
/// Panics only on internal invariant violation: the post-trim
/// `stripped` slice is guaranteed non-empty by the preceding
/// `is_empty` short-circuit, so `chars().next()` cannot be `None`.
/// The `.expect(...)` documents that invariant.
pub fn parse_text(text: &str) -> Result<CircuitGraph, PyErr> {
    let logical = stitch_continuation_lines(text);
    let mut builder = CircuitBuilder::new();
    let mut subckt: Option<SubcktAccumulator> = None;
    let mut saw_first_physical_line = false;
    let mut reached_end = false;

    for line in &logical {
        if reached_end {
            break;
        }
        // SPICE convention: skip the first physical line of the
        // deck unconditionally (the deck title). This applies even
        // if the title text happens to start with a SPICE-letter
        // — that's what every conforming SPICE flavour does.
        if !saw_first_physical_line {
            saw_first_physical_line = true;
            continue;
        }
        let stripped_owned = strip_inline_comment(&line.text);
        let stripped = stripped_owned.trim();
        if stripped.is_empty() {
            continue;
        }
        let first = stripped
            .chars()
            .next()
            .expect("non-empty after trim implies at least one char");
        if first == '*' || first == ';' || first == '$' {
            continue;
        }

        if let Some(rest_of_dot) = stripped.strip_prefix('.') {
            handle_directive(rest_of_dot, line.number, &mut builder, &mut subckt)
                .map_err(annotate_with_line(line.number))?;
            if directive_is_end(rest_of_dot) {
                reached_end = true;
            }
            continue;
        }

        // Inside a .SUBCKT block: accumulate element declarations
        // until we see .ENDS.
        if let Some(acc) = subckt.as_mut() {
            let decl = parse_element_card(stripped, line.number)
                .map_err(annotate_with_line(line.number))?;
            acc.body.push(decl);
            continue;
        }

        replay_top_level_card(stripped, line.number, &mut builder)
            .map_err(annotate_with_line(line.number))?;
    }

    if let Some(acc) = subckt {
        return Err(PyValueError::new_err(format!(
            "circuit_solver.parse_netlist: unterminated .SUBCKT '{}' (started at line {}); \
             expected matching .ENDS",
            acc.name, acc.opened_at_line,
        )));
    }

    builder.build().map_err(|e| to_py_err(&e))
}

// ---------------------------------------------------------------------
// Logical-line stitching (continuation lines).
// ---------------------------------------------------------------------

/// A logical (post-continuation) SPICE line, carrying the source line
/// number of its first physical line so error messages can point at
/// the right spot.
struct LogicalLine {
    /// 1-indexed source line number where the logical line begins.
    number: usize,
    /// Concatenated text of the logical line. Continuation `+`
    /// prefixes have been stripped and replaced with a single space.
    text: String,
}

fn stitch_continuation_lines(text: &str) -> Vec<LogicalLine> {
    let mut out: Vec<LogicalLine> = Vec::new();
    for (idx, raw) in text.lines().enumerate() {
        let lineno = idx + 1;
        let trimmed_start = raw.trim_start();
        if let Some(cont) = trimmed_start.strip_prefix('+') {
            // Continuation. Attach to the previous logical line if
            // any; otherwise treat as a standalone line minus the
            // leading `+` (an orphan continuation is harmless and
            // pruning it lets the rest of the parser proceed).
            if let Some(last) = out.last_mut() {
                last.text.push(' ');
                last.text.push_str(cont.trim());
                continue;
            }
            out.push(LogicalLine {
                number: lineno,
                text: cont.trim().to_string(),
            });
            continue;
        }
        out.push(LogicalLine {
            number: lineno,
            text: raw.to_string(),
        });
    }
    out
}

fn strip_inline_comment(line: &str) -> String {
    // SPICE inline comment markers: `;` and `$`. `*` is whole-line
    // only by convention and is handled in the main loop.
    let cut = line.find([';', '$']).unwrap_or(line.len());
    line[..cut].to_string()
}

// ---------------------------------------------------------------------
// Directive dispatch.
// ---------------------------------------------------------------------

struct SubcktAccumulator {
    name: String,
    ports: Vec<String>,
    body: Vec<ElementDecl>,
    opened_at_line: usize,
}

fn directive_is_end(rest: &str) -> bool {
    let head = rest
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_uppercase();
    head == "END"
}

fn handle_directive(
    rest: &str,
    lineno: usize,
    builder: &mut CircuitBuilder,
    subckt: &mut Option<SubcktAccumulator>,
) -> Result<(), PyErr> {
    let mut toks = rest.split_whitespace();
    let head = toks
        .next()
        .ok_or_else(|| PyValueError::new_err("empty directive after '.'"))?
        .to_ascii_uppercase();

    match head.as_str() {
        "MODEL" => {
            if subckt.is_some() {
                return Err(PyValueError::new_err(
                    ".MODEL inside .SUBCKT is not supported in v1",
                ));
            }
            let name = toks.next().ok_or_else(|| {
                PyValueError::new_err(".MODEL requires a model name as the next token")
            })?;
            // The `<type>` and any trailing parameter tokens are
            // consumed-and-discarded — the v1 surface only needs the
            // name so `add_model` can register it.
            for _ in toks {}
            builder.add_model(circuit_solver_types::ModelName::new(name));
            Ok(())
        }
        "SUBCKT" => {
            if subckt.is_some() {
                return Err(PyValueError::new_err(
                    "nested .SUBCKT definitions are not supported in v1",
                ));
            }
            let name = toks
                .next()
                .ok_or_else(|| PyValueError::new_err(".SUBCKT requires a definition name"))?
                .to_string();
            let ports: Vec<String> = toks.map(str::to_string).collect();
            if ports.is_empty() {
                return Err(PyValueError::new_err(format!(
                    ".SUBCKT '{name}' requires at least one port"
                )));
            }
            *subckt = Some(SubcktAccumulator {
                name,
                ports,
                body: Vec::new(),
                opened_at_line: lineno,
            });
            Ok(())
        }
        "ENDS" => {
            let acc = subckt
                .take()
                .ok_or_else(|| PyValueError::new_err(".ENDS without a matching open .SUBCKT"))?;
            let definition = SubcircuitDefinition::new(acc.name.into(), acc.ports, acc.body);
            builder
                .add_subcircuit(definition)
                .map_err(|e| to_py_err(&e))?;
            Ok(())
        }
        // `.END` and every analysis / option / control card we
        // silently accept and skip at v1 (`.OPTIONS`, `.DC`, `.AC`,
        // `.TRAN`, `.NOISE`, `.PRINT`, `.PLOT`, `.SAVE`, `.IC`,
        // `.NODESET`, `.GLOBAL`, `.INCLUDE`, `.LIB`, `.TEMP`, …)
        // share an identical "noop in the v1 builder" body, so we
        // group them under the wildcard. `.END` is recognised
        // separately by [`directive_is_end`] which signals the
        // caller to stop reading further lines.
        _ => Ok(()),
    }
}

// ---------------------------------------------------------------------
// Element-card replay (top-level and subcircuit-body shared shape).
// ---------------------------------------------------------------------

fn replay_top_level_card(
    line: &str,
    lineno: usize,
    builder: &mut CircuitBuilder,
) -> Result<(), PyErr> {
    let head = line
        .chars()
        .next()
        .ok_or_else(|| PyValueError::new_err("empty element line"))?;
    if head.eq_ignore_ascii_case(&'X') {
        return replay_subcircuit_instance(line, lineno, builder);
    }
    let decl = parse_element_card(line, lineno)?;
    builder
        .add_element(decl.name, decl.kind, decl.terminals, decl.model)
        .map_err(|e| to_py_err(&e))?;
    Ok(())
}

fn replay_subcircuit_instance(
    line: &str,
    _lineno: usize,
    builder: &mut CircuitBuilder,
) -> Result<(), PyErr> {
    let toks: Vec<&str> = line.split_whitespace().collect();
    if toks.len() < 3 {
        return Err(PyValueError::new_err(
            "X-card requires at least: Xname node subname",
        ));
    }
    let name = toks[0];
    let subname = toks[toks.len() - 1];
    let port_bindings: Vec<String> = toks[1..toks.len() - 1]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    builder
        .add_subcircuit_instance(name, subname, port_bindings)
        .map_err(|e| to_py_err(&e))?;
    Ok(())
}

/// Parse one SPICE element card into an [`ElementDecl`]. Shared
/// between top-level cards (replayed via `builder.add_element`) and
/// subcircuit-body cards (accumulated into a
/// [`SubcircuitDefinition`]).
///
/// Returns the decl in the same shape the Rust `CircuitBuilder` API
/// expects so the caller can hand it off without further conversion.
fn parse_element_card(line: &str, _lineno: usize) -> Result<ElementDecl, PyErr> {
    let toks: Vec<&str> = line.split_whitespace().collect();
    if toks.is_empty() {
        return Err(PyValueError::new_err("empty element line"));
    }
    let raw_name = toks[0];
    let first = raw_name
        .chars()
        .next()
        .ok_or_else(|| PyValueError::new_err("element name token is empty"))?
        .to_ascii_uppercase();
    let name = raw_name.to_string();

    match first {
        'R' | 'C' | 'L' | 'V' | 'I' => {
            // Two-terminal linear: name node1 node2 value
            if toks.len() < 4 {
                return Err(PyValueError::new_err(format!(
                    "'{first}' card '{name}' requires: name n1 n2 value (got {} tokens)",
                    toks.len()
                )));
            }
            let n1 = toks[1].to_string();
            let n2 = toks[2].to_string();
            let value = parse_spice_number(toks[3]).map_err(|e| {
                PyValueError::new_err(format!(
                    "'{first}' card '{name}': cannot parse value {:?}: {e}",
                    toks[3]
                ))
            })?;
            let kind = linear_kind(first, value);
            Ok(ElementDecl {
                name: name.into(),
                kind,
                terminals: vec![n1, n2],
                model: None,
            })
        }
        'D' => {
            // Two-terminal semiconductor: name anode cathode model
            if toks.len() < 4 {
                return Err(PyValueError::new_err(format!(
                    "'D' card '{name}' requires: name anode cathode model (got {} tokens)",
                    toks.len()
                )));
            }
            let n1 = toks[1].to_string();
            let n2 = toks[2].to_string();
            let model = toks[3].to_string();
            Ok(ElementDecl {
                name: name.into(),
                kind: ElementKind::Semiconductor,
                terminals: vec![n1, n2],
                model: Some(circuit_solver_types::ModelName::new(model)),
            })
        }
        'Q' => {
            // Three-terminal BJT: name c b e model
            if toks.len() < 5 {
                return Err(PyValueError::new_err(format!(
                    "'Q' card '{name}' requires: name c b e model (got {} tokens)",
                    toks.len()
                )));
            }
            let terminals = vec![toks[1].into(), toks[2].into(), toks[3].into()];
            let model = toks[4].to_string();
            Ok(ElementDecl {
                name: name.into(),
                kind: ElementKind::Semiconductor,
                terminals,
                model: Some(circuit_solver_types::ModelName::new(model)),
            })
        }
        'M' => {
            // Four-terminal MOSFET: name d g s b model
            if toks.len() < 6 {
                return Err(PyValueError::new_err(format!(
                    "'M' card '{name}' requires: name d g s b model (got {} tokens)",
                    toks.len()
                )));
            }
            let terminals = vec![
                toks[1].into(),
                toks[2].into(),
                toks[3].into(),
                toks[4].into(),
            ];
            let model = toks[5].to_string();
            Ok(ElementDecl {
                name: name.into(),
                kind: ElementKind::Semiconductor,
                terminals,
                model: Some(circuit_solver_types::ModelName::new(model)),
            })
        }
        other => Err(netlist_parse_error_unrecognised_device(&name, other)),
    }
}

fn linear_kind(letter: char, value: f64) -> ElementKind {
    match letter {
        'R' => ElementKind::Resistor {
            resistance_ohms: value,
        },
        'C' => ElementKind::Capacitor {
            capacitance_farads: value,
        },
        'L' => ElementKind::Inductor {
            inductance_henries: value,
        },
        'V' => ElementKind::VoltageSource {
            voltage_volts: value,
        },
        'I' => ElementKind::CurrentSource {
            current_amperes: value,
        },
        _ => unreachable!("linear_kind only called with R/C/L/V/I"),
    }
}

/// Parse a SPICE-style numeric value: scientific notation with
/// optional engineering suffix.
///
/// Recognised suffixes (case-insensitive):
///
/// | suffix | multiplier  |
/// |--------|-------------|
/// | `T`    | 1e12        |
/// | `G`    | 1e9         |
/// | `MEG`  | 1e6         |
/// | `K`    | 1e3         |
/// | `MIL`  | 25.4e-6     |
/// | `M`    | 1e-3        |
/// | `U`    | 1e-6        |
/// | `N`    | 1e-9        |
/// | `P`    | 1e-12       |
/// | `F`    | 1e-15       |
///
/// Trailing alphabetic noise after the suffix is silently allowed
/// (SPICE convention: `1.0kohm` is `1.0 * 1e3 = 1000.0`).
fn parse_spice_number(s: &str) -> Result<f64, String> {
    // Find the boundary between the numeric prefix (digits, sign,
    // decimal point, exponent letter `e`/`E` followed by optional
    // sign and digits) and the suffix. We pull the longest such
    // prefix manually because Rust's `f64::from_str` rejects
    // anything past the number.
    let bytes = s.as_bytes();
    let mut end = 0usize;
    let mut seen_exp = false;
    while end < bytes.len() {
        let c = bytes[end] as char;
        let is_sign = (c == '+' || c == '-')
            && (end == 0
                || (seen_exp && bytes[end - 1] as char == 'e' || bytes[end - 1] as char == 'E'));
        if c.is_ascii_digit() || c == '.' || is_sign {
            end += 1;
            continue;
        }
        if (c == 'e' || c == 'E') && !seen_exp {
            seen_exp = true;
            end += 1;
            continue;
        }
        break;
    }
    if end == 0 {
        return Err("no numeric prefix".to_string());
    }
    let num_part: f64 = s[..end]
        .parse()
        .map_err(|e: std::num::ParseFloatError| e.to_string())?;
    let suffix = s[end..].to_ascii_uppercase();
    let mult = suffix_multiplier(&suffix);
    Ok(num_part * mult)
}

fn suffix_multiplier(suffix: &str) -> f64 {
    // Order matters: check longer suffixes first so `MEG` and `MIL`
    // win over a bare leading `M`.
    if suffix.starts_with("MEG") {
        return 1e6;
    }
    if suffix.starts_with("MIL") {
        return 25.4e-6;
    }
    match suffix.chars().next() {
        Some('T') => 1e12,
        Some('G') => 1e9,
        Some('K') => 1e3,
        Some('M') => 1e-3,
        Some('U') => 1e-6,
        Some('N') => 1e-9,
        Some('P') => 1e-12,
        Some('F') => 1e-15,
        _ => 1.0,
    }
}

// ---------------------------------------------------------------------
// Error-annotation helper.
// ---------------------------------------------------------------------

fn annotate_with_line(lineno: usize) -> impl Fn(PyErr) -> PyErr {
    move |err| {
        // Carry the original exception type through (a
        // `CircuitBuilderError` stays a `CircuitBuilderError`,
        // `PyValueError` stays a `PyValueError`, `NetlistParseError`
        // stays a `NetlistParseError`) but prepend the line number to
        // the message so callers know where the failure originated.
        // `from_type` is the PyO3 idiom for type-preserving rewrap.
        pyo3::Python::attach(|py| {
            let msg = err.value(py).to_string();
            let py_type = err.get_type(py);
            let new_msg = format!("circuit_solver.parse_netlist: line {lineno}: {msg}");
            PyErr::from_type(py_type, new_msg)
        })
    }
}

// =====================================================================
// Unit tests (Python-free in source, but the `pyo3::Python::attach`
// calls in some negative-path tests still need libpython at link
// time). Gated off when the `extension-module` feature is active so
// `cargo test --workspace` (default features) doesn't try to link
// libpython into the unit-test binary — the recipe for this crate's
// test target is `cargo test -p circuit-solver-py --no-default-features`.
// =====================================================================

#[cfg(all(test, not(feature = "extension-module")))]
mod tests {
    use super::*;
    use netlist_graph::CircuitBuilder;

    /// Build the equivalent graph incrementally, the way a Python
    /// caller would after task #53. The Gherkin scenario's third
    /// assertion ("identical to one built incrementally with the
    /// same topology") is verified by comparing the parsed graph to
    /// what this helper produces.
    fn build_incrementally_resistive_divider() -> CircuitGraph {
        let mut b = CircuitBuilder::new();
        b.add_element(
            "R1",
            ElementKind::Resistor {
                resistance_ohms: 1_000.0,
            },
            ["n1", "n2"]
                .iter()
                .map(|s| (*s).to_string())
                .collect::<Vec<_>>(),
            None,
        )
        .unwrap();
        b.add_element(
            "R2",
            ElementKind::Resistor {
                resistance_ohms: 2_000.0,
            },
            ["n2", "0"]
                .iter()
                .map(|s| (*s).to_string())
                .collect::<Vec<_>>(),
            None,
        )
        .unwrap();
        b.add_element(
            "V1",
            ElementKind::VoltageSource { voltage_volts: 5.0 },
            ["n1", "0"]
                .iter()
                .map(|s| (*s).to_string())
                .collect::<Vec<_>>(),
            None,
        )
        .unwrap();
        b.build().expect("incremental build must succeed")
    }

    fn extract_signature(g: &CircuitGraph) -> (usize, usize, usize, Vec<String>, Vec<String>) {
        let mut element_names: Vec<String> = g
            .elements()
            .iter()
            .map(|e| e.name().as_str().to_string())
            .collect();
        let mut node_names: Vec<String> = g.nodes().iter().map(|n| n.name().to_string()).collect();
        element_names.sort();
        node_names.sort();
        (
            g.element_count(),
            g.node_count(),
            g.model_count(),
            element_names,
            node_names,
        )
    }

    #[test]
    fn parse_minimal_resistive_divider_matches_incremental_build() {
        let deck = "\
Resistive divider test deck
R1 n1 n2 1k
R2 n2 0 2k
V1 n1 0 5
.end
";
        let parsed = parse_text(deck).expect("parse must succeed");
        let incremental = build_incrementally_resistive_divider();
        assert_eq!(
            extract_signature(&parsed),
            extract_signature(&incremental),
            "parsed and incrementally-built graphs must be identical"
        );
    }

    #[test]
    fn parse_handles_comments_and_blank_lines() {
        let deck = "\
* This deck exercises comments and blanks
* a leading title line and inline-comment stripping.

R1 n1 n2 1k ; trailing inline comment
$ shell-style comment line
R2 n2 0 2k
V1 n1 0 5
.END
";
        let g = parse_text(deck).expect("parse must succeed");
        assert_eq!(g.element_count(), 3);
        // 3 distinct nets: n1, n2, 0.
        assert_eq!(g.node_count(), 3);
    }

    #[test]
    fn parse_handles_continuation_lines() {
        let deck = "\
Continuation-line test
R1 n1
+ n2
+ 1k
V1 n1 0 5
";
        let g = parse_text(deck).expect("parse must succeed");
        assert_eq!(g.element_count(), 2);
    }

    #[test]
    fn parse_handles_spice_suffixes() {
        let deck = "\
Suffix test
R1 a b 1.5k
R2 b c 2.2meg
R3 c d 470
C1 d e 1u
C2 e f 10n
L1 f g 1mil
V1 a 0 5
";
        let g = parse_text(deck).expect("parse must succeed");
        assert_eq!(g.element_count(), 7);
    }

    #[test]
    fn parse_recognises_dot_model() {
        let deck = "\
Diode test
.MODEL DMOD D IS=1e-14 N=1.5
D1 a b DMOD
V1 a 0 5
";
        let g = parse_text(deck).expect("parse must succeed");
        assert_eq!(g.element_count(), 2);
        assert_eq!(g.model_count(), 1);
    }

    #[test]
    fn parse_recognises_subckt_definition_and_instance() {
        let deck = "\
Subcircuit test
.SUBCKT INV in out vdd vss
R1 in mid 1k
R2 mid out 1k
.ENDS
X1 a b vdd 0 INV
V1 vdd 0 5
";
        let g = parse_text(deck).expect("parse must succeed");
        // X1 expands to R1+R2; plus V1 → 3 elements.
        assert_eq!(g.element_count(), 3);
        assert!(g.is_fully_expanded());
    }

    #[test]
    fn parse_title_line_is_unconditionally_skipped() {
        // Documents the SPICE-convention title-line rule: the very
        // first physical line of a deck is always treated as a
        // title, even when it happens to start with what looks
        // like a SPICE-letter. Decks without an intended title
        // must prepend a `*`-comment line.
        let headless = "\
R1 n1 n2 1k
R2 n2 0 2k
V1 n1 0 5
";
        // Without a leading title, the first R1 card is dropped.
        let parsed = parse_text(headless).expect("parse must succeed");
        assert_eq!(
            parsed.element_count(),
            2,
            "title-line rule: first physical line is consumed as title"
        );

        // With a comment title, all three cards survive.
        let titled = "* Comment-style title\nR1 n1 n2 1k\nR2 n2 0 2k\nV1 n1 0 5\n";
        let parsed = parse_text(titled).expect("parse must succeed");
        assert_eq!(parsed.element_count(), 3);
    }

    #[test]
    fn parse_unrecognised_device_letter_returns_netlist_parse_error() {
        // tasks.md #61 behaviour: an unrecognised SPICE device letter
        // surfaces as a dedicated `NetlistParseError` whose message
        // carries the line number and the offending token. Verified
        // by exception type and message-content substrings; the
        // Gherkin witness for `python-frontend#error-on-malformed-netlist`
        // exercises the same contract end-to-end via the Python
        // binding in `tests/error_on_malformed_netlist.rs`.
        use crate::errors::NetlistParseError;
        let deck = "\
Bad deck
Z1 a b 1k
";
        let err = parse_text(deck).expect_err("unknown device letter must fail");
        pyo3::Python::attach(|py| {
            assert!(
                err.is_instance_of::<NetlistParseError>(py),
                "must be NetlistParseError; got: {}",
                err.value(py)
            );
            let msg = err.value(py).to_string();
            assert!(
                msg.contains('Z'),
                "error message must identify the offending letter; got: {msg}"
            );
            assert!(
                msg.contains("Z1"),
                "error message must identify the unrecognised token ('Z1'); got: {msg}"
            );
            // The deck above places the bad card on physical line 2
            // (line 1 is the SPICE title line, which is skipped).
            assert!(
                msg.contains("line 2"),
                "error message must identify the line number ('line 2'); got: {msg}"
            );
        });
    }

    #[test]
    fn parse_unterminated_subckt_returns_value_error() {
        let deck = "\
Bad subckt
.SUBCKT INV in out
R1 in out 1k
";
        let err = parse_text(deck).expect_err("unterminated .SUBCKT must fail");
        let msg = pyo3::Python::attach(|py| err.value(py).to_string());
        assert!(
            msg.contains("unterminated .SUBCKT"),
            "error must mention unterminated subckt; got: {msg}"
        );
    }

    #[test]
    fn parse_spice_number_basic_and_suffixes() {
        // Sanity-check the SPICE-number parser in isolation.
        assert!((parse_spice_number("1k").unwrap() - 1000.0).abs() < 1e-9);
        assert!((parse_spice_number("2.2meg").unwrap() - 2_200_000.0).abs() < 1e-6);
        assert!((parse_spice_number("1u").unwrap() - 1e-6).abs() < 1e-15);
        // Arbitrary literal — *not* an approximation of π; clippy's
        // `approx_constant` lint sees `3.14` and flags it because it
        // *resembles* π, so we use a value that doesn't shadow a
        // physical constant.
        assert!((parse_spice_number("2.5").unwrap() - 2.5).abs() < 1e-9);
        assert!((parse_spice_number("1.5e-3").unwrap() - 1.5e-3).abs() < 1e-15);
        assert!((parse_spice_number("1.0kohm").unwrap() - 1000.0).abs() < 1e-9);
    }

    #[test]
    fn parse_then_compare_to_python_style_incremental_resistor_divider() {
        // Explicit Gherkin-scenario witness: build a 2-resistor +
        // source deck both ways and assert byte-for-byte signature
        // identity. This is the same property tested by
        // parse_minimal_resistive_divider_matches_incremental_build
        // but with the equivalence asserted on element count, node
        // count, model count, *and* both name lists — the strongest
        // statement of "identical" the public CircuitGraph surface
        // permits.
        let deck = "\
Resistive divider
R1 n1 n2 1k
R2 n2 0 2k
V1 n1 0 5
";
        let parsed = parse_text(deck).expect("parse must succeed");
        let incremental = build_incrementally_resistive_divider();
        let (pe, pn, pm, pel, pnl) = extract_signature(&parsed);
        let (ie, in_, im, iel, inl) = extract_signature(&incremental);
        assert_eq!(pe, ie, "element_count must match");
        assert_eq!(pn, in_, "node_count must match");
        assert_eq!(pm, im, "model_count must match");
        assert_eq!(pel, iel, "element name set must match");
        assert_eq!(pnl, inl, "node name set must match");
    }

    #[test]
    fn parse_full_signature_round_trip() {
        let deck = "\
Round trip
R1 n1 n2 1k
R2 n2 0 2k
V1 n1 0 5
";
        let parsed = parse_text(deck).unwrap();
        let again = parse_text(deck).unwrap();
        assert_eq!(extract_signature(&parsed), extract_signature(&again));
    }
}
