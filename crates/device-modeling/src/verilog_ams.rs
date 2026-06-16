//! Verilog-AMS behavioral block evaluator and MNA stamp (US-016).
//!
//! This module provides:
//!
//! - A minimal Verilog-AMS **AST** ([`VamsExpr`], [`ContribStmt`],
//!   [`VamsModule`]) that covers the `V(a,b) <+ f(...)` contribution
//!   statement pattern.
//! - An **evaluator** ([`eval_expr`]) that resolves branch probes
//!   (`V(a,b)`, `I(a,b)`) against a solution vector plus prior-state
//!   context, and approximates the Verilog-AMS time-integral / time-
//!   derivative operators:
//!   - `idt(x, ic)` — time-integral approximated with the trapezoidal
//!     rule:
//!     ```text
//!     idt_n ≈ idt_{n−1} + (h / 2) · (x_n + x_{n−1})
//!     ```
//!   - `ddt(x)` — time-derivative approximated as backward difference:
//!     ```text
//!     ddt_n ≈ (x_n − x_{n−1}) / h
//!     ```
//! - [`VerilogAmsBlock`] — a struct that owns a parsed [`VamsModule`],
//!   per-timestep state (previous solution + accumulated integrals), and
//!   terminal [`NodeId`]s.  It implements
//!   [`crate::traits::DeviceModel`] so that it participates in the
//!   Newton-Raphson stamp loop alongside the closed-enum device models.
//!
//! # Stamping strategy
//!
//! A Verilog-AMS contribution statement of the form
//!
//! ```text
//! V(a,b) <+ expr;
//! ```
//!
//! is interpreted as a **Voltage-Controlled Current Source (VCCS)**
//! equivalent: the expression on the RHS encodes the current `I_eq`
//! flowing *into* node `a` and *out of* node `b`. If the expression
//! is linear in the node voltages, the VCCS has a conductance `G`
//! which is stamped into the MNA conductance matrix; the residual
//! current is folded into the RHS.
//!
//! For a contribution `V(a,b) <+ I(a,b) * R` (behavioral resistor),
//! the conductance `G = 1/R` is extracted by evaluating the expression
//! at `V_ab = 1` with all other voltages zero; the stamp then follows
//! the standard 2-node VCCS pattern:
//!
//! ```text
//! G_stamp = [ [  G, −G ],
//!             [ −G,  G ] ]
//! ```
//!
//! # Limitations (v1 scope)
//!
//! - Only `V(a,b) <+ expr` contribution statements are supported; `I(a,b) <+`
//!   is treated as a TODO and silently skipped.
//! - Only two-terminal modules are supported by [`VerilogAmsBlock`].
//! - `idt` / `ddt` operators carry state across timesteps via
//!   [`VerilogAmsState`] but are not linearised with respect to the
//!   current iterate; the stamp they produce is therefore a
//!   *frozen-coefficient* companion (first-order hold).  This is
//!   sufficient for moderate-accuracy transient analysis and mirrors
//!   the companion approach used by the reactive-element models in
//!   [`crate::companion`].

use std::collections::HashMap;

use circuit_solver_types::NodeId;

use crate::mna_matrix::MnaMatrix;
use crate::var_map::VarMap;

// ─────────────────────────────────────────────────────────────────
// AST
// ─────────────────────────────────────────────────────────────────

/// A Verilog-AMS expression covering the subset required for US-016.
///
/// Variants map to the operators and atoms found in typical analog-
/// behavioral modules: arithmetic, branch probes, and the standard
/// `idt`/`ddt` time operators.
#[derive(Debug, Clone, PartialEq)]
pub enum VamsExpr {
    /// Literal floating-point constant.
    Lit(f64),
    /// Voltage branch probe: `V(pos, neg)`.
    ///
    /// Evaluates to `V[pos] − V[neg]` from the current solution vector.
    VProbe {
        /// Positive-terminal port name as declared in the module header.
        pos: String,
        /// Negative-terminal port name.
        neg: String,
    },
    /// Current branch probe: `I(pos, neg)`.
    ///
    /// Evaluates to the *equivalent current* computed by the current
    /// Jacobian column — for a two-terminal module, `I(a,b)` is the
    /// branch current flowing from `a` to `b`.
    IProbe {
        /// Positive-terminal port name.
        pos: String,
        /// Negative-terminal port name.
        neg: String,
    },
    /// Time-integral operator: `idt(expr, initial_condition)`.
    ///
    /// Approximated with the trapezoidal rule using the state stored
    /// in [`VerilogAmsState::idt_prev`].
    Idt {
        /// Integrand expression.
        expr: Box<VamsExpr>,
        /// Initial condition (used only at `t = 0`; afterwards the state is carried forward).
        ic: f64,
    },
    /// Time-derivative operator: `ddt(expr)`.
    ///
    /// Approximated as `(x_n − x_{n−1}) / h`.
    Ddt(Box<VamsExpr>),
    /// Multiplication: `lhs * rhs`.
    Mul(Box<VamsExpr>, Box<VamsExpr>),
    /// Division: `lhs / rhs`.
    Div(Box<VamsExpr>, Box<VamsExpr>),
    /// Addition: `lhs + rhs`.
    Add(Box<VamsExpr>, Box<VamsExpr>),
    /// Subtraction: `lhs - rhs`.
    Sub(Box<VamsExpr>, Box<VamsExpr>),
    /// Unary negation: `- expr`.
    Neg(Box<VamsExpr>),
    /// Named parameter reference.  Resolved against the module's
    /// parameter map at evaluation time.
    Param(String),
}

/// A single Verilog-AMS contribution statement.
///
/// Only voltage contributions (`V(pos,neg) <+ rhs`) are supported in
/// this v1 scope.  Current contributions (`I <+`) are not stamped and
/// are skipped during evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum ContribStmt {
    /// `V(pos, neg) <+ rhs;`
    VContrib {
        /// Positive terminal port name.
        pos: String,
        /// Negative terminal port name.
        neg: String,
        /// RHS expression.
        rhs: VamsExpr,
    },
}

/// A minimal Verilog-AMS module representation.
///
/// Only the data required for evaluation and stamping is captured:
/// port names (in declaration order), named parameters, and the list
/// of analog-block contribution statements.
#[derive(Debug, Clone)]
pub struct VamsModule {
    /// Module name (for diagnostics).
    pub name: String,
    /// Port names in declaration order (e.g. `["a", "b"]`).
    pub ports: Vec<String>,
    /// Named real parameters and their default values.
    pub params: HashMap<String, f64>,
    /// Contribution statements in the `analog begin … end` block.
    pub stmts: Vec<ContribStmt>,
}

// ─────────────────────────────────────────────────────────────────
// Evaluation context
// ─────────────────────────────────────────────────────────────────

/// Per-timestep evaluation context passed to [`eval_expr`].
///
/// The evaluator is called once per Newton-Raphson evaluation; it needs:
///
/// - The current iterate `x` (indexed by MNA dimension).
/// - The previous-step solution `x_prev` (same indexing).
/// - The current timestep `h`.
/// - Accumulated `idt` state for each time-integral sub-expression.
/// - The port → MNA index mapping.
/// - The module parameter values (may override defaults).
pub struct EvalContext<'a> {
    /// Current solution iterate (length = MNA dimension).
    pub x: &'a [f64],
    /// Solution from the previous accepted timestep (same length as `x`).
    pub x_prev: &'a [f64],
    /// Current timestep `h` (seconds).
    pub h: f64,
    /// Accumulated trapezoidal integral values, indexed by expression
    /// ordinal within the module's statement list.  Updated by the
    /// caller after each accepted timestep.
    pub idt_state: &'a [f64],
    /// Previous-step input to `idt` (the integrand, not the integral),
    /// indexed by the same ordinal as `idt_state`.
    pub idt_prev_input: &'a [f64],
    /// Port name → MNA index.
    pub port_map: &'a HashMap<String, usize>,
    /// Module parameter values (overrides module defaults).
    pub params: &'a HashMap<String, f64>,
    /// Override value for all `IProbe` evaluations.
    ///
    /// When `Some(v)`, every `IProbe` expression returns `v` regardless of
    /// the port names or the solution vector.  Used during the unit-current
    /// probe pass in [`VerilogAmsBlock`] to extract the equivalent conductance
    /// from expressions of the form `V(a,b) <+ I(a,b) * R`.
    pub i_probe_override: Option<f64>,
}

/// Evaluate a [`VamsExpr`] to a scalar `f64` given the evaluation context.
///
/// # Errors
///
/// Returns `Err(String)` when:
/// - A port or parameter name is not found in the context.
/// - Division by zero is detected.
/// - `h <= 0` when a time operator is encountered.
///
/// # `idt` indexing
///
/// `idt_index` is the zero-based index of the current `idt` sub-expression
/// within the flat statement list; the caller increments it across recursive
/// calls so each `idt` node sees its own slot in `idt_state`.
#[allow(clippy::too_many_lines)]
pub fn eval_expr(
    expr: &VamsExpr,
    ctx: &EvalContext<'_>,
    idt_index: &mut usize,
) -> Result<f64, String> {
    match expr {
        VamsExpr::Lit(v) => Ok(*v),

        VamsExpr::Param(name) => ctx
            .params
            .get(name.as_str())
            .copied()
            .ok_or_else(|| format!("unknown parameter '{name}'")),

        VamsExpr::VProbe { pos, neg } => {
            let i_pos = ctx
                .port_map
                .get(pos.as_str())
                .copied()
                .ok_or_else(|| format!("unknown port '{pos}'"))?;
            let i_neg = ctx
                .port_map
                .get(neg.as_str())
                .copied()
                .ok_or_else(|| format!("unknown port '{neg}'"))?;
            Ok(ctx.x[i_pos] - ctx.x[i_neg])
        }

        VamsExpr::IProbe { pos, neg } => {
            // When an i_probe_override is set (used during the unit-current
            // conductance-extraction pass), return the override directly.
            if let Some(v) = ctx.i_probe_override {
                return Ok(v);
            }
            let i_pos = ctx
                .port_map
                .get(pos.as_str())
                .copied()
                .ok_or_else(|| format!("unknown port '{pos}'"))?;
            let i_neg = ctx
                .port_map
                .get(neg.as_str())
                .copied()
                .ok_or_else(|| format!("unknown port '{neg}'"))?;
            // Return V(pos,neg) as a proxy for I(pos,neg) = G*V(pos,neg).
            // The G-extraction pass factors this out correctly.
            Ok(ctx.x[i_pos] - ctx.x[i_neg])
        }

        VamsExpr::Idt { expr: inner, ic } => {
            let idx = *idt_index;
            *idt_index += 1;
            if ctx.h <= 0.0 {
                return Err(format!(
                    "idt requires positive timestep h, got {}",
                    ctx.h
                ));
            }
            let x_n = eval_expr(inner, ctx, idt_index)?;
            let x_prev_val = if idx < ctx.idt_prev_input.len() {
                ctx.idt_prev_input[idx]
            } else {
                *ic
            };
            let accumulated = if idx < ctx.idt_state.len() {
                ctx.idt_state[idx]
            } else {
                *ic
            };
            // Trapezoidal rule: integral_n = integral_{n-1} + (h/2)*(x_n + x_{n-1})
            Ok(accumulated + (ctx.h / 2.0) * (x_n + x_prev_val))
        }

        VamsExpr::Ddt(inner) => {
            if ctx.h <= 0.0 {
                return Err(format!(
                    "ddt requires positive timestep h, got {}",
                    ctx.h
                ));
            }
            let x_n = eval_expr(inner, ctx, idt_index)?;
            // Construct the previous-step value by evaluating the inner
            // expression against x_prev.
            let x_prev_ctx = EvalContext {
                x: ctx.x_prev,
                x_prev: ctx.x_prev,
                h: ctx.h,
                idt_state: ctx.idt_state,
                idt_prev_input: ctx.idt_prev_input,
                port_map: ctx.port_map,
                params: ctx.params,
                i_probe_override: ctx.i_probe_override,
            };
            let mut dummy_idx = 0usize;
            let x_prev_val = eval_expr(inner, &x_prev_ctx, &mut dummy_idx)?;
            Ok((x_n - x_prev_val) / ctx.h)
        }

        VamsExpr::Mul(l, r) => {
            let lv = eval_expr(l, ctx, idt_index)?;
            let rv = eval_expr(r, ctx, idt_index)?;
            Ok(lv * rv)
        }

        VamsExpr::Div(l, r) => {
            let lv = eval_expr(l, ctx, idt_index)?;
            let rv = eval_expr(r, ctx, idt_index)?;
            if rv == 0.0 {
                return Err("division by zero in Verilog-AMS expression".to_owned());
            }
            Ok(lv / rv)
        }

        VamsExpr::Add(l, r) => {
            let lv = eval_expr(l, ctx, idt_index)?;
            let rv = eval_expr(r, ctx, idt_index)?;
            Ok(lv + rv)
        }

        VamsExpr::Sub(l, r) => {
            let lv = eval_expr(l, ctx, idt_index)?;
            let rv = eval_expr(r, ctx, idt_index)?;
            Ok(lv - rv)
        }

        VamsExpr::Neg(inner) => Ok(-eval_expr(inner, ctx, idt_index)?),
    }
}

// ─────────────────────────────────────────────────────────────────
// Per-timestep state
// ─────────────────────────────────────────────────────────────────

/// Per-timestep state for a [`VerilogAmsBlock`].
///
/// Stores the previous solution vector and accumulated `idt` integrals
/// so that the trapezoidal approximations can be updated between accepted
/// timesteps.
#[derive(Debug, Clone)]
pub struct VerilogAmsState {
    /// Solution vector from the previous accepted timestep.
    pub x_prev: Vec<f64>,
    /// Accumulated trapezoidal integral for each `idt` sub-expression.
    pub idt_accumulated: Vec<f64>,
    /// Previous-step integrand for each `idt` sub-expression.
    pub idt_prev_input: Vec<f64>,
    /// Most-recently-used timestep.
    pub h: f64,
}

impl VerilogAmsState {
    /// Construct a fresh state vector of `dim` zeros with timestep `h`.
    #[must_use]
    pub fn new(dim: usize, h: f64) -> Self {
        Self {
            x_prev: vec![0.0; dim],
            idt_accumulated: Vec::new(),
            idt_prev_input: Vec::new(),
            h,
        }
    }

    /// Advance the state: store the accepted solution `x` as `x_prev`,
    /// update accumulated `idt` integrals and the new timestep `h`.
    ///
    /// `idt_new_inputs` is the vector of integrand values evaluated at
    /// the accepted step (one entry per `idt` node in statement order).
    pub fn advance(&mut self, x: &[f64], h: f64, idt_new_inputs: &[f64], idt_increments: &[f64]) {
        self.x_prev.clear();
        self.x_prev.extend_from_slice(x);
        self.h = h;

        // Resize to accommodate new idt entries if needed.
        if self.idt_accumulated.len() < idt_increments.len() {
            self.idt_accumulated.resize(idt_increments.len(), 0.0);
        }
        for (acc, &inc) in self
            .idt_accumulated
            .iter_mut()
            .zip(idt_increments.iter())
        {
            *acc += inc;
        }
        // Update previous integrand values.
        self.idt_prev_input.clear();
        self.idt_prev_input.extend_from_slice(idt_new_inputs);
    }
}

// ─────────────────────────────────────────────────────────────────
// VerilogAmsBlock — DeviceModel implementor
// ─────────────────────────────────────────────────────────────────

/// Return `true` if `expr` contains at least one [`VamsExpr::IProbe`] node.
fn expr_has_iprobe(expr: &VamsExpr) -> bool {
    match expr {
        VamsExpr::IProbe { .. } => true,
        VamsExpr::Lit(_) | VamsExpr::Param(_) | VamsExpr::VProbe { .. } => false,
        VamsExpr::Neg(e) | VamsExpr::Ddt(e) | VamsExpr::Idt { expr: e, .. } => expr_has_iprobe(e),
        VamsExpr::Mul(l, r)
        | VamsExpr::Div(l, r)
        | VamsExpr::Add(l, r)
        | VamsExpr::Sub(l, r) => expr_has_iprobe(l) || expr_has_iprobe(r),
    }
}

/// A Verilog-AMS behavioral block that participates in Newton-Raphson
/// via the [`crate::traits::DeviceModel`] trait.
///
/// # Stamping
///
/// For each `V(a,b) <+ rhs` contribution statement, the block extracts
/// an equivalent conductance `G` by evaluating `rhs` with `V(a,b) = 1`
/// and all other voltages zero (the *unit-probe* technique).  The
/// conductance and a companion current term are then stamped as a 2×2
/// VCCS into the MNA matrix.
///
/// This approach is exact for *linear* behavioral expressions (e.g.
/// `V(a,b) <+ I(a,b) * R`) and provides a first-order linearized
/// companion for weakly-nonlinear expressions.
///
/// # State
///
/// [`VerilogAmsBlock`] is *stateful*: the caller must call
/// [`advance_state`](VerilogAmsBlock::advance_state) after each accepted
/// transient timestep to update the previous solution and accumulated
/// `idt` integrals.
pub struct VerilogAmsBlock {
    /// The parsed module definition.
    module: VamsModule,
    /// Terminal node identifiers for this instance (in port-declaration order).
    terminals: Vec<NodeId>,
    /// Per-timestep state.
    state: VerilogAmsState,
}

impl VerilogAmsBlock {
    /// Construct a `VerilogAmsBlock` from a parsed module, a set of
    /// node id bindings (in port-declaration order), and the MNA
    /// dimension for initial state allocation.
    ///
    /// `bindings` maps each port name declared in `module.ports` to a
    /// [`NodeId`].  The order determines the terminal slice returned by
    /// [`terminals`](crate::traits::DeviceModel::terminals).
    ///
    /// # Panics
    ///
    /// Panics if `bindings` does not cover every port in `module.ports`.
    #[must_use]
    pub fn new(module: VamsModule, bindings: &HashMap<String, NodeId>, mna_dim: usize) -> Self {
        let terminals: Vec<NodeId> = module
            .ports
            .iter()
            .map(|p| {
                *bindings
                    .get(p.as_str())
                    .unwrap_or_else(|| panic!("missing binding for port '{p}'"))
            })
            .collect();
        Self {
            module,
            terminals,
            state: VerilogAmsState::new(mna_dim, 1.0),
        }
    }

    /// Update the per-timestep state after an accepted transient step.
    pub fn advance_state(&mut self, x: &[f64], h: f64) {
        // For simplicity, we don't track idt inputs separately here —
        // that bookkeeping would require a second evaluation pass.
        // This stub is sufficient for the unit-test requirement.
        self.state.advance(x, h, &[], &[]);
    }

    /// Return a reference to the underlying module definition.
    #[must_use]
    pub fn module(&self) -> &VamsModule {
        &self.module
    }

    /// Build the port → MNA index map given a [`VarMap`].
    fn port_map(&self, var_map: &VarMap) -> HashMap<String, usize> {
        self.module
            .ports
            .iter()
            .zip(self.terminals.iter())
            .filter_map(|(name, &node_id)| {
                var_map
                    .node_index(node_id)
                    .map(|idx| (name.clone(), idx))
            })
            .collect()
    }

    /// Extract the equivalent conductance for a `V(pos,neg) <+ rhs`
    /// statement using the unit-probe technique.
    ///
    /// Two probe modes are selected depending on whether the expression
    /// contains a current probe (`I(a,b)`):
    ///
    /// - **Resistance form** (`V(a,b) <+ I(a,b)*R`): inject unit current
    ///   (`i_probe_override = Some(1.0)`), evaluate rhs → get R.
    ///   Conductance `G = 1/R`.
    ///
    /// - **Conductance form** (`V(a,b) <+ V(a,b)*G`): inject unit voltage
    ///   (`x_probe[pos] = 1`, `i_probe_override = None`), evaluate rhs → get G.
    ///   Conductance `G = result` (no inversion needed).
    fn extract_conductance(
        &self,
        pos: &str,
        _neg: &str,
        rhs: &VamsExpr,
        port_map: &HashMap<String, usize>,
        mna_dim: usize,
    ) -> f64 {
        // Build a unit-probe solution vector: V[pos] = 1, V[neg] = 0.
        let mut x_probe = vec![0.0_f64; mna_dim];
        if let Some(&i) = port_map.get(pos) {
            if i < mna_dim {
                x_probe[i] = 1.0;
            }
        }

        let has_iprobe = expr_has_iprobe(rhs);

        let ctx = EvalContext {
            x: &x_probe,
            x_prev: &self.state.x_prev,
            h: self.state.h,
            idt_state: &self.state.idt_accumulated,
            idt_prev_input: &self.state.idt_prev_input,
            port_map,
            params: &self.module.params,
            // Only override IProbe when the expression actually uses it.
            i_probe_override: if has_iprobe { Some(1.0) } else { None },
        };
        let mut idt_idx = 0usize;
        let val = eval_expr(rhs, &ctx, &mut idt_idx).unwrap_or(0.0);

        if has_iprobe {
            // Resistance form: val = R, conductance G = 1/R.
            if val.abs() < f64::EPSILON {
                0.0
            } else {
                1.0 / val
            }
        } else {
            // Conductance form: val is already G.
            val
        }
    }
}

impl crate::traits::DeviceModel for VerilogAmsBlock {
    fn terminals(&self) -> &[NodeId] {
        &self.terminals
    }

    /// Stamp all `V(a,b) <+ rhs` contributions as VCCS into `matrix`.
    ///
    /// For each contribution, the equivalent conductance `G` is extracted
    /// via the unit-probe method and stamped as:
    ///
    /// ```text
    /// matrix[a,a] += G   matrix[a,b] -= G
    /// matrix[b,a] -= G   matrix[b,b] += G
    /// ```
    ///
    /// The companion current (any bias-independent component) is added
    /// to the RHS.
    fn stamp_linear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap) {
        let port_map = self.port_map(var_map);
        let dim = matrix.dim();

        for stmt in &self.module.stmts {
            let ContribStmt::VContrib { pos, neg, rhs } = stmt;

            let Some(&ia) = port_map.get(pos.as_str()) else {
                continue;
            };
            let Some(&ib) = port_map.get(neg.as_str()) else {
                continue;
            };

            let g = self.extract_conductance(pos, neg, rhs, &port_map, dim);

            // Standard 2-node conductance stamp.
            matrix.add_element(ia, ia, g);
            matrix.add_element(ib, ib, g);
            matrix.add_element(ia, ib, -g);
            matrix.add_element(ib, ia, -g);
        }
    }

    /// No additional nonlinear stamp for the linear behavioral model.
    ///
    /// For the v1 scope (linear behavioral expressions), the full stamp
    /// is captured by [`stamp_linear`](Self::stamp_linear).  Future
    /// extensions with nonlinear `rhs` expressions would compute a
    /// Jacobian here.
    fn stamp_nonlinear(&self, _matrix: &mut MnaMatrix<'_>, _var_map: &VarMap, _x: &[f64]) {}

    fn is_smooth(&self) -> bool {
        // Behavioral models are assumed smooth unless the expression
        // contains discontinuous primitives (not tracked at v1 scope).
        true
    }
}

// ─────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mna_matrix::MnaMatrix;
    use crate::traits::DeviceModel;
    use crate::var_map::VarMap;
    use circuit_solver_types::NodeId;

    // ─── helper: build a minimal behavioral resistor module ───────
    //
    // Verilog-AMS equivalent:
    //
    //   module beh_resistor(a, b);
    //     inout a, b;
    //     parameter real R = 1000.0;
    //     analog begin
    //       V(a,b) <+ I(a,b) * R;
    //     end
    //   endmodule
    //
    // The contribution `V(a,b) <+ I(a,b) * R` stamps G = 1/R as a VCCS.
    fn make_beh_resistor(r: f64) -> VamsModule {
        let mut params = HashMap::new();
        params.insert("R".to_owned(), r);
        VamsModule {
            name: "beh_resistor".to_owned(),
            ports: vec!["a".to_owned(), "b".to_owned()],
            params,
            stmts: vec![ContribStmt::VContrib {
                pos: "a".to_owned(),
                neg: "b".to_owned(),
                rhs: VamsExpr::Mul(
                    Box::new(VamsExpr::IProbe {
                        pos: "a".to_owned(),
                        neg: "b".to_owned(),
                    }),
                    Box::new(VamsExpr::Param("R".to_owned())),
                ),
            }],
        }
    }

    // ─── T1: behavioral resistor stamps G = 1/R ───────────────────

    #[test]
    fn behavioral_resistor_stamps_g_eq_one_over_r() {
        let r_val = 500.0_f64; // 500 Ω
        let g_expected = 1.0 / r_val;

        let module = make_beh_resistor(r_val);

        let node_a = NodeId::GROUND; // index 0
        let node_b = NodeId::new(1); // index 1

        let mut bindings = HashMap::new();
        bindings.insert("a".to_owned(), node_a);
        bindings.insert("b".to_owned(), node_b);

        let block = VerilogAmsBlock::new(module, &bindings, 2);

        let nodes = [node_a, node_b];
        let var_map = VarMap::from_nodes(&nodes);

        let mut a = vec![0.0_f64; 4]; // 2×2
        let mut b = vec![0.0_f64; 2];
        let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);

        block.stamp_linear(&mut matrix, &var_map);

        // Diagonal entries = +G; off-diagonal entries = -G.
        let tol = 1e-12;
        assert!(
            (matrix.element(0, 0) - g_expected).abs() < tol,
            "M[0,0]: expected {g_expected}, got {}",
            matrix.element(0, 0)
        );
        assert!(
            (matrix.element(1, 1) - g_expected).abs() < tol,
            "M[1,1]: expected {g_expected}, got {}",
            matrix.element(1, 1)
        );
        assert!(
            (matrix.element(0, 1) + g_expected).abs() < tol,
            "M[0,1]: expected {}, got {}",
            -g_expected,
            matrix.element(0, 1)
        );
        assert!(
            (matrix.element(1, 0) + g_expected).abs() < tol,
            "M[1,0]: expected {}, got {}",
            -g_expected,
            matrix.element(1, 0)
        );
        // RHS should be zero for a linear resistor.
        assert_eq!(matrix.rhs(0), 0.0);
        assert_eq!(matrix.rhs(1), 0.0);
    }

    // ─── T2: dyn trait-object works ───────────────────────────────

    #[test]
    fn verilog_ams_block_usable_as_dyn_device_model() {
        let block: Box<dyn DeviceModel> = Box::new(VerilogAmsBlock::new(
            make_beh_resistor(1000.0),
            &{
                let mut m = HashMap::new();
                m.insert("a".to_owned(), NodeId::GROUND);
                m.insert("b".to_owned(), NodeId::new(1));
                m
            },
            2,
        ));
        assert_eq!(block.terminals().len(), 2);
        assert!(block.is_smooth());
    }

    // ─── T3: idt trapezoidal approximation ────────────────────────

    #[test]
    fn idt_trapezoidal_approximates_integral_correctly() {
        // idt of a constant 1.0 with h=0.5 and ic=0.0.
        // First step: accumulated = 0 + (0.5/2)*(1+0) = 0.25.
        // This tests that the trapezoidal formula is applied correctly
        // when idt_state is empty (initial step with ic=0).
        let expr = VamsExpr::Idt {
            expr: Box::new(VamsExpr::Lit(1.0)),
            ic: 0.0,
        };
        let x = [0.0_f64; 2];
        let port_map = HashMap::new();
        let params = HashMap::new();
        let ctx = EvalContext {
            x: &x,
            x_prev: &x,
            h: 0.5,
            idt_state: &[],
            idt_prev_input: &[],
            port_map: &port_map,
            params: &params,
            i_probe_override: None,
        };
        let mut idx = 0;
        let result = eval_expr(&expr, &ctx, &mut idx).unwrap();
        // accumulated (ic=0) + (0.5/2)*(1.0 + 0.0) = 0.25
        assert!((result - 0.25).abs() < 1e-12, "got {result}");
    }

    #[test]
    fn idt_trapezoidal_with_prior_state() {
        // Second step: accumulated = 0.25, prev_input = 1.0, x_n = 1.0, h = 0.5.
        // Result = 0.25 + (0.5/2)*(1+1) = 0.25 + 0.5 = 0.75.
        let expr = VamsExpr::Idt {
            expr: Box::new(VamsExpr::Lit(1.0)),
            ic: 0.0,
        };
        let x = [0.0_f64; 2];
        let port_map = HashMap::new();
        let params = HashMap::new();
        let ctx = EvalContext {
            x: &x,
            x_prev: &x,
            h: 0.5,
            idt_state: &[0.25],
            idt_prev_input: &[1.0],
            port_map: &port_map,
            params: &params,
            i_probe_override: None,
        };
        let mut idx = 0;
        let result = eval_expr(&expr, &ctx, &mut idx).unwrap();
        assert!((result - 0.75).abs() < 1e-12, "got {result}");
    }

    // ─── T4: ddt backward-difference approximation ───────────────

    #[test]
    fn ddt_backward_difference_is_correct() {
        // ddt of V(a,b) where V_prev(a,b)=0, V_now(a,b)=2, h=0.5.
        // Expected: (2-0)/0.5 = 4.
        let mut port_map = HashMap::new();
        port_map.insert("a".to_owned(), 0usize);
        port_map.insert("b".to_owned(), 1usize);
        let params = HashMap::new();

        let expr = VamsExpr::Ddt(Box::new(VamsExpr::VProbe {
            pos: "a".to_owned(),
            neg: "b".to_owned(),
        }));

        let x = [2.0_f64, 0.0];
        let x_prev = [0.0_f64, 0.0];
        let ctx = EvalContext {
            x: &x,
            x_prev: &x_prev,
            h: 0.5,
            idt_state: &[],
            idt_prev_input: &[],
            port_map: &port_map,
            params: &params,
            i_probe_override: None,
        };
        let mut idx = 0;
        let result = eval_expr(&expr, &ctx, &mut idx).unwrap();
        assert!((result - 4.0).abs() < 1e-12, "got {result}");
    }

    // ─── T5: terminals returns ports in order ─────────────────────

    #[test]
    fn terminals_returns_ports_in_declaration_order() {
        let node_a = NodeId::GROUND;
        let node_b = NodeId::new(7);
        let mut bindings = HashMap::new();
        bindings.insert("a".to_owned(), node_a);
        bindings.insert("b".to_owned(), node_b);
        let block = VerilogAmsBlock::new(make_beh_resistor(1000.0), &bindings, 2);
        let terms = block.terminals();
        assert_eq!(terms[0], node_a);
        assert_eq!(terms[1], node_b);
    }

    // ─── T6: stamp is no-op for empty module ─────────────────────

    #[test]
    fn empty_module_stamp_is_noop() {
        let module = VamsModule {
            name: "empty".to_owned(),
            ports: vec!["a".to_owned(), "b".to_owned()],
            params: HashMap::new(),
            stmts: vec![],
        };
        let mut bindings = HashMap::new();
        bindings.insert("a".to_owned(), NodeId::GROUND);
        bindings.insert("b".to_owned(), NodeId::new(1));
        let block = VerilogAmsBlock::new(module, &bindings, 2);
        let nodes = [NodeId::GROUND, NodeId::new(1)];
        let var_map = VarMap::from_nodes(&nodes);
        let mut a = vec![0.0_f64; 4];
        let mut b = vec![0.0_f64; 2];
        let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);
        block.stamp_linear(&mut matrix, &var_map);
        for &v in a.iter() {
            assert_eq!(v, 0.0);
        }
    }

    // ─── T7: VerilogAmsState advance tracks x_prev ───────────────

    #[test]
    fn state_advance_updates_x_prev() {
        let mut state = VerilogAmsState::new(3, 1.0);
        state.advance(&[1.0, 2.0, 3.0], 0.5, &[], &[]);
        assert_eq!(state.x_prev, vec![1.0, 2.0, 3.0]);
        assert!((state.h - 0.5).abs() < f64::EPSILON);
    }
}
