## ADDED Requirements

### Requirement: DC operating point analysis with node voltages and branch currents
The system SHALL run DC analysis by solving the steady-state nonlinear circuit equations via Newton-Raphson, outputting node voltages and branch currents (through inductors, voltage sources, and controlled sources) at the computed operating point, per [[computer-methods-circuit-analysis-design]] §12 and [[simulation-analog-mixed-signal-circuits]] §DC.

#### Scenario: DC output format and node voltage access
- **WHEN** DC analysis completes successfully at the operating point (NR converges to tol = 1e-6 V in <20 iterations)
- **THEN** the result object exposes a mapping of node names to DC voltages (e.g., node.v1 = 2.4567 V, node.gnd = 0 V by definition), all branch currents are available (e.g., current through V_dd source, through each inductor), and the total power dissipation is computed as a consistency check (ΣV×I over all sources)

#### Scenario: Operating point with parameter sweep
- **WHEN** the netlist specifies a DC sweep parameter (e.g., V_dd from 3.0 V to 5.0 V in 0.2 V steps)
- **THEN** the analyzer runs Newton-Raphson for each parameter value, outputs a matrix of results (rows = parameter values, columns = node voltages + key branch currents), and the result is exportable as a CSV or NumPy array for post-processing

### Requirement: AC small-signal analysis via linearization and frequency-domain transfer function computation
The system SHALL linearize the nonlinear circuit at the DC operating point, extract the small-signal MNA admittance matrix, and compute frequency-domain transfer functions (voltage/current gains, impedances) at user-specified frequencies, per [[computer-methods-circuit-analysis-design]] §15 and [[simulation-analog-mixed-signal-circuits]] §AC.

#### Scenario: AC transfer function at specified frequencies
- **WHEN** AC analysis is run with an AC input source (e.g., Vac = 1 V magnitude at node 'input') and specified output nodes (e.g., 'output', 'fb'), over a frequency sweep (10 Hz to 1 MHz, logarithmic with 10 points/decade)
- **THEN** the system outputs transfer functions H(f) = V_output / V_input at each frequency, expressed as magnitude and phase (e.g., |H| = 100 V/V = 40 dB, ∠H = −45°), the -3dB bandwidth is extractable from the magnitude curve, and poles/zeros are identifiable from the phase response

#### Scenario: AC impedance and pole-zero extraction
- **WHEN** a user requests input impedance Z_in(f) = V_input / I_input over a frequency range (1 kHz to 100 MHz)
- **THEN** the small-signal admittance matrix is inverted (via sparse LU factorization) at each frequency, the impedance is computed as Z_in = V / I, the magnitude and phase are output, resonances (impedance peaks) and anti-resonances (troughs) are clearly visible, and the user can extract Q factors and effective L/C from the impedance curve

### Requirement: Transient analysis with waveform sampling on internal timestep grid
The system SHALL run transient analysis via Radau IIA (or BDF) integration, sample all node voltages and key branch currents at the internal timestep grid (without interpolation), and output the waveforms for post-processing, per [[solving-ode-ii-stiff-dae]] and [[simulation-analog-mixed-signal-circuits]] §transient.

#### Scenario: Transient waveform data and timestep alignment
- **WHEN** transient analysis is run from t=0 to t=1 μs on a circuit with analog/digital mixed-signal activity (e.g., a PLL locking)
- **THEN** the integrator samples all node voltages and branch currents at every internal timestep (adaptive timestep, 100–10,000 samples depending on circuit stiffness), the timestep vector and corresponding voltage/current matrices are stored, temporal resolution is sufficient to resolve transients (e.g., digital slew transitions at sub-nanosecond dV/dt are captured with <1% error), and the user can plot any node voltage vs. time

#### Scenario: Transient convergence and error control
- **WHEN** the transient solver encounters a stiff system (e.g., fast digital switching, slow RC settling) and must adapt timestep size
- **THEN** the local truncation error (LTE) is monitored in the voltage domain (not charge), the timestep shrinks automatically when LTE exceeds the tolerance (default 1e-5 V), the solver avoids false convergence by maintaining KCL check, and dense output (if enabled) can interpolate waveforms at arbitrary times with bounded error

### Requirement: Waveform output in Nutmeg binary and Parquet columnar formats
The system SHALL support two output formats for transient/AC results: SPICE-compatible Nutmeg binary (.raw) and columnar Parquet (.parquet) for efficient storage and NumPy/pandas integration, per [[simulation-analog-mixed-signal-circuits]] and modern data science workflows.

#### Scenario: Nutmeg binary format compatibility
- **WHEN** a transient analysis completes and the user requests Nutmeg output format (filename.raw)
- **THEN** the system writes a SPICE-compatible .raw file with the following sections: header (title, date, plotname), vector declarations (node names, branch current names, frequency/time), and binary data (all samples in native float64), the file is readable by ngspice, PySpice, or custom post-processors via Nutmeg parsers, and waveform magnitudes match the internal simulation exactly

#### Scenario: Parquet columnar format with metadata
- **WHEN** a transient analysis completes and the user requests Parquet output (filename.parquet)
- **THEN** the system writes a columnar Parquet file where each column corresponds to one node voltage or branch current, the first column is time (monotonically increasing), column names match node/branch names from the netlist, metadata (circuit title, analysis type, timestamp, simulator version) is stored in Parquet file attributes, and the file is directly loadable into Python: `df = pd.read_parquet('sim.parquet')`

### Requirement: Python-callable result object via PyO3 or file output, compatible with pandas/NumPy
The system SHALL expose simulation results to Python via either in-process PyO3 bindings or a structured file interface, allowing users to access waveforms as pandas DataFrames or NumPy arrays for post-processing without external conversion tools, per [[python-data-science]] and modern Python data workflows.

#### Scenario: Direct Python integration via PyO3
- **WHEN** a user imports the Rust simulator as a Python module: `import circuit_solver_delta as csd`
- **THEN** the user can instantiate a circuit, run transient analysis, and access results directly: `result = circuit.transient(t_stop=1e-6); df = result.as_pandas(); v_out = df['output'].values` (numpy array), the result object supports indexing by node name, the timestep vector is accessible as `result.time`, and large waveforms (>1 GB) are streamed or memory-mapped if needed

#### Scenario: File-based result object with NumPy/pandas compatibility
- **WHEN** the simulator writes transient results to a structured HDF5 or Parquet file and the user loads them in Python: `result_file = 'sim_out.h5'`
- **THEN** the user loads the results via pandas or h5py: `df = pd.read_hdf5('sim_out.h5'); v_out = df['node_out']; current = df['I_vdd']`, all node voltages and currents are directly accessible as pandas Series/DataFrame, the time vector is a pandas Index, and downstream analysis (FFT, spectral estimation, noise metrics) uses standard NumPy/SciPy functions without custom interface code

#### Scenario: Waveform slicing and time-based indexing
- **WHEN** the result object (either PyO3-bound or file-loaded) is indexed by time range: `result[0.5e-6:1.0e-6]` or `df.loc[0.5e-6:1.0e-6]`
- **THEN** the user obtains a subset of waveforms for the specified time interval, the subset is a valid result/DataFrame that can be re-exported or plotted, and slicing operations are efficient (no full-file reloads)
