//! SPICE netlist tokenizer.
//!
//! Parses a SPICE netlist string into a sequence of [`NetlistToken`]s.
//! Unknown or unsupported lines emit a [`ParseWarning`] and are skipped rather
//! than causing a hard error.

/// A parsed SPICE element or directive.
#[derive(Debug, Clone, PartialEq)]
pub enum NetlistToken {
    // ── Passive / source elements ────────────────────────────────────────────
    /// Resistor: `R<name> <n+> <n-> <value>`
    Resistor { name: String, n_pos: String, n_neg: String, value: String },
    /// Inductor: `L<name> <n+> <n-> <value>`
    Inductor { name: String, n_pos: String, n_neg: String, value: String },
    /// Capacitor: `C<name> <n+> <n-> <value>`
    Capacitor { name: String, n_pos: String, n_neg: String, value: String },
    /// Independent voltage source: `V<name> <n+> <n-> <value>`
    VoltageSource { name: String, n_pos: String, n_neg: String, value: String },
    /// Independent current source: `I<name> <n+> <n-> <value>`
    CurrentSource { name: String, n_pos: String, n_neg: String, value: String },

    // ── Controlled sources ───────────────────────────────────────────────────
    /// Voltage-Controlled Voltage Source (VCVS): `E<name> <n+> <n-> <nc+> <nc-> <gain>`
    Vcvs {
        name: String,
        n_pos: String,
        n_neg: String,
        nc_pos: String,
        nc_neg: String,
        gain: String,
    },
    /// Voltage-Controlled Current Source (VCCS): `G<name> <n+> <n-> <nc+> <nc-> <transconductance>`
    Vccs {
        name: String,
        n_pos: String,
        n_neg: String,
        nc_pos: String,
        nc_neg: String,
        transconductance: String,
    },
    /// Current-Controlled Voltage Source (CCVS): `H<name> <n+> <n-> <vname> <transresistance>`
    Ccvs {
        name: String,
        n_pos: String,
        n_neg: String,
        vname: String,
        transresistance: String,
    },
    /// Current-Controlled Current Source (CCCS): `F<name> <n+> <n-> <vname> <gain>`
    Cccs {
        name: String,
        n_pos: String,
        n_neg: String,
        vname: String,
        gain: String,
    },

    // ── Directives ───────────────────────────────────────────────────────────
    /// `.subckt <name> <node>*`
    Subckt { name: String, nodes: Vec<String> },
    /// `.ends [name]`
    Ends { name: Option<String> },
    /// `.tran <tstep> <tstop> [<tstart> [<tmax>]]`
    Tran { args: Vec<String> },
    /// `.dc <srcname> <start> <stop> <step>`
    Dc { args: Vec<String> },
    /// `.ac <variation> <n> <fstart> <fstop>`
    Ac { args: Vec<String> },
    /// `.op`
    Op,
}

/// A non-fatal warning produced when the tokenizer encounters a line it does
/// not recognise or cannot fully parse.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseWarning {
    /// 1-based line number in the original netlist.
    pub line: usize,
    /// Raw text of the offending line.
    pub text: String,
    /// Human-readable reason.
    pub reason: String,
}

/// Tokenize a SPICE netlist.
///
/// Returns all recognised tokens and any warnings for lines that were skipped.
/// The title line (first line) is silently ignored per SPICE convention.
pub fn tokenize(netlist: &str) -> (Vec<NetlistToken>, Vec<ParseWarning>) {
    let mut tokens = Vec::new();
    let mut warnings = Vec::new();

    // Join continuation lines ('+' as first non-whitespace char).
    let joined = join_continuation_lines(netlist);

    for (line_no, raw) in joined.iter().enumerate() {
        let line_no = line_no + 1; // 1-based

        // SPICE convention: first line is always the title and is ignored.
        if line_no == 1 {
            continue;
        }

        // Blank lines and comment lines (starting with '*') are ignored.
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        match parse_line(trimmed) {
            Ok(Some(tok)) => tokens.push(tok),
            Ok(None) => {} // intentionally skipped (title, blank, etc.)
            Err(reason) => warnings.push(ParseWarning {
                line: line_no,
                text: raw.clone(),
                reason,
            }),
        }
    }

    (tokens, warnings)
}

// ── Internal helpers ─────────────────────────────────────────────────────────

/// Join lines that begin with '+' (SPICE line-continuation marker) to the
/// previous line so that each entry in the returned vec is a complete
/// logical line.
fn join_continuation_lines(netlist: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in netlist.lines() {
        if line.trim_start().starts_with('+')
            && let Some(last) = result.last_mut()
        {
            last.push(' ');
            last.push_str(line.trim_start().trim_start_matches('+').trim());
            continue;
        }
        result.push(line.to_string());
    }
    result
}

/// Parse a single logical line.  Returns `Ok(None)` for lines that should be
/// silently skipped (title line handling is done in `tokenize` at the
/// call-site level).
fn parse_line(line: &str) -> Result<Option<NetlistToken>, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(None);
    }

    let first = parts[0];

    // Directive lines start with '.'
    if let Some(directive) = first.strip_prefix('.') {
        return parse_directive(directive, &parts[1..]);
    }

    // Element lines: first char is the element type letter
    let type_char = first.chars().next().unwrap().to_ascii_uppercase();
    let elem_name = &first[1..]; // everything after the type char

    match type_char {
        'R' => parse_two_terminal("R", elem_name, &parts[1..], |name, n_pos, n_neg, value| {
            Ok(Some(NetlistToken::Resistor { name, n_pos, n_neg, value }))
        }),
        'L' => parse_two_terminal("L", elem_name, &parts[1..], |name, n_pos, n_neg, value| {
            Ok(Some(NetlistToken::Inductor { name, n_pos, n_neg, value }))
        }),
        'C' => parse_two_terminal("C", elem_name, &parts[1..], |name, n_pos, n_neg, value| {
            Ok(Some(NetlistToken::Capacitor { name, n_pos, n_neg, value }))
        }),
        'V' => parse_two_terminal("V", elem_name, &parts[1..], |name, n_pos, n_neg, value| {
            Ok(Some(NetlistToken::VoltageSource { name, n_pos, n_neg, value }))
        }),
        'I' => parse_two_terminal("I", elem_name, &parts[1..], |name, n_pos, n_neg, value| {
            Ok(Some(NetlistToken::CurrentSource { name, n_pos, n_neg, value }))
        }),
        'E' => parse_four_terminal("E", elem_name, &parts[1..], |name, n_pos, n_neg, nc_pos, nc_neg, val| {
            Ok(Some(NetlistToken::Vcvs { name, n_pos, n_neg, nc_pos, nc_neg, gain: val }))
        }),
        'G' => parse_four_terminal("G", elem_name, &parts[1..], |name, n_pos, n_neg, nc_pos, nc_neg, val| {
            Ok(Some(NetlistToken::Vccs { name, n_pos, n_neg, nc_pos, nc_neg, transconductance: val }))
        }),
        'H' => parse_current_controlled("H", elem_name, &parts[1..], |name, n_pos, n_neg, vname, val| {
            Ok(Some(NetlistToken::Ccvs { name, n_pos, n_neg, vname, transresistance: val }))
        }),
        'F' => parse_current_controlled("F", elem_name, &parts[1..], |name, n_pos, n_neg, vname, val| {
            Ok(Some(NetlistToken::Cccs { name, n_pos, n_neg, vname, gain: val }))
        }),
        _ => Err(format!("unknown element type '{type_char}' in line: {line}")),
    }
}

fn parse_two_terminal<F>(
    _type_char: &str,
    elem_name: &str,
    rest: &[&str],
    build: F,
) -> Result<Option<NetlistToken>, String>
where
    F: FnOnce(String, String, String, String) -> Result<Option<NetlistToken>, String>,
{
    if rest.len() < 3 {
        return Err(format!(
            "expected at least 3 fields (n+, n-, value) but got {}",
            rest.len()
        ));
    }
    build(
        elem_name.to_string(),
        rest[0].to_string(),
        rest[1].to_string(),
        rest[2].to_string(),
    )
}

fn parse_four_terminal<F>(
    _type_char: &str,
    elem_name: &str,
    rest: &[&str],
    build: F,
) -> Result<Option<NetlistToken>, String>
where
    F: FnOnce(String, String, String, String, String, String) -> Result<Option<NetlistToken>, String>,
{
    if rest.len() < 5 {
        return Err(format!(
            "expected at least 5 fields (n+, n-, nc+, nc-, value) but got {}",
            rest.len()
        ));
    }
    build(
        elem_name.to_string(),
        rest[0].to_string(),
        rest[1].to_string(),
        rest[2].to_string(),
        rest[3].to_string(),
        rest[4].to_string(),
    )
}

fn parse_current_controlled<F>(
    _type_char: &str,
    elem_name: &str,
    rest: &[&str],
    build: F,
) -> Result<Option<NetlistToken>, String>
where
    F: FnOnce(String, String, String, String, String) -> Result<Option<NetlistToken>, String>,
{
    if rest.len() < 4 {
        return Err(format!(
            "expected at least 4 fields (n+, n-, vname, value) but got {}",
            rest.len()
        ));
    }
    build(
        elem_name.to_string(),
        rest[0].to_string(),
        rest[1].to_string(),
        rest[2].to_string(),
        rest[3].to_string(),
    )
}

fn parse_directive(directive: &str, args: &[&str]) -> Result<Option<NetlistToken>, String> {
    match directive.to_ascii_lowercase().as_str() {
        "subckt" => {
            if args.is_empty() {
                return Err(".subckt requires at least a name".to_string());
            }
            Ok(Some(NetlistToken::Subckt {
                name: args[0].to_string(),
                nodes: args[1..].iter().map(|s| s.to_string()).collect(),
            }))
        }
        "ends" => Ok(Some(NetlistToken::Ends {
            name: args.first().map(|s| s.to_string()),
        })),
        "tran" => Ok(Some(NetlistToken::Tran {
            args: args.iter().map(|s| s.to_string()).collect(),
        })),
        "dc" => Ok(Some(NetlistToken::Dc {
            args: args.iter().map(|s| s.to_string()).collect(),
        })),
        "ac" => Ok(Some(NetlistToken::Ac {
            args: args.iter().map(|s| s.to_string()).collect(),
        })),
        "op" => Ok(Some(NetlistToken::Op)),
        other => Err(format!("unknown directive '.{other}'")),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: assert no warnings were produced.
    fn no_warnings(warnings: &[ParseWarning]) {
        assert!(
            warnings.is_empty(),
            "unexpected warnings: {warnings:?}"
        );
    }

    // ── Simple RC circuit ────────────────────────────────────────────────────

    #[test]
    fn parse_simple_rc_circuit() {
        let netlist = "\
Simple RC low-pass filter
V1 in 0 DC 5
R1 in out 1k
C1 out 0 1u
.tran 1n 1m
.op
";
        let (tokens, warnings) = tokenize(netlist);
        no_warnings(&warnings);

        assert_eq!(tokens.len(), 5);
        assert_eq!(
            tokens[0],
            NetlistToken::VoltageSource {
                name: "1".into(),
                n_pos: "in".into(),
                n_neg: "0".into(),
                value: "DC".into(),
            }
        );
        assert_eq!(
            tokens[1],
            NetlistToken::Resistor {
                name: "1".into(),
                n_pos: "in".into(),
                n_neg: "out".into(),
                value: "1k".into(),
            }
        );
        assert_eq!(
            tokens[2],
            NetlistToken::Capacitor {
                name: "1".into(),
                n_pos: "out".into(),
                n_neg: "0".into(),
                value: "1u".into(),
            }
        );
        assert_eq!(
            tokens[3],
            NetlistToken::Tran { args: vec!["1n".into(), "1m".into()] }
        );
        assert_eq!(tokens[4], NetlistToken::Op);
    }

    // ── Individual element types ──────────────────────────────────────────────

    #[test]
    fn parse_resistor() {
        let (tokens, warnings) = tokenize("RC filter\nR1 a b 100");
        no_warnings(&warnings);
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0],
            NetlistToken::Resistor {
                name: "1".into(),
                n_pos: "a".into(),
                n_neg: "b".into(),
                value: "100".into(),
            }
        );
    }

    #[test]
    fn parse_inductor() {
        let (tokens, warnings) = tokenize("LC\nL1 a b 10mH");
        no_warnings(&warnings);
        assert_eq!(
            tokens[0],
            NetlistToken::Inductor {
                name: "1".into(),
                n_pos: "a".into(),
                n_neg: "b".into(),
                value: "10mH".into(),
            }
        );
    }

    #[test]
    fn parse_capacitor() {
        let (tokens, warnings) = tokenize("cap\nC2 n1 0 100n");
        no_warnings(&warnings);
        assert_eq!(
            tokens[0],
            NetlistToken::Capacitor {
                name: "2".into(),
                n_pos: "n1".into(),
                n_neg: "0".into(),
                value: "100n".into(),
            }
        );
    }

    #[test]
    fn parse_voltage_source() {
        let (tokens, warnings) = tokenize("vs\nVcc vdd 0 3.3");
        no_warnings(&warnings);
        assert_eq!(
            tokens[0],
            NetlistToken::VoltageSource {
                name: "cc".into(),
                n_pos: "vdd".into(),
                n_neg: "0".into(),
                value: "3.3".into(),
            }
        );
    }

    #[test]
    fn parse_current_source() {
        let (tokens, warnings) = tokenize("is\nI1 in 0 1m");
        no_warnings(&warnings);
        assert_eq!(
            tokens[0],
            NetlistToken::CurrentSource {
                name: "1".into(),
                n_pos: "in".into(),
                n_neg: "0".into(),
                value: "1m".into(),
            }
        );
    }

    // ── Controlled sources ────────────────────────────────────────────────────

    #[test]
    fn parse_vcvs() {
        let (tokens, warnings) = tokenize("vcvs\nE1 out 0 in 0 10");
        no_warnings(&warnings);
        assert_eq!(
            tokens[0],
            NetlistToken::Vcvs {
                name: "1".into(),
                n_pos: "out".into(),
                n_neg: "0".into(),
                nc_pos: "in".into(),
                nc_neg: "0".into(),
                gain: "10".into(),
            }
        );
    }

    #[test]
    fn parse_vccs() {
        let (tokens, warnings) = tokenize("vccs\nG1 out 0 in 0 0.01");
        no_warnings(&warnings);
        assert_eq!(
            tokens[0],
            NetlistToken::Vccs {
                name: "1".into(),
                n_pos: "out".into(),
                n_neg: "0".into(),
                nc_pos: "in".into(),
                nc_neg: "0".into(),
                transconductance: "0.01".into(),
            }
        );
    }

    #[test]
    fn parse_ccvs() {
        let (tokens, warnings) = tokenize("ccvs\nH1 out 0 Vmeas 100");
        no_warnings(&warnings);
        assert_eq!(
            tokens[0],
            NetlistToken::Ccvs {
                name: "1".into(),
                n_pos: "out".into(),
                n_neg: "0".into(),
                vname: "Vmeas".into(),
                transresistance: "100".into(),
            }
        );
    }

    #[test]
    fn parse_cccs() {
        let (tokens, warnings) = tokenize("cccs\nF1 out 0 Vctrl 5");
        no_warnings(&warnings);
        assert_eq!(
            tokens[0],
            NetlistToken::Cccs {
                name: "1".into(),
                n_pos: "out".into(),
                n_neg: "0".into(),
                vname: "Vctrl".into(),
                gain: "5".into(),
            }
        );
    }

    // ── Directives ────────────────────────────────────────────────────────────

    #[test]
    fn parse_subckt_and_ends() {
        let netlist = "sub\n.subckt lowpass in out\nR1 in out 1k\n.ends lowpass\n";
        let (tokens, warnings) = tokenize(netlist);
        no_warnings(&warnings);
        assert_eq!(tokens.len(), 3);
        assert_eq!(
            tokens[0],
            NetlistToken::Subckt {
                name: "lowpass".into(),
                nodes: vec!["in".into(), "out".into()],
            }
        );
        assert_eq!(
            tokens[2],
            NetlistToken::Ends { name: Some("lowpass".into()) }
        );
    }

    #[test]
    fn parse_dc_directive() {
        let (tokens, warnings) = tokenize("dc\n.dc V1 0 5 0.1");
        no_warnings(&warnings);
        assert_eq!(
            tokens[0],
            NetlistToken::Dc {
                args: vec!["V1".into(), "0".into(), "5".into(), "0.1".into()],
            }
        );
    }

    #[test]
    fn parse_ac_directive() {
        let (tokens, warnings) = tokenize("ac\n.ac DEC 10 1 1MEG");
        no_warnings(&warnings);
        assert_eq!(
            tokens[0],
            NetlistToken::Ac {
                args: vec!["DEC".into(), "10".into(), "1".into(), "1MEG".into()],
            }
        );
    }

    // ── Warning / error paths ─────────────────────────────────────────────────

    #[test]
    fn unknown_element_produces_warning_not_error() {
        let netlist = "test\nQ1 c b e 2N3904\nR1 a b 1k";
        let (tokens, warnings) = tokenize(netlist);
        assert_eq!(tokens.len(), 1, "only R1 should be parsed");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].line, 2);
        assert!(warnings[0].text.contains("Q1"));
    }

    #[test]
    fn unknown_directive_produces_warning() {
        let netlist = "test\n.model nmos NMOS\nR1 a b 1k";
        let (tokens, warnings) = tokenize(netlist);
        assert_eq!(tokens.len(), 1);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].reason.contains("model"));
    }

    #[test]
    fn comment_lines_are_ignored() {
        let netlist = "test\n* this is a comment\nR1 a b 1k";
        let (tokens, warnings) = tokenize(netlist);
        no_warnings(&warnings);
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn continuation_lines_are_joined() {
        // The '+' continuation joins to the previous line
        let netlist = "test\n.tran 1n\n+ 1m";
        let (tokens, warnings) = tokenize(netlist);
        no_warnings(&warnings);
        assert_eq!(
            tokens[0],
            NetlistToken::Tran { args: vec!["1n".into(), "1m".into()] }
        );
    }
}
