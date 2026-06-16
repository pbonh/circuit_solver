## ADDED Requirements

### Requirement: Analog-to-digital threshold crossing detection and discrete event injection
The system SHALL monitor analog signal trajectories during transient integration and detect threshold crossings on monitored nodes, injecting a discrete event into the digital event scheduler upon crossing, per [[verilog-ams]] and [[simulation-analog-mixed-signal-circuits]] §Mixed-Level.

#### Scenario: Threshold crossing generates digital event
- **WHEN** a transient integration step traverses a user-specified threshold on an analog node (e.g., a comparator output signal crossing 0.5 V from below)
- **THEN** the analog solver detects the crossing (via interpolation or step-back refinement), emits an event with (timestamp, node_id, threshold, direction) to the digital scheduler, and the digital scheduler immediately processes any pending digital logic that depends on this threshold event

#### Scenario: Multiple threshold monitoring on one node
- **WHEN** a node is monitored for multiple thresholds (e.g., V > 0.3 V, V > 0.7 V, V < 0.1 V)
- **THEN** the crossing detector tracks all thresholds independently, emits events only on actual crossings (not false positives), and maintains the correct temporal order of events if multiple thresholds are crossed in a single timestep

### Requirement: Digital waveform event injection and piecewise-linear (transition/slew) analog waveform generation
The system SHALL accept digital transition events (discrete time-stamped logic changes) from the digital scheduler and inject them into the analog domain as piecewise-linear waveforms (via `transition` and `slew` operators), per [[verilog-ams]] §5.3 and [[simulation-analog-mixed-signal-circuits]] §behavioral-models.

#### Scenario: Digital event drives analog transition waveform
- **WHEN** a digital output transitions from logic-0 (0.0 V) to logic-1 (3.3 V) with a specified slew time (e.g., 100 ps)
- **THEN** the analog domain receives a piecewise-linear waveform: V(t) = 0 for t < t_start, V(t) = (3.3 V) × (t − t_start) / t_slew for t_start ≤ t ≤ t_start + t_slew, and V(t) = 3.3 V for t > t_start + t_slew, with continuous first derivative (no kinks in dV/dt)

#### Scenario: Slew-limited digital-to-analog waveform in oscillator driver
- **WHEN** a digital oscillator clock output (1 kHz, 50% duty) drives the base of a BJT amplifier stage through a `slew` function with slew rate 2 V/ns
- **THEN** the resulting BJT base voltage exhibits piecewise-linear rise/fall with the specified slew rate, the BJT collector response is computed via transient integration of the amplifier stage, and the waveform bandwidth is limited by the slew rate (no aliasing or spurious high-frequency components)

### Requirement: Analog timestep synchronization to next pending digital event time
The system SHALL synchronize the analog transient integrator's internal timestep grid to align with scheduled digital event times, ensuring events are processed at exact clock boundaries without interpolation error, per [[devs-simulation]] and [[simulation-analog-mixed-signal-circuits]] §discrete-continuous.

#### Scenario: Analog timestep aligns to digital event time
- **WHEN** the analog integrator is advancing through a transient (current time t_a = 10.5 ns) and the next digital event is scheduled at t_d = 10.7 ns
- **THEN** the analog integrator computes the next internal step size such that the step lands exactly at t_d (or slightly before), completes integration to t_d, halts, and signals the digital scheduler to process events at t_d before resuming analog integration

#### Scenario: Dense digital clocking with sparse analog updates
- **WHEN** the digital circuit has a high-frequency clock (10 GHz) but only a few analog nodes interact with it (e.g., via `transition` on the clock, fed to a phase-locked loop)
- **THEN** the analog integrator adapts its timestep: it uses fine steps when near discontinuities (transition edges), coarser steps when the analog circuit exhibits low activity (slow exponential settling), and always respects the next digital event time without overshooting

### Requirement: Behavioral analog blocks (Verilog-AMS operators as behavioral device models)
The system SHALL support a subset of Verilog-AMS analog block operators (`idt`, `ddt`, `transition`, `slew`) as behavioral device models in the MNA stamping layer, allowing behavioral macromodels to be instantiated alongside netlist devices, per [[verilog-ams]] §5 and [[simulation-analog-mixed-signal-circuits]] §behavioral-models.

#### Scenario: Behavioral integrator (idt) in a phase-locked loop
- **WHEN** a Verilog-AMS behavioral block uses the `idt` operator to model a phase accumulator: `y = idt(x)`
- **THEN** the operator is stamped into the MNA as an auxiliary branch variable (the integral state), the relationship y = ∫x dt is enforced in the MNA system, and transient integration correctly evolves the integral state via the stiff integrator (BDF or Radau), maintaining charge conservation

#### Scenario: Behavioral derivative (ddt) in a feedback loop
- **WHEN** a behavioral block uses `ddt` to compute the time derivative of a signal: `y = ddt(x)`
- **THEN** the operator computes a finite-difference approximation (or explicit time-derivative formula from the integrator state), the result exhibits the correct dimensionality (dV/dt with appropriate time units), and the feedback loop into which ddt feeds remains stable (no artificial oscillation due to derivative approximation)

### Requirement: VCO modeled as Verilog-AMS behavioral block with digital frequency counter co-simulation
The system SHALL correctly simulate a voltage-controlled oscillator (VCO) described entirely as a behavioral analog block (integrator-based phase accumulator with oscillating output) driving a digital frequency counter (event-based counter triggered on rising edges), demonstrating analog-digital co-simulation of a PLL subsystem, per [[verilog-ams]] and [[simulation-analog-mixed-signal-circuits]] §PLL.

#### Scenario: VCO-counter co-simulation and frequency locking
- **WHEN** a transient analysis exercises a VCO-counter system: the VCO receives a control voltage (Vctrl) and outputs a periodic oscillation; the digital counter increments on rising edges and outputs a digital frequency estimate (updated every N edges)
- **THEN** the analog solver integrates the VCO's phase accumulator, generates rising-edge threshold events (crossing 0.5 V), injects events into the digital counter, the counter increments, the frequency estimate is fed back to analog as Vctrl via a DAC, the system exhibits the expected PLL dynamics (frequency pulls toward the setpoint), and the final steady-state oscillation frequency matches the control voltage (within control linearity)

#### Scenario: VCO period and phase noise characterization
- **WHEN** the transient analysis completes and post-processing extracts timestamps of all rising-edge threshold crossings from the VCO output
- **THEN** the instantaneous period (time difference between consecutive edges) exhibits the expected voltage-to-frequency conversion (VCO gain K_vco in Hz/V), the long-term frequency average locks to the setpoint within the PLL bandwidth, and phase jitter (deviation of edge times from ideal periodic spacing) is quantifiable for noise analysis
