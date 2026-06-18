//! Minimal VCD (Value Change Dump) writer for transient simulation results.
//!
//! Writes IEEE 1364-style VCD with `$var real 64` declarations and
//! picosecond-resolution timestamps.  Suitable for viewing in GTKWave or
//! parsing with the `vcd` crate.
//!
//! # Example
//!
//! ```no_run
//! use circuit_solver_delta::vcd_writer::write_vcd;
//! use circuit_solver_delta::transient::TransientSolution;
//! use std::collections::HashMap;
//!
//! let mut waveforms = HashMap::new();
//! waveforms.insert("out".to_string(), vec![0.0_f64, 0.9, 1.8]);
//! let sol = TransientSolution {
//!     times: vec![0.0, 5e-9, 10e-9],
//!     waveforms,
//! };
//! // write_vcd("/tmp/test.vcd", &sol).unwrap();
//! ```

use std::{fs, io::Write, path::Path};

use crate::transient::TransientSolution;

/// Write a [`TransientSolution`] to a VCD file at `path`.
///
/// Signal names are sorted alphabetically.  Each time step is written at
/// picosecond granularity.
pub fn write_vcd(path: impl AsRef<Path>, sol: &TransientSolution) -> std::io::Result<()> {
    let path = path.as_ref();
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
        let t_ps = (t * 1e12).round() as u64;
        writeln!(out, "#{t_ps}")?;
        for (i, k) in keys.iter().enumerate() {
            if let Some(buf) = sol.waveforms.get(k) {
                let v = buf.get(pt).copied().unwrap_or(0.0);
                writeln!(out, "r{v:.6e} v{i}")?;
            }
        }
    }

    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    /// Write a 2-signal VCD and verify the file is parseable (non-empty,
    /// contains expected tokens).
    #[test]
    fn vcd_file_is_parseable() {
        let mut waveforms = HashMap::new();
        waveforms.insert("out".to_string(), vec![0.0_f64, 0.9, 1.8]);
        waveforms.insert("mid".to_string(), vec![1.8_f64, 0.9, 0.0]);
        let sol = TransientSolution {
            times: vec![0.0, 5e-9, 10e-9],
            waveforms,
        };

        let dir = std::env::temp_dir();
        let path = dir.join("test_vcd_parseable.vcd");
        write_vcd(&path, &sol).expect("write_vcd should succeed");

        let content = std::fs::read_to_string(&path).expect("should read file");
        assert!(content.contains("$timescale 1ps $end"), "missing timescale");
        assert!(content.contains("$enddefinitions $end"), "missing enddefinitions");
        assert!(content.contains("$var real 64"), "missing var declaration");
        assert!(content.contains("#0"), "missing t=0 timestamp");
        assert!(content.contains("#5000"), "missing t=5ns timestamp (5000 ps)");
        assert!(content.contains("#10000"), "missing t=10ns timestamp (10000 ps)");

        // Clean up.
        let _ = std::fs::remove_file(&path);
    }
}
