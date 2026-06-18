//! `circuit_solver_delta` CLI.
//!
//! Usage:
//!   circuit_solver_delta solve <netlist> [OPTIONS]
//!
//! Parses a SPICE netlist, runs the requested analysis, and writes the result
//! to an output file.

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::Instant,
};

use clap::{Parser, Subcommand, ValueEnum};

use circuit_solver_delta::{
    linear_elements::{Capacitor, CurrentSource, Inductor, Resistor, VoltageSource},
    netlist::{tokenize, NetlistToken},
    newton_raphson::NewtonRaphson,
    traits::DeviceModel,
    transient::{TransientAnalysis, TransientSolution},
    VarMap,
};

// ── CLI argument types ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum Analysis {
    Dc,
    Ac,
    Tran,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum Format {
    Nutmeg,
    Vcd,
    Parquet,
}

#[derive(Parser, Debug)]
#[command(name = "circuit_solver_delta")]
#[command(about = "SPICE-like circuit simulator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Solve a circuit netlist and write results.
    Solve {
        /// Path to the SPICE netlist file.
        netlist: PathBuf,

        /// Analysis type.
        #[arg(long, value_enum, default_value = "dc")]
        analysis: Analysis,

        /// Output file path (default: <netlist>.out).
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,

        /// Output format.
        #[arg(long, value_enum, default_value = "nutmeg")]
        format: Format,

        // ── Transient flags ────────────────────────────────────────────
        /// Transient stop time (e.g. 1e-3, 1m).  Required for --analysis tran.
        #[arg(long)]
        tran_stop: Option<String>,

        /// Transient timestep (e.g. 1e-9, 1n).
        #[arg(long)]
        tran_step: Option<String>,

        // ── AC flags ──────────────────────────────────────────────────
        /// AC start frequency (Hz).
        #[arg(long)]
        ac_start: Option<String>,

        /// AC stop frequency (Hz).
        #[arg(long)]
        ac_stop: Option<String>,

        /// AC number of points per decade.
        #[arg(long)]
        ac_points: Option<u32>,
    },
}

// ── SPICE value parser ────────────────────────────────────────────────────────

/// Parse a SPICE-style number string (e.g. `"1k"`, `"10n"`, `"1.5MEG"`, `"1e-3"`)
/// into an `f64`.
fn parse_spice_value(s: &str) -> Result<f64, String> {
    if s.is_empty() {
        return Err("empty value string".into());
    }
    // Find where the numeric part ends.
    let num_end = s
        .find(|c: char| c.is_alphabetic())
        .unwrap_or(s.len());
    let (num_str, suffix) = s.split_at(num_end);
    let base: f64 = num_str
        .parse()
        .map_err(|_| format!("cannot parse numeric part of '{s}'"))?;
    let scale = match suffix.to_uppercase().as_str() {
        "" | "V" | "A" | "S" | "HZ" => 1.0,
        "T" => 1e12,
        "G" | "GIG" => 1e9,
        "MEG" => 1e6,
        "K" => 1e3,
        "M" => 1e-3,
        "U" | "MU" => 1e-6,
        "N" => 1e-9,
        "P" => 1e-12,
        "F" => 1e-15,
        other => return Err(format!("unknown SPICE suffix '{other}' in '{s}'")),
    };
    Ok(base * scale)
}

// ── Netlist → devices ─────────────────────────────────────────────────────────

/// Build a `VarMap` and device list from parsed netlist tokens.
fn build_circuit(
    tokens: &[NetlistToken],
) -> Result<(VarMap, Vec<Box<dyn DeviceModel>>), String> {
    let mut vm = VarMap::new();
    let mut devices: Vec<Box<dyn DeviceModel>> = Vec::new();

    // Two-pass: first register all nodes and branches (so VarMap is complete
    // before we build devices), then build devices.

    // Pass 1 — register nodes + branches.
    for tok in tokens {
        match tok {
            NetlistToken::Resistor { n_pos, n_neg, .. }
            | NetlistToken::Capacitor { n_pos, n_neg, .. }
            | NetlistToken::Inductor { n_pos, n_neg, .. }
            | NetlistToken::CurrentSource { n_pos, n_neg, .. } => {
                if n_pos != "0" { vm.add_node(n_pos); }
                if n_neg != "0" { vm.add_node(n_neg); }
            }
            NetlistToken::VoltageSource { name, n_pos, n_neg, .. } => {
                if n_pos != "0" { vm.add_node(n_pos); }
                if n_neg != "0" { vm.add_node(n_neg); }
                vm.add_branch(&format!("V{name}"));
            }
            _ => {}
        }
    }

    // Pass 2 — build device objects.
    for tok in tokens {
        match tok {
            NetlistToken::Resistor { name: _, n_pos, n_neg, value } => {
                let r = parse_spice_value(value)?;
                devices.push(Box::new(Resistor::new(n_pos.clone(), n_neg.clone(), r)));
            }
            NetlistToken::Capacitor { name: _, n_pos, n_neg, value } => {
                let c = parse_spice_value(value)?;
                devices.push(Box::new(Capacitor::new(n_pos.clone(), n_neg.clone(), c)));
            }
            NetlistToken::Inductor { name, n_pos, n_neg, value } => {
                let l = parse_spice_value(value)?;
                let branch = format!("L{name}");
                // Inductors need a branch-current variable too.
                vm.add_branch(&branch);
                devices.push(Box::new(Inductor::new(n_pos.clone(), n_neg.clone(), branch, l)));
            }
            NetlistToken::VoltageSource { name, n_pos, n_neg, value } => {
                let v = parse_spice_value(value)?;
                let branch = format!("V{name}");
                devices.push(Box::new(VoltageSource::new(
                    n_pos.clone(),
                    n_neg.clone(),
                    branch,
                    v,
                )));
            }
            NetlistToken::CurrentSource { name: _, n_pos, n_neg, value } => {
                let i = parse_spice_value(value)?;
                devices.push(Box::new(CurrentSource::new(n_pos.clone(), n_neg.clone(), i)));
            }
            // Directives and unsupported elements are skipped for device building.
            _ => {}
        }
    }

    Ok((vm, devices))
}

// ── DC operating-point solver ─────────────────────────────────────────────────

/// Run a Newton-Raphson DC operating-point analysis.
///
/// Returns `(var_names, solution_vector)` where index 0 = ground (always 0).
fn run_dc(
    vm: &VarMap,
    devices: &[Box<dyn DeviceModel>],
) -> Result<(Vec<String>, Vec<f64>), String> {
    let n = vm.len() - 1; // exclude ground at index 0
    if n == 0 {
        return Ok((vec!["0".into()], vec![0.0]));
    }
    let nr = NewtonRaphson::default();
    let x = nr
        .solve(n, devices, vm)
        .map_err(|e| format!("DC convergence failed: {e}"))?;

    // Build name list (index 0 = ground).
    let mut names = vec!["0".to_string()];
    for i in 1..=n {
        names.push(vm.var_name(i).unwrap_or("?").to_string());
    }

    // Prepend ground voltage 0.0 to align with names.
    let mut result = vec![0.0];
    result.extend_from_slice(&x);

    Ok((names, result))
}

// ── Transient solver ──────────────────────────────────────────────────────────

fn run_tran(
    vm: &VarMap,
    devices: Vec<Box<dyn DeviceModel>>,
    t_stop: f64,
    h_step: f64,
) -> Result<TransientSolution, String> {
    let mut analysis = TransientAnalysis::builder(0.0, t_stop, vm, devices)
        .h_initial(h_step)
        .h_max(h_step)
        .rtol(1e-3)
        .atol(1e-6)
        .build();
    analysis.run().map_err(|e| format!("Transient integration failed: {e}"))
}

// ── Output writers ────────────────────────────────────────────────────────────

/// Write a DC result as SPICE nutmeg ASCII format.
fn write_nutmeg_dc(
    path: &Path,
    names: &[String],
    values: &[f64],
) -> std::io::Result<()> {
    let mut out = fs::File::create(path)?;
    writeln!(out, "Title: circuit_solver_delta DC analysis")?;
    writeln!(out, "Date: {}", chrono_now())?;
    writeln!(out, "Plotname: DC operating point")?;
    writeln!(out, "Flags: real")?;
    writeln!(out, "No. Variables: {}", names.len())?;
    writeln!(out, "No. Points: 1")?;
    writeln!(out, "Variables:")?;
    for (i, name) in names.iter().enumerate() {
        writeln!(out, "\t{i}\t{name}\tvoltage")?;
    }
    writeln!(out, "Values:")?;
    writeln!(out, "0")?;
    for v in values {
        writeln!(out, "\t{v:.12e}")?;
    }
    Ok(())
}

/// Write a transient result as SPICE nutmeg ASCII format.
fn write_nutmeg_tran(path: &Path, sol: &TransientSolution) -> std::io::Result<()> {
    // Collect ordered signal names: time first, then sorted waveform keys.
    let mut sig_names: Vec<String> = vec!["time".into()];
    let mut keys: Vec<String> = sol.waveforms.keys().cloned().collect();
    keys.sort();
    sig_names.extend(keys.iter().cloned());

    let n_pts = sol.times.len();
    let mut out = fs::File::create(path)?;
    writeln!(out, "Title: circuit_solver_delta transient analysis")?;
    writeln!(out, "Date: {}", chrono_now())?;
    writeln!(out, "Plotname: Transient Analysis")?;
    writeln!(out, "Flags: real")?;
    writeln!(out, "No. Variables: {}", sig_names.len())?;
    writeln!(out, "No. Points: {n_pts}")?;
    writeln!(out, "Variables:")?;
    for (i, name) in sig_names.iter().enumerate() {
        let kind = if name == "time" { "time" } else { "voltage" };
        writeln!(out, "\t{i}\t{name}\t{kind}")?;
    }
    writeln!(out, "Values:")?;
    for (pt_idx, &t) in sol.times.iter().enumerate() {
        writeln!(out, "{pt_idx}")?;
        writeln!(out, "\t{t:.12e}")?;
        for k in &keys {
            let v = sol.waveforms[k][pt_idx];
            writeln!(out, "\t{v:.12e}")?;
        }
    }
    Ok(())
}

/// Write a DC result as a CSV file (used for --format parquet; a real Parquet
/// writer requires arrow2/parquet2 which are not yet in this crate's deps).
fn write_parquet_dc(path: &Path, names: &[String], values: &[f64]) -> std::io::Result<()> {
    let mut out = fs::File::create(path)?;
    writeln!(out, "{}", names.join(","))?;
    let row: Vec<String> = values.iter().map(|v| format!("{v:.12e}")).collect();
    writeln!(out, "{}", row.join(","))?;
    Ok(())
}

/// Write a transient result as CSV (parquet format placeholder).
fn write_parquet_tran(path: &Path, sol: &TransientSolution) -> std::io::Result<()> {
    let mut keys: Vec<String> = sol.waveforms.keys().cloned().collect();
    keys.sort();
    let mut out = fs::File::create(path)?;
    // Header
    let mut header = vec!["time".to_string()];
    header.extend(keys.iter().cloned());
    writeln!(out, "{}", header.join(","))?;
    // Rows
    for (i, &t) in sol.times.iter().enumerate() {
        let mut row = vec![format!("{t:.12e}")];
        for k in &keys {
            row.push(format!("{:.12e}", sol.waveforms[k][i]));
        }
        writeln!(out, "{}", row.join(","))?;
    }
    Ok(())
}

/// Write a DC result as a minimal VCD file (digital view of final voltages).
fn write_vcd_dc(path: &Path, names: &[String], values: &[f64]) -> std::io::Result<()> {
    let mut out = fs::File::create(path)?;
    writeln!(out, "$timescale 1ps $end")?;
    writeln!(out, "$scope module circuit_solver_delta $end")?;
    // Emit only non-ground nodes.
    let sigs: Vec<(&str, f64)> = names
        .iter()
        .zip(values.iter())
        .filter(|(n, _)| n.as_str() != "0")
        .map(|(n, &v)| (n.as_str(), v))
        .collect();
    for (i, (name, _)) in sigs.iter().enumerate() {
        writeln!(out, "$var real 64 v{i} {name} $end")?;
    }
    writeln!(out, "$upscope $end")?;
    writeln!(out, "$enddefinitions $end")?;
    writeln!(out, "#0")?;
    for (i, (_, val)) in sigs.iter().enumerate() {
        writeln!(out, "r{val:.6e} v{i}")?;
    }
    writeln!(out, "#1")?;
    Ok(())
}

/// Write a transient result as a minimal VCD (real-valued signals).
fn write_vcd_tran(path: &Path, sol: &TransientSolution) -> std::io::Result<()> {
    let mut keys: Vec<String> = sol.waveforms.keys().cloned().collect();
    keys.sort();
    let mut out = fs::File::create(path)?;
    writeln!(out, "$timescale 1ps $end")?;
    writeln!(out, "$scope module circuit_solver_delta $end")?;
    for (i, name) in keys.iter().enumerate() {
        writeln!(out, "$var real 64 v{i} {name} $end")?;
    }
    writeln!(out, "$upscope $end")?;
    writeln!(out, "$enddefinitions $end")?;
    for (pt, &t) in sol.times.iter().enumerate() {
        // Convert seconds to picoseconds (integer) for VCD timestamp.
        let t_ps = (t * 1e12).round() as u64;
        writeln!(out, "#{t_ps}")?;
        for (i, k) in keys.iter().enumerate() {
            let v = sol.waveforms[k][pt];
            writeln!(out, "r{v:.6e} v{i}")?;
        }
    }
    Ok(())
}

// ── Helpers ────────────────────────────────────────────────────────────────────

fn chrono_now() -> String {
    // Simple timestamp without chrono dependency.
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("Unix timestamp {secs}")
}

fn default_output(netlist: &Path, format: Format) -> PathBuf {
    let ext = match format {
        Format::Nutmeg => "raw",
        Format::Vcd => "vcd",
        Format::Parquet => "csv",
    };
    netlist.with_extension(ext)
}

// ── Entry point ────────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let Commands::Solve {
        netlist,
        analysis,
        output,
        format,
        tran_stop,
        tran_step,
        ac_start: _,
        ac_stop: _,
        ac_points: _,
    } = cli.command;

    // ── Read netlist ──────────────────────────────────────────────────────
    let src = fs::read_to_string(&netlist)
        .map_err(|e| format!("cannot read '{}': {e}", netlist.display()))?;

    let (tokens, warnings, _models) = tokenize(&src);
    for w in &warnings {
        eprintln!("warning: line {}: {}", w.line, w.reason);
    }

    let (vm, devices) = build_circuit(&tokens)?;

    let out_path = output.unwrap_or_else(|| default_output(&netlist, format));

    // ── Run analysis ──────────────────────────────────────────────────────
    let t0 = Instant::now();

    match analysis {
        Analysis::Dc => {
            let (names, values) = run_dc(&vm, &devices)?;
            let elapsed = t0.elapsed();

            // Report convergence to stderr.
            eprintln!("DC analysis converged in {:.3} ms", elapsed.as_secs_f64() * 1e3);
            for (name, val) in names.iter().zip(values.iter()) {
                if name != "0" {
                    eprintln!("  V({name}) = {val:.6} V");
                }
            }

            // Write output.
            match format {
                Format::Nutmeg => write_nutmeg_dc(&out_path, &names, &values),
                Format::Parquet => write_parquet_dc(&out_path, &names, &values),
                Format::Vcd => write_vcd_dc(&out_path, &names, &values),
            }
            .map_err(|e| format!("write failed: {e}"))?;

            eprintln!("wrote {} -> '{}'", format_name(format), out_path.display());
        }

        Analysis::Tran => {
            let t_stop = tran_stop
                .as_deref()
                .map(parse_spice_value)
                .transpose()?
                .ok_or("--tran-stop is required for transient analysis")?;
            let h_step = tran_step
                .as_deref()
                .map(parse_spice_value)
                .transpose()?
                .unwrap_or(t_stop / 1000.0);

            let sol = run_tran(&vm, devices, t_stop, h_step)?;
            let elapsed = t0.elapsed();

            eprintln!(
                "Transient analysis complete: {} timepoints in {:.3} ms",
                sol.times.len(),
                elapsed.as_secs_f64() * 1e3
            );

            match format {
                Format::Nutmeg => write_nutmeg_tran(&out_path, &sol),
                Format::Parquet => write_parquet_tran(&out_path, &sol),
                Format::Vcd => write_vcd_tran(&out_path, &sol),
            }
            .map_err(|e| format!("write failed: {e}"))?;

            eprintln!("wrote {} -> '{}'", format_name(format), out_path.display());
        }

        Analysis::Ac => {
            return Err(
                "AC frequency sweep is not yet implemented in this version of circuit_solver_delta"
                    .into(),
            );
        }
    }

    Ok(())
}

fn format_name(f: Format) -> &'static str {
    match f {
        Format::Nutmeg => "nutmeg",
        Format::Vcd => "vcd",
        Format::Parquet => "parquet(csv)",
    }
}

// ── Smoke tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_spice_value_basic() {
        assert!((parse_spice_value("1k").unwrap() - 1000.0).abs() < 1e-10);
        assert!((parse_spice_value("1n").unwrap() - 1e-9).abs() < 1e-20);
        assert!((parse_spice_value("1u").unwrap() - 1e-6).abs() < 1e-16);
        assert!((parse_spice_value("1m").unwrap() - 1e-3).abs() < 1e-13);
        assert!((parse_spice_value("1MEG").unwrap() - 1e6).abs() < 1e-4);
        assert!((parse_spice_value("1.5").unwrap() - 1.5).abs() < 1e-10);
    }

    #[test]
    fn dc_simple_resistor_divider() {
        // V1=5V, R1=1kΩ (in→mid), R2=1kΩ (mid→0) → V(mid) = 2.5V
        let netlist = "test\nV1 in 0 5\nR1 in mid 1k\nR2 mid 0 1k\n";
        let (tokens, _, _) = tokenize(netlist);
        let (vm, devices) = build_circuit(&tokens).unwrap();
        let (_names, vals) = run_dc(&vm, &devices).unwrap();
        // Find "mid" index.
        let mid_idx = vm.node_index("mid").unwrap();
        // vals[0] is ground; vals[i] corresponds to var_map index i.
        let v_mid = vals[mid_idx];
        assert!(
            (v_mid - 2.5).abs() < 0.01,
            "V(mid) should be ~2.5 V, got {v_mid}"
        );
    }
}
