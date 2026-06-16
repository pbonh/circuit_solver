//! Subset Verilog-AMS (`.vams`) parser.
//!
//! Parses a behavioural Verilog-AMS module into an AST.  No evaluation is
//! performed; the returned tree is suitable for a subsequent elaboration pass.
//!
//! ## Supported subset
//!
//! - Module header with a port list.
//! - Port declarations: `input`, `output`, `inout` with optional discipline
//!   (`electrical`, `logic`, or bare wire/net type).
//! - One `analog` block containing contribution statements:
//!   `V(a,b) <+ expr;` and `I(a,b) <+ expr;`
//! - Expressions: numeric literals, identifiers, binary `+`/`-`/`*`/`/`,
//!   unary `-`, and the AMS operators `idt(x, ic)`, `ddt(x)`,
//!   `transition(x, td, tr, tf)`, `slew(x, sr, sf)`.
//!
//! ## Example
//!
//! ```
//! use circuit_solver_delta::vams_parser::parse_module;
//!
//! let src = r#"
//! module resistor(p, n);
//!   inout electrical p, n;
//!   parameter real R = 1000.0;
//!   analog begin
//!     V(p,n) <+ R * I(p,n);
//!   end
//! endmodule
//! "#;
//!
//! let m = parse_module(src).unwrap();
//! assert_eq!(m.name, "resistor");
//! assert_eq!(m.ports.len(), 2);
//! ```

// ──────────────────────────────────────────────────────────────────────────────
// AST definitions
// ──────────────────────────────────────────────────────────────────────────────

/// Electrical discipline of a port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Discipline {
    /// `electrical` — continuous voltage/current domain.
    Electrical,
    /// `logic` — discrete logic domain.
    Logic,
    /// No explicit discipline keyword (wire / net type).
    None,
}

/// Direction of a port declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortDir {
    Input,
    Output,
    Inout,
}

/// A single port declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct Port {
    pub name: String,
    pub dir: PortDir,
    pub discipline: Discipline,
}

/// An AMS nature probe / branch access — `V(a,b)` or `I(a,b)`.
#[derive(Debug, Clone, PartialEq)]
pub struct BranchAccess {
    /// `'V'` for voltage, `'I'` for current.
    pub nature: char,
    pub pos: String,
    pub neg: Option<String>,
}

/// Expression node in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Numeric literal (integer or real).
    Num(f64),
    /// Identifier reference (e.g. a parameter name or port name).
    Ident(String),
    /// `idt(x, ic)` — time-domain integration.
    Idt {
        x: Box<Expr>,
        ic: Box<Expr>,
    },
    /// `ddt(x)` — time-domain differentiation.
    Ddt {
        x: Box<Expr>,
    },
    /// `transition(x, td, tr, tf)` — digital-to-analogue transition filter.
    Transition {
        x: Box<Expr>,
        td: Box<Expr>,
        tr: Box<Expr>,
        tf: Box<Expr>,
    },
    /// `slew(x, sr, sf)` — slew-rate limiter.
    Slew {
        x: Box<Expr>,
        sr: Box<Expr>,
        sf: Box<Expr>,
    },
    /// Branch access probe inside an expression, e.g. `I(p,n)`.
    BranchProbe(BranchAccess),
    /// Binary operation.
    BinOp {
        op: BinOpKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    /// Unary negation.
    Neg(Box<Expr>),
}

/// Binary operators supported in expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

/// A single contribution statement inside an `analog` block.
///
/// ```text
/// V(p,n) <+ expr;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ContribStmt {
    /// Left-hand side branch access (`V` or `I`).
    pub lhs: BranchAccess,
    /// Right-hand side expression.
    pub rhs: Expr,
}

/// The parsed body of the `analog` block.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogBlock {
    pub stmts: Vec<ContribStmt>,
}

/// Top-level module AST node.
#[derive(Debug, Clone, PartialEq)]
pub struct VamsModule {
    pub name: String,
    pub ports: Vec<Port>,
    pub analog: Option<AnalogBlock>,
}

// ──────────────────────────────────────────────────────────────────────────────
// Tokeniser (hand-rolled, no external deps)
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    // keywords
    Module,
    Endmodule,
    Input,
    Output,
    Inout,
    Electrical,
    Logic,
    Analog,
    Begin,
    End,
    Parameter,
    Real,
    // AMS operators
    Idt,
    Ddt,
    Transition,
    Slew,
    // probe kinds
    V,
    I,
    // symbols
    LParen,
    RParen,
    Semi,
    Comma,
    ContribArrow, // <+
    Plus,
    Minus,
    Star,
    Slash,
    // atoms
    Ident(String),
    Num(f64),
    // end
    Eof,
}

struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Lexer {
            src: src.as_bytes(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let ch = self.src.get(self.pos).copied();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // skip whitespace
            while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
                self.advance();
            }
            // single-line comment //
            if self.src.get(self.pos..self.pos + 2) == Some(b"//") {
                while !matches!(self.peek(), Some(b'\n') | None) {
                    self.advance();
                }
                continue;
            }
            // block comment /* ... */
            if self.src.get(self.pos..self.pos + 2) == Some(b"/*") {
                self.pos += 2;
                loop {
                    if self.src.get(self.pos..self.pos + 2) == Some(b"*/") {
                        self.pos += 2;
                        break;
                    }
                    if self.advance().is_none() {
                        break;
                    }
                }
                continue;
            }
            break;
        }
    }

    fn read_ident_or_kw(&mut self) -> Token {
        let start = self.pos;
        while matches!(self.peek(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'$')) {
            self.advance();
        }
        let s = std::str::from_utf8(&self.src[start..self.pos]).unwrap();
        match s {
            "module" => Token::Module,
            "endmodule" => Token::Endmodule,
            "input" => Token::Input,
            "output" => Token::Output,
            "inout" => Token::Inout,
            "electrical" => Token::Electrical,
            "logic" => Token::Logic,
            "analog" => Token::Analog,
            "begin" => Token::Begin,
            "end" => Token::End,
            "parameter" => Token::Parameter,
            "real" => Token::Real,
            "idt" => Token::Idt,
            "ddt" => Token::Ddt,
            "transition" => Token::Transition,
            "slew" => Token::Slew,
            "V" => Token::V,
            "I" => Token::I,
            other => Token::Ident(other.to_string()),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        // integer part
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.advance();
        }
        // optional decimal
        if self.peek() == Some(b'.') {
            self.advance();
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.advance();
            }
        }
        // optional exponent
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.advance();
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.advance();
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.advance();
            }
        }
        let s = std::str::from_utf8(&self.src[start..self.pos]).unwrap();
        Token::Num(s.parse().unwrap_or(0.0))
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        match self.peek() {
            None => Token::Eof,
            Some(b'(') => { self.advance(); Token::LParen }
            Some(b')') => { self.advance(); Token::RParen }
            Some(b';') => { self.advance(); Token::Semi }
            Some(b',') => { self.advance(); Token::Comma }
            Some(b'+') => { self.advance(); Token::Plus }
            Some(b'-') => { self.advance(); Token::Minus }
            Some(b'*') => { self.advance(); Token::Star }
            Some(b'/') => { self.advance(); Token::Slash }
            Some(b'<') => {
                self.advance();
                if self.peek() == Some(b'+') {
                    self.advance();
                    Token::ContribArrow
                } else {
                    // Not handled in this subset; return a placeholder ident
                    Token::Ident("<".to_string())
                }
            }
            Some(ch) if ch.is_ascii_alphabetic() || ch == b'_' || ch == b'$' => {
                self.read_ident_or_kw()
            }
            Some(ch) if ch.is_ascii_digit() => self.read_number(),
            Some(ch) => {
                let c = ch as char;
                self.advance();
                Token::Ident(c.to_string())
            }
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Parser
// ──────────────────────────────────────────────────────────────────────────────

/// Parse error with a human-readable message.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(src: &str) -> Self {
        let mut lex = Lexer::new(src);
        let mut tokens = Vec::new();
        loop {
            let tok = lex.next_token();
            let is_eof = tok == Token::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.advance().clone() {
            Token::Ident(s) => Ok(s),
            other => Err(ParseError(format!("expected identifier, got {:?}", other))),
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParseError> {
        let tok = self.advance().clone();
        if std::mem::discriminant(&tok) == std::mem::discriminant(expected) {
            Ok(())
        } else {
            Err(ParseError(format!("expected {:?}, got {:?}", expected, tok)))
        }
    }

    // ── grammar productions ─────────────────────────────────────────────────

    /// Parse a top-level module.
    fn parse_module(&mut self) -> Result<VamsModule, ParseError> {
        self.expect(&Token::Module)?;
        let name = self.expect_ident()?;

        // optional port name list: ( p, n, ... )
        let mut port_names: Vec<String> = Vec::new();
        if *self.peek() == Token::LParen {
            self.advance(); // consume '('
            while *self.peek() != Token::RParen {
                if *self.peek() == Token::Comma {
                    self.advance();
                    continue;
                }
                port_names.push(self.expect_ident()?);
            }
            self.expect(&Token::RParen)?;
        }
        self.expect(&Token::Semi)?;

        // module body: port declarations, parameter decls, analog block
        let mut ports: Vec<Port> = Vec::new();
        let mut analog: Option<AnalogBlock> = None;

        loop {
            match self.peek().clone() {
                Token::Endmodule => {
                    self.advance();
                    break;
                }
                Token::Eof => break,
                Token::Input | Token::Output | Token::Inout => {
                    let mut decls = self.parse_port_decl()?;
                    ports.append(&mut decls);
                }
                Token::Analog => {
                    analog = Some(self.parse_analog_block()?);
                }
                // skip `parameter real R = 1000.0;` and other decls we don't model
                _ => {
                    self.skip_to_semi();
                }
            }
        }

        // If port names were given in the header but no explicit direction
        // declarations appeared, fabricate minimal inout entries so the port
        // list is populated (common in abbreviated test modules).
        if ports.is_empty() {
            for pn in &port_names {
                ports.push(Port {
                    name: pn.clone(),
                    dir: PortDir::Inout,
                    discipline: Discipline::None,
                });
            }
        }

        Ok(VamsModule { name, ports, analog })
    }

    /// Skip tokens until (and including) the next `;`.
    fn skip_to_semi(&mut self) {
        loop {
            match self.advance() {
                Token::Semi | Token::Eof => break,
                _ => {}
            }
        }
    }

    /// Parse a port declaration line:
    ///
    /// ```text
    /// input  [electrical] name1, name2, ... ;
    /// output [electrical] name1 ;
    /// inout  [electrical] name1 ;
    /// ```
    fn parse_port_decl(&mut self) -> Result<Vec<Port>, ParseError> {
        let dir = match self.advance().clone() {
            Token::Input => PortDir::Input,
            Token::Output => PortDir::Output,
            Token::Inout => PortDir::Inout,
            other => return Err(ParseError(format!("expected port direction, got {:?}", other))),
        };
        // optional discipline
        let discipline = match self.peek().clone() {
            Token::Electrical => { self.advance(); Discipline::Electrical }
            Token::Logic => { self.advance(); Discipline::Logic }
            _ => Discipline::None,
        };
        // one or more identifiers separated by commas
        let mut ports = Vec::new();
        loop {
            let name = self.expect_ident()?;
            ports.push(Port { name, dir: dir.clone(), discipline: discipline.clone() });
            match self.peek() {
                Token::Comma => { self.advance(); }
                Token::Semi => { self.advance(); break; }
                _ => break,
            }
        }
        Ok(ports)
    }

    /// Parse the `analog begin … end` block (or single-statement `analog stmt;`).
    fn parse_analog_block(&mut self) -> Result<AnalogBlock, ParseError> {
        self.expect(&Token::Analog)?;
        let mut stmts = Vec::new();

        if *self.peek() == Token::Begin {
            self.advance(); // consume 'begin'
            while *self.peek() != Token::End && *self.peek() != Token::Eof {
                stmts.push(self.parse_contrib_stmt()?);
            }
            self.expect(&Token::End)?;
        } else {
            stmts.push(self.parse_contrib_stmt()?);
        }

        Ok(AnalogBlock { stmts })
    }

    /// Parse `V(a,b) <+ expr;` or `I(a,b) <+ expr;`.
    fn parse_contrib_stmt(&mut self) -> Result<ContribStmt, ParseError> {
        let lhs = self.parse_branch_access()?;
        self.expect(&Token::ContribArrow)?;
        let rhs = self.parse_expr()?;
        self.expect(&Token::Semi)?;
        Ok(ContribStmt { lhs, rhs })
    }

    /// Parse `V(a)` or `V(a,b)` or `I(a)` or `I(a,b)`.
    fn parse_branch_access(&mut self) -> Result<BranchAccess, ParseError> {
        let nature = match self.advance().clone() {
            Token::V => 'V',
            Token::I => 'I',
            other => return Err(ParseError(format!("expected V or I, got {:?}", other))),
        };
        self.expect(&Token::LParen)?;
        let pos = self.expect_ident()?;
        let neg = if *self.peek() == Token::Comma {
            self.advance();
            Some(self.expect_ident()?)
        } else {
            None
        };
        self.expect(&Token::RParen)?;
        Ok(BranchAccess { nature, pos, neg })
    }

    // ── expression parser (precedence climbing) ─────────────────────────────

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_mul_div()?;
        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let rhs = self.parse_mul_div()?;
                    lhs = Expr::BinOp { op: BinOpKind::Add, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                Token::Minus => {
                    self.advance();
                    let rhs = self.parse_mul_div()?;
                    lhs = Expr::BinOp { op: BinOpKind::Sub, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_unary()?;
        loop {
            match self.peek() {
                Token::Star => {
                    self.advance();
                    let rhs = self.parse_unary()?;
                    lhs = Expr::BinOp { op: BinOpKind::Mul, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                Token::Slash => {
                    self.advance();
                    let rhs = self.parse_unary()?;
                    lhs = Expr::BinOp { op: BinOpKind::Div, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if *self.peek() == Token::Minus {
            self.advance();
            let e = self.parse_primary()?;
            return Ok(Expr::Neg(Box::new(e)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().clone() {
            Token::Num(v) => { self.advance(); Ok(Expr::Num(v)) }
            Token::LParen => {
                self.advance();
                let e = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(e)
            }
            Token::V | Token::I => {
                let ba = self.parse_branch_access()?;
                Ok(Expr::BranchProbe(ba))
            }
            Token::Idt => {
                self.advance();
                self.expect(&Token::LParen)?;
                let x = self.parse_expr()?;
                self.expect(&Token::Comma)?;
                let ic = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(Expr::Idt { x: Box::new(x), ic: Box::new(ic) })
            }
            Token::Ddt => {
                self.advance();
                self.expect(&Token::LParen)?;
                let x = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(Expr::Ddt { x: Box::new(x) })
            }
            Token::Transition => {
                self.advance();
                self.expect(&Token::LParen)?;
                let x = self.parse_expr()?;
                self.expect(&Token::Comma)?;
                let td = self.parse_expr()?;
                self.expect(&Token::Comma)?;
                let tr = self.parse_expr()?;
                self.expect(&Token::Comma)?;
                let tf = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(Expr::Transition {
                    x: Box::new(x),
                    td: Box::new(td),
                    tr: Box::new(tr),
                    tf: Box::new(tf),
                })
            }
            Token::Slew => {
                self.advance();
                self.expect(&Token::LParen)?;
                let x = self.parse_expr()?;
                self.expect(&Token::Comma)?;
                let sr = self.parse_expr()?;
                self.expect(&Token::Comma)?;
                let sf = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(Expr::Slew {
                    x: Box::new(x),
                    sr: Box::new(sr),
                    sf: Box::new(sf),
                })
            }
            Token::Ident(name) => {
                self.advance();
                Ok(Expr::Ident(name))
            }
            other => Err(ParseError(format!("unexpected token in expression: {:?}", other))),
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Public entry point
// ──────────────────────────────────────────────────────────────────────────────

/// Parse a Verilog-AMS source string and return the module AST.
///
/// Returns `Err(ParseError)` on any syntax error.
pub fn parse_module(src: &str) -> Result<VamsModule, ParseError> {
    Parser::new(src).parse_module()
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Behavioural resistor: V(p,n) <+ R * I(p,n)
    const RESISTOR_VAMS: &str = r#"
        module resistor(p, n);
          inout electrical p, n;
          parameter real R = 1000.0;
          analog begin
            V(p,n) <+ R * I(p,n);
          end
        endmodule
    "#;

    #[test]
    fn parse_resistor_port_count() {
        let m = parse_module(RESISTOR_VAMS).unwrap();
        assert_eq!(m.name, "resistor");
        assert_eq!(m.ports.len(), 2, "expected 2 ports");
    }

    #[test]
    fn parse_resistor_port_directions_and_disciplines() {
        let m = parse_module(RESISTOR_VAMS).unwrap();
        for port in &m.ports {
            assert_eq!(port.dir, PortDir::Inout);
            assert_eq!(port.discipline, Discipline::Electrical);
        }
        let names: Vec<&str> = m.ports.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, vec!["p", "n"]);
    }

    #[test]
    fn parse_resistor_analog_block_present() {
        let m = parse_module(RESISTOR_VAMS).unwrap();
        assert!(m.analog.is_some(), "analog block should be present");
        let analog = m.analog.unwrap();
        assert_eq!(analog.stmts.len(), 1, "expected 1 contribution statement");
    }

    #[test]
    fn parse_resistor_contribution_lhs() {
        let m = parse_module(RESISTOR_VAMS).unwrap();
        let stmt = &m.analog.unwrap().stmts[0];
        assert_eq!(stmt.lhs.nature, 'V');
        assert_eq!(stmt.lhs.pos, "p");
        assert_eq!(stmt.lhs.neg.as_deref(), Some("n"));
    }

    #[test]
    fn parse_resistor_contribution_rhs_is_mul() {
        let m = parse_module(RESISTOR_VAMS).unwrap();
        let stmt = &m.analog.unwrap().stmts[0];
        // R * I(p,n)
        match &stmt.rhs {
            Expr::BinOp { op, lhs, rhs } => {
                assert_eq!(*op, BinOpKind::Mul);
                assert!(matches!(lhs.as_ref(), Expr::Ident(s) if s == "R"));
                match rhs.as_ref() {
                    Expr::BranchProbe(ba) => {
                        assert_eq!(ba.nature, 'I');
                        assert_eq!(ba.pos, "p");
                        assert_eq!(ba.neg.as_deref(), Some("n"));
                    }
                    other => panic!("expected BranchProbe, got {:?}", other),
                }
            }
            other => panic!("expected BinOp(Mul), got {:?}", other),
        }
    }

    #[test]
    fn parse_idt_operator() {
        let src = r#"
            module cap(p, n);
              inout electrical p, n;
              parameter real C = 1e-12;
              analog begin
                I(p,n) <+ idt(V(p,n), 0.0) * C;
              end
            endmodule
        "#;
        let m = parse_module(src).unwrap();
        let stmt = &m.analog.unwrap().stmts[0];
        match &stmt.rhs {
            Expr::BinOp { op, lhs, .. } => {
                assert_eq!(*op, BinOpKind::Mul);
                assert!(matches!(lhs.as_ref(), Expr::Idt { .. }));
            }
            other => panic!("expected BinOp with Idt lhs, got {:?}", other),
        }
    }

    #[test]
    fn parse_ddt_operator() {
        let src = r#"
            module ind(p, n);
              inout electrical p, n;
              parameter real L = 1e-9;
              analog begin
                V(p,n) <+ L * ddt(I(p,n));
              end
            endmodule
        "#;
        let m = parse_module(src).unwrap();
        let stmt = &m.analog.unwrap().stmts[0];
        match &stmt.rhs {
            Expr::BinOp { op, rhs, .. } => {
                assert_eq!(*op, BinOpKind::Mul);
                assert!(matches!(rhs.as_ref(), Expr::Ddt { .. }));
            }
            other => panic!("expected BinOp with Ddt rhs, got {:?}", other),
        }
    }

    #[test]
    fn parse_transition_operator() {
        let src = r#"
            module buf(out, in);
              input logic in;
              output electrical out;
              analog begin
                V(out) <+ transition(in, 1e-9, 2e-9, 2e-9);
              end
            endmodule
        "#;
        let m = parse_module(src).unwrap();
        let stmt = &m.analog.unwrap().stmts[0];
        assert!(matches!(stmt.rhs, Expr::Transition { .. }));
    }

    #[test]
    fn parse_slew_operator() {
        let src = r#"
            module slewer(out, in);
              input logic in;
              output electrical out;
              analog begin
                V(out) <+ slew(in, 1e6, 1e6);
              end
            endmodule
        "#;
        let m = parse_module(src).unwrap();
        let stmt = &m.analog.unwrap().stmts[0];
        assert!(matches!(stmt.rhs, Expr::Slew { .. }));
    }

    #[test]
    fn parse_input_output_directions() {
        let src = r#"
            module drv(out, in);
              input electrical in;
              output electrical out;
              analog begin
                V(out) <+ V(in);
              end
            endmodule
        "#;
        let m = parse_module(src).unwrap();
        assert_eq!(m.ports.len(), 2);
        let dirs: Vec<&PortDir> = m.ports.iter().map(|p| &p.dir).collect();
        assert!(dirs.contains(&&PortDir::Input));
        assert!(dirs.contains(&&PortDir::Output));
    }

    #[test]
    fn parse_error_on_invalid_module() {
        let src = "not_a_valid_module { }";
        assert!(parse_module(src).is_err());
    }
}
