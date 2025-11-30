# Page 1

## 1 FEATURES

- Provides year, month, day, weekday, hours, minutes, and seconds based on a 32.768 kHz quartz crystal
- Century flag
- Clock operating voltage: 1.2 V to 5.5 V at room temperature
- Low backup current; typical $0.5 \mu \mathrm{~A}$ at $\mathrm{V}_{\mathrm{DD}}=3.0 \mathrm{~V}$ and $\mathrm{T}_{\text {amb }}=25^{\circ} \mathrm{C}$
- 400 kHz two-wire $\mathrm{I}^{2} \mathrm{C}$-bus interface (at $\mathrm{V}_{\mathrm{DD}}=1.8 \mathrm{~V}$ to 5.5 V )
- Programmable clock output for peripheral devices ( $32.768 \mathrm{kHz}, 1.024 \mathrm{kHz}, 32 \mathrm{~Hz}$, and 1 Hz )
- Alarm and timer functions
- Integrated oscillator capacitor
- Internal Power-On Reset (POR)
- $\mathrm{I}^{2} \mathrm{C}$-bus slave address: read A3h and write A2h
- Open-drain interrupt pin


## 2 APPLICATIONS

- Mobile telephones Utility Meters
- Portable instruments
- Electronic metering
- Battery powered products


## 3 DESCRIPTION

The BM8563 is a CMOS Real-Time Clock (RTC) and calendar optimized for low power consumption. A programmable clock output, interrupt output, and voltage-low detector are also provided. All addresses and data are transferred serially via a two-line bidirectional $I^{2} C$-bus. Maximum bus speed is $400 \mathrm{kbit} / \mathrm{s}$. The register address is incremented automatically after each written or read data byte.

BM8563 Block Diagram
![img-0.jpeg](img-0.jpeg)

# Page 2

# 4 Pin Configuration and Functions 

![img-1.jpeg](img-1.jpeg)

BM8563

Pin Functions

| PIN | Name | FUNCTION |
| :--: | :--: | :--: |
| 1 | OSCI | oscillator input |
| 2 | OSCO | oscillator output |
| 3 | $\overline{\mathrm{INT}}$ | interrupt output (open-drain; active LOW) |
| 4 | VSS | ground |
| 5 | SDA | serial data input and output |
| 6 | SCL | serial clock input |
| 7 | CLKOUT | clock output, open-drain |
| 8 | VDD | supply voltage |

## 5 Specifications

### 5.1 Limiting values

| Symbol | Parameter | Conditions | Min | Max | Unit |
| :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{V}_{\text {DD }}$ | supply voltage |  | $-0.5$ | $+6.5$ | V |
| $I_{D D}$ | supply current |  | $-50$ | $+50$ | mA |
| $V_{I}$ | input voltage | on pins SCL, SDA, and OSCI | $-0.5$ | $+6.5$ | V |
| $V_{O}$ | Output voltage | on pins CLKOUT and $\overline{\mathrm{INT}}$ | $-0.5$ | $+6.5$ | V |
| $I_{I}$ | input current | at any input | $-10$ | $+10$ | mA |
| $I_{D}$ | output current | at any output | $-10$ | $+10$ | mA |
| $P_{\text {tot }}$ | total power dissipation |  | - | 300 | mW |
| $V_{\text {ESD }}$ | electrostatic discharge voltage | HBM ${ }^{[1]}$ | - | $\pm 4000$ | V |
|  |  | CDM ${ }^{[2]}$ | - | $\pm 1000$ | V |
| $I_{I u}$ | latch-up current | ${ }^{[3]}$ | - | 200 | mA |
| $T_{\text {stg }}$ | storage temperature |  | $-55$ | 125 | ${ }^{\circ} \mathrm{C}$ |
| $T_{\text {amb }}$ | ambient temperature | operating device | $-40$ | $+85$ | ${ }^{\circ} \mathrm{C}$ |

[1] Pass level; Human Body Model (HBM), according to "JESD22-A114".
[2] Pass level; Charged-Device Model (CDM), according to "JESD22-C101" .
[3] Pass level; latch-up testing according to "JESD78" at maximum ambient temperature ( $\mathrm{T}_{\text {amb(max) }}$ ).

# Page 3

# 5.2 Static Characteristics 

$\left(\mathrm{V}_{\mathrm{DD}}=1.8 \mathrm{~V}\right.$ to $\left.5.5 \mathrm{~V} ; \mathrm{V}_{\mathrm{SS}}=0 \mathrm{~V} ; \mathrm{T}_{\mathrm{amb}}=-40^{\circ} \mathrm{C}\right.$ to $\left.+85^{\circ} \mathrm{C} ; \mathrm{f}_{\mathrm{occ}}=32.768 \mathrm{kHz} ;$ quartz $\mathrm{R}_{\mathrm{s}}=40 \mathrm{k} \Omega ; \mathrm{C}_{\mathrm{L}}=8 \mathrm{pF} ;\right.$ unless otherwise specified.)

| SYMBOL | PARAMETER | CONDITIONS | MIN | TYP | MAX | UNITS |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Supplies |  |  |  |  |  |  |
| $V_{D D}$ | supply voltage | interface inactive; $\mathrm{f}_{\mathrm{SCL}}=0 \mathrm{~Hz} ; \mathrm{T}_{\mathrm{amb}}=25^{\circ} \mathrm{C}^{[1]}$ | 1.2 | - | 5.5 | V |
|  |  | interface active; $\mathrm{f}_{\mathrm{SCL}}=400 \mathrm{kHz}^{[1]}$ | 1.8 | - | 5.5 |  |
|  |  | clock data integrity; $\mathrm{T}_{\mathrm{amb}}=25^{\circ} \mathrm{C}$ | 1.2 | - | 5.5 |  |
| $I_{D D}$ | supply current | interface active |  |  |  |  |
|  |  | $\mathrm{f}_{\mathrm{SCL}}=400 \mathrm{kHz}$ | - | - | 800 | $\mu \mathrm{A}$ |
|  |  | $\mathrm{f}_{\mathrm{SCL}}=100 \mathrm{kHz}$ | - | - | 200 | $\mu \mathrm{A}$ |
|  |  | interface inactive ( $\mathrm{f}_{\mathrm{SCL}}=0 \mathrm{~Hz}$ ); CLKOUT disabled; $\mathrm{T}_{\mathrm{amb}}=25^{\circ} \mathrm{C}^{[2]}$ |  |  |  |  |
|  |  | $\mathrm{V}_{\mathrm{DD}}=5.0 \mathrm{~V}$ | - | 600 | 800 | nA |
|  |  | $\mathrm{V}_{\mathrm{DD}}=3.0 \mathrm{~V}$ | - | 500 | 650 | nA |
|  |  | $\mathrm{V}_{\mathrm{DD}}=2.0 \mathrm{~V}$ | - | 500 | 600 | nA |
|  |  | interface inactive ( $\mathrm{f}_{\mathrm{SCL}}=0 \mathrm{~Hz}$ ); CLKOUT disabled; $\mathrm{T}_{\mathrm{amb}}=-40^{\circ} \mathrm{C}$ to $+85^{\circ} \mathrm{C}^{[2]}$ |  |  |  |  |
|  |  | $\mathrm{V}_{\mathrm{DD}}=5.0 \mathrm{~V}$ | - | 800 | 950 | nA |
|  |  | $\mathrm{V}_{\mathrm{DD}}=3.0 \mathrm{~V}$ | - | 700 | 900 | nA |
|  |  | $\mathrm{V}_{\mathrm{DD}}=2.0 \mathrm{~V}$ | - | 700 | 850 | nA |
|  |  | interface inactive ( $\mathrm{f}_{\mathrm{SCL}}=0 \mathrm{~Hz}$ ); CLKOUT enabled at $32 \mathrm{kHz} ; \mathrm{T}_{\mathrm{amb}}=25^{\circ} \mathrm{C}^{[2]}$ |  |  |  |  |
|  |  | $\mathrm{V}_{\mathrm{DD}}=5.0 \mathrm{~V}$ | - | 900 | 1600 | nA |
|  |  | $\mathrm{V}_{\mathrm{DD}}=3.0 \mathrm{~V}$ | - | 800 | 1000 | nA |
|  |  | $\mathrm{V}_{\mathrm{DD}}=2.0 \mathrm{~V}$ | - | 750 | 800 | nA |
|  |  | interface inactive ( $\mathrm{f}_{\mathrm{SCL}}=0 \mathrm{~Hz}$ ); CLKOUT enabled at $32 \mathrm{kHz} ; \mathrm{T}_{\mathrm{amb}}=-40^{\circ} \mathrm{C}$ to $+85^{\circ} \mathrm{C}^{[2]}$ |  |  |  |  |
|  |  | $\mathrm{V}_{\mathrm{DD}}=5.0 \mathrm{~V}$ | - | 1100 | 1700 | nA |
|  |  | $\mathrm{V}_{\mathrm{DD}}=3.0 \mathrm{~V}$ | - | 900 | 1100 | nA |
|  |  | $\mathrm{V}_{\mathrm{DD}}=2.0 \mathrm{~V}$ | - | 850 | 900 | nA |
| Inputs |  |  |  |  |  |  |
| $\mathrm{V}_{\mathrm{R}}$ | LOW-level input voltage |  | $\mathrm{V}_{\mathrm{SS}}$ | - | $0.3 \mathrm{~V}_{\mathrm{DD}}$ | V |
| $\mathrm{V}_{\mathrm{HI}}$ | HIGH-level input voltage |  | $0.7 \mathrm{~V}_{\mathrm{DD}}$ | - | $\mathrm{V}_{\mathrm{DD}}$ | V |
| $I_{U}$ | input leakage current |  | $-1$ | 0 | $+1$ | $\mu \mathrm{A}$ |
| $C_{I}$ | input capacitance | $\mathrm{V}_{\mathrm{I}}=\mathrm{V}_{\mathrm{DD}}$ or $\mathrm{V}_{\mathrm{SS}}{ }^{[3]}$ | - | - | 7 | pF |

# Page 4

# Static Characteristics(continued) 

$\left(V_{D D}=1.8 \mathrm{~V}\right.$ to $5.5 \mathrm{~V} ; V_{S S}=0 \mathrm{~V} ; T_{\text {amb }}=-40^{\circ} \mathrm{C}$ to $+85^{\circ} \mathrm{C} ; f_{\text {osc }}=32.768 \mathrm{kHz} ;$ quartz $\mathrm{R}_{\mathrm{s}}=40 \mathrm{k} \Omega ; \mathrm{C}_{\mathrm{L}}=8 \mathrm{pF} ;$ unless otherwise specified.)

## Outputs

| $\mathrm{I}_{\mathrm{OL}}$ | LOW-level output current | output sink current; $\mathrm{V}_{\mathrm{OL}}=0.4 \mathrm{~V} ; \mathrm{V}_{\mathrm{DD}}=5 \mathrm{~V}$ |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | on pin SDA | 3 | - | - | mA |
|  |  | on pin INT | 1 | - | - | mA |
|  |  | on pin CLKOUT | 1 | - | - | mA |
| $\mathrm{I}_{\mathrm{LO}}$ | output leakage current | $\mathrm{V}_{\mathrm{O}}=\mathrm{V}_{\mathrm{DD}}$ or $\mathrm{V}_{\mathrm{SS}}$ | $-1$ | 0 | $+1$ | $\mu \mathrm{~A}$ |
| Voltage detector |  |  |  |  |  |  |
| $\mathrm{V}_{\text {low }}$ | low voltage | $\mathrm{T}_{\text {amb }}=25^{\circ} \mathrm{C}$; sets bit VL; see Figure 7 | - | 0.5 | 0.6 | V |

[1] For reliable oscillator start-up at power-up: $\mathrm{V}_{\text {DD(min) power-up }}=\mathrm{V}_{\text {DD(min) }}+0.3 \mathrm{~V}$.
[2] Timer source clock $=1 / 60 \mathrm{~Hz}$, level of pins SCL and SDA is $\mathrm{V}_{\mathrm{DD}}$ or $\mathrm{V}_{\mathrm{SS}}$.
[3] Tested on sample basis.

### 5.3 Dynamic Characteristics

$\left(V_{D D}=1.8 \mathrm{~V}\right.$ to $5.5 \mathrm{~V} ; V_{S S}=0 \mathrm{~V} ; T_{\text {amb }}=-40^{\circ} \mathrm{C}$ to $+85^{\circ} \mathrm{C} ; f_{\text {osc }}=32.768 \mathrm{kHz} ;$ quartz $\mathrm{R}_{\mathrm{s}}=40 \mathrm{k} \Omega ; \mathrm{C}_{\mathrm{L}}=8 \mathrm{pF} ;$ unless otherwise specified. $]$

| SYMBOL | PARAMETER | CONDITIONS | MIN | TYP | MAX | UNITS |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Oscillator |  |  |  |  |  |  |
| $\mathrm{C}_{\text {OSCO }}$ | capacitance on pin OSCO |  | 15 | 25 | 35 | pF |
| $\Delta \mathrm{f}_{\text {osc }} / \mathrm{f}_{\text {osc }}$ | relative oscillator frequency variation | $\Delta \mathrm{V}_{\mathrm{DD}}=200 \mathrm{mV} ; \mathrm{T}_{\text {amb }}=25^{\circ} \mathrm{C}$ | - | 0.2 | - | ppm |
| Quartz crystal parameters ( $\mathbf{f = 3 2 . 7 6 8} \mathbf{~ k H z}$ ) |  |  |  |  |  |  |
| $R_{s}$ | series resistance |  | - | - | 100 | $\mathrm{k} \Omega$ |
| $C_{L}$ | load capacitance | Parallel ${ }^{[1]}$ | 7 | - | 12.5 | pF |
| $\mathrm{C}_{\text {trim }}$ | trimmer capacitance | external;on pin OSCI | 5 | - | 25 | pF |
| CLKOUT output |  |  |  |  |  |  |
| $\delta_{\text {CLKOUT }}$ | duty cycle on pin CLKOUT | ${ }^{[2]}$ | - | 50 | - | \% |

## $\mathrm{I}^{2} \mathrm{C}$-bus timing characteristics (see Figure 1) ${ }^{[3][4]}$

| $f_{\text {SCL }}$ | SCL clock frequency | ${ }^{[5]}$ | - | - | 400 | kHz |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $t_{\text {HSS }} \text { STA }$ | hold time (repeated) START condition |  | 0.6 | - | - | $\mu \mathrm{s}$ |
| $t_{\text {SLL }}$ STA | set-up time for a repeated START condition |  | 0.6 | - | - | $\mu \mathrm{s}$ |
| $t_{\text {LOW }}$ | LOW period of the SCL clock |  | 1.3 | - | - | $\mu \mathrm{s}$ |
| $t_{\text {HIGH }}$ | HIGH period of the SCL clock |  | 0.6 | - | - | $\mu \mathrm{s}$ |
| $t_{r}$ | rise time of both SDA and SCL signals | standard-mode | - | - | 1 | $\mu \mathrm{s}$ |
|  |  | fast-mode | - | - | 0.3 | $\mu \mathrm{s}$ |

# Page 5

Dynamic Characteristics(continued)
$\left(V_{D D}=1.8 \mathrm{~V}\right.$ to $\left.5.5 \mathrm{~V} ; V_{S S}=0 \mathrm{~V} ; T_{\text {amb }}=-40^{\circ} \mathrm{C}\right.$ to $\left.+85^{\circ} \mathrm{C} ; f_{\text {osc }}=32.768 \mathrm{kHz} ;$ quartz $\mathrm{R}_{\mathrm{s}}=40 \mathrm{k} \Omega ; C_{\mathrm{L}}=8 \mathrm{pF} ;$ unless otherwise specified. $)$

| $t_{f}$ | fall time of both SDA <br> and SCL signals |  | - | - | 0.3 | $\mu \mathrm{s}$ |
| :--: | :-- | :-- | :--: | :--: | :--: | :--: |
| $t_{\text {BUF }}$ | bus free time between <br> a STOP and START <br> condition |  | 1.3 | - | - | $\mu \mathrm{s}$ |
| $C_{b}$ | capacitive load for <br> each bus line |  | - | - | 400 | pF |
| $t_{S U} ;$ DAT | data set-up time |  | 100 | - | - | ns |
| $t_{\text {HD }} ;$ DAT | data hold time |  | 0 | - | - | ns |
| $t_{S U} ;$ STO | set-up time for STOP <br> condition |  | 0.6 | - | - | $\mu \mathrm{s}$ |
| $t_{\text {w(splie) }}$ | spike pulse width | on bus | - | - | 50 | ns |

[1] $C_{L}$ is a calculation of $C_{\text {trim }}$ and $C_{\text {OSCO }}$ in series: $C_{L}=\frac{\left(C_{\text {trim}}+C_{\text {OSCO }}\right)}{\left(C_{\text {trim }}+C_{\text {OSCO }}\right)}$.
[2] Unspecified for $f_{\text {CUGUT }}=32.768 \mathrm{kHz}$.
[3] All timing values are valid within the operating supply voltage at ambient temperature and referenced to $\mathrm{V}_{\mathrm{IL}}$ and $\mathrm{V}_{\mathrm{IN}}$ with an input voltage swing of $\mathrm{V}_{\mathrm{SS}}$ to $\mathrm{V}_{\mathrm{DD}}$.
[4] I²C-bus access time between two STARTs or between a START and a STOP condition to this device must be less than one second.

# 5.4 Typical Characteristics 

![img-2.jpeg](img-2.jpeg)

Fig 1. I²C-bus timing waveforms

# Page 6

# 6 Functional Description 

The BM8563 contains sixteen 8-bit registers with an auto-incrementing register address, an on-chip 32.768 kHz oscillator with one integrated capacitor, a frequency divider which provides the source clock for the Real-Time Clock (RTC) and calendar, a programmable clock output, a timer, an alarm, a voltage-low detector, and a $400 \mathrm{kHz} \mathrm{I}^{2} \mathrm{C}$-bus interface.
All 16 registers are designed as addressable 8-bit parallel registers although not all bits are implemented. The first two registers (memory address 00 h and 01 h ) are used as control and/or status registers. The memory addresses 02 h through 08 h are used as counters for the clock function (seconds up to years counters). Address locations 09 h through 0 Ch contain alarm registers which define the conditions for an alarm. Address 0 Dh controls the CLKOUT output frequency. OEh and OFh are the Timer control and Timer registers, respectively.
The Seconds, Minutes, Hours, Days, Months, Years as well as the Minute_alarm, Hour_alarm, and Day_alarm registers are all coded in Binary Coded Decimal (BCD) format.
When one of the RTC registers is written or read, the contents of all time counters are frozen. Therefore, faulty writing or reading of the clock and calendar during a carry condition is prevented.

### 6.1 CLKOUT output

A programmable square wave is available at the CLKOUT pin. Operation is controlled by the register CLKOUT control at address 0Dh. Frequencies of 32.768 kHz (default), 1.024 kHz , 32 Hz , and 1 Hz can be generated for use as a system clock, microcontroller clock, input to a charge pump, or for calibration of the oscillator. CLKOUT is an open drain output and enabled at power-on. If disabled it becomes high-impedance.

### 6.2 Register organization

Table 1. Formatted registers overview
Bit positions labelled as $x$ are not relevant. Bit positions labelled with N should always be written with logic 0 ; if read they could be either logic 0 or logic 1 . After reset, all registers are set according to Table 24.

| Address | Register name | Bit |  |  |  |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| Control and status registers |  |  |  |  |  |  |  |  |  |
| 00h | Control_status_1 | TEST1 | N | STOP | N | TESTC | N | N | N |
| 01h | Control_status_2 | N | N | N | Tl_TP | AF | TF | AIE | TIE |
| Time and date registers |  |  |  |  |  |  |  |  |  |
| 02h | VL_seconds | VL | SECONDS (0 to 59) |  |  |  |  |  |  |
| 03h | Minutes | $x$ | MINUTES (0 to 59) |  |  |  |  |  |  |
| 04h | Hours | $x$ | $x$ | HOURS (0 to 23) |  |  |  |  |  |
| 05h | Days | $x$ | $x$ | DAYS (1 to 31) |  |  |  |  |  |
| 06h | Weekdays | $x$ | $x$ | $x$ | $x$ | $x$ | WEEKDAYS (0 to 6) |  |  |
| 07h | Century_months | C | $x$ | $x$ | MONTHS (1 to 12) |  |  |  |  |
| 08h | Years | YEARS (0 to 99) |  |  |  |  |  |  |  |
| Alarm registers |  |  |  |  |  |  |  |  |  |
| 09h | Minute_alarm | AE_M | MINUTE_ALARM (0 to 59) |  |  |  |  |  |  |
| 0Ah | Hour_alarm | AE_H | $x$ | HOUR_ALARM (0 to 23) |  |  |  |  |  |
| 0Bh | Day_alarm | AE_D | $x$ | DAY_ALARM (1 to 31) |  |  |  |  |  |
| 0Ch | Weekday_alarm | AE_W | $x$ | $x$ | $x$ | $x$ | WEEKDAY_ALARM (0 to 6) |  |  |
| CLKOUT control register |  |  |  |  |  |  |  |  |  |
| 0Dh | CLKOUT_control | FE | $x$ | $x$ | $x$ | $x$ | $x$ | FD[1:0] |  |
| Timer registers |  |  |  |  |  |  |  |  |  |
| 0Eh | Timer_control | TE | $x$ | $x$ | $x$ | $x$ | $x$ | TD[1:0] |  |
| OFh | Timer | TIMER[7:0] |  |  |  |  |  |  |  |

# Page 7

# 6.3 Control registers 

### 6.3.1 Register Control_status_1

Table 2. Control_status_1 - control and status register 1 (address 00 h ) bit description

| Bit | Symbol | Value | Description | Reference |
| :--: | :--: | :--: | :--: | :--: |
| 7 | TEST1 | $0^{[1]}$ | normal mode <br> must be set to logic 0 during normal operations | Section 6.9 |
|  |  | 1 | EXT_CLK test mode |  |
| 6 | N | $0^{[2]}$ | unused |  |
| 5 | STOP | $0^{[1]}$ | RTC source clock runs | Section 6.10 |
|  |  | 1 | all RTC divider chain flip-flops are asynchronously set to logic 0 ; the RTC clock is stopped (CLKOUT at 32.768 kHz is still available) |  |
| 4 | N | $0^{[2]}$ | unused |  |
| 3 | TESTC | 0 | Power-On Reset (POR) override facility is disabled; set to logic 0 for normal operation | Section 6.11.1 |
|  |  | $1^{[1]}$ | Power-On Reset (POR) override may be enabled |  |
| 2 to 0 | N | $000^{[2]}$ | unused |  |

[1] Default value.
[2] Bits labeled as N should always be written with logic 0 .

### 6.3.2 Register Control_status_2

Table 3. Control_status_2 - control and status register 2 (address 01h) bit description

| Bit | Symbol | Value | Description | Reference |
| :--: | :--: | :--: | :--: | :--: |
| 7 to 5 | N | $000^{[1]}$ | unused |  |
| 4 | TI_TP | $0^{[2]}$ | $\overline{\text { INT }}$ is active when TF is active (subject to the status of TIE) | Section 6.3.2.1 <br> And <br> Section 6.8 |
|  |  | 1 | $\overline{\text { INT }}$ pulses active according to Table 4 (subject to the status of TIE); Remark: note that if AF and AIE are active then $\overline{\text { INT }}$ will be permanently active |  |
| 3 | AF | $0^{[2]}$ | read: alarm flag inactive | Section 6.3.2.1 |
|  |  |  | write: alarm flag is cleared |  |
|  |  | 1 | read: alarm flag active |  |
|  |  |  | write: alarm flag remains unchanged |  |
| 2 | TF | $0^{[2]}$ | read: timer flag inactive |  |
|  |  |  | write: timer flag is cleared |  |
|  |  | 1 | read: timer flag active |  |
|  |  |  | write: timer flag remains unchanged |  |
| 1 | AIE | $0^{[2]}$ | alarm interrupt disabled |  |
|  |  | 1 | alarm interrupt enabled |  |
| 0 | TIE | $0^{[2]}$ | timer interrupt disabled |  |
|  |  | 1 | timer interrupt enabled |  |

[1] Bits labeled as N should always be written with logic 0.
[2] Default value.

# Page 8

# 6.3.2.1 Interrupt output 

Bits TF and AF: When an alarm occurs, AF is set to logic 1. Similarly, at the end of a timer countdown, TF is set to logic 1. These bits maintain their value until overwritten using the interface. If both timer and alarm interrupts are required in the application, the source of the interrupt can be determined by reading these bits. To prevent one flag being overwritten while clearing another, a logic AND is performed during a write access.
![img-3.jpeg](img-3.jpeg)

When bits TIE and AIE are disabled, pin INT will remain high-impedance.
Fig 6. Interrupt scheme
Bits TIE and AIE: These bits activate or deactivate the generation of an interrupt when TF or AF is asserted, respectively. The interrupt is the logical OR of these two conditions when both AIE and TIE are set.
Countdown timer interrupts: The pulse generator for the countdown timer interrupt uses an internal clock and is dependent on the selected source clock for the countdown timer and on the countdown value $n$. As a consequence, the width of the interrupt pulse varies (see Table 4).

Table 4. $\overline{\mathrm{INT}}$ operation (bit $\mathrm{TI}_{-} \mathrm{TP}=1$ ) ${ }^{[1]}$

| Source clock (Hz) | $\overline{\text { INT }}$ period (s) |  |
| :--: | :--: | :--: |
|  | $\mathrm{n}=1^{[2]}$ | $\mathrm{n}>1^{[2]}$ |
| 4096 | $1 / 8192$ | $1 / 4096$ |
| 64 | $1 / 128$ | $1 / 64$ |
| 1 | $1 / 64$ | $1 / 64$ |
| $1 / 60$ | $1 / 64$ | $1 / 64$ |

[1] TF and INT become active simultaneously. Default value.
[2] $\mathrm{n}=$ loaded countdown value. Timer stops when $\mathrm{n}=0$.

### 6.4 Time and date registers

The majority of the registers are coded in the BCD format to simplify application use.

### 6.4.1 Register VL_seconds

Table 5. VL_seconds - seconds and clock integrity status register (address 02h) bit description

| Bit | Symbol | Value | Place value | Description |
| :--: | :--: | :--: | :--: | :--: |
| 7 | VL | 0 | - | clock integrity is guaranteed |
|  |  | $1^{[1]}$ | - | integrity of the clock information is not guaranteed |
| 6 to 4 | SECONDS | 0 to 5 | ten's place | actual seconds coded in BCD format, see Table 6 |
| 3 to 0 |  | 0 to 9 | unit place |  |

[1] Start-up value.

# Page 9

Table 6. Seconds coded in BCD format

| Seconds value <br> (decimal) | Upper-digit (ten's place) |  |  | Digit (unit place) |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  | Bit 6 | Bit 5 | Bit 4 | Bit 3 | Bit 2 | Bit 1 | Bit 0 |
| 00 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 01 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| 02 | 0 | 0 | 0 | 0 | 0 | 1 | 0 |
| $:$ | $:$ | $:$ | $:$ | $:$ | $:$ | $:$ | $:$ |
| 09 | 0 | 0 | 0 | 1 | 0 | 0 | 1 |
| 10 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| $:$ | $:$ | $:$ | $:$ | $:$ | $:$ | $:$ | $:$ |
| 58 | 1 | 0 | 1 | 1 | 0 | 0 | 0 |
| 59 | 1 | 0 | 1 | 1 | 0 | 0 | 1 |

# 6.4.1.1 Voltage-low detector and clock monitor 

The BM8563 has an on-chip voltage-low detector (see Figure 7). When $\mathrm{V}_{\mathrm{DD}}$ drops below $\mathrm{V}_{\text {low }}$, bit VL in the VL_seconds register is set to indicate that the integrity of the clock information is no longer guaranteed. The VL flag can only be cleared by using the interface.
![img-4.jpeg](img-4.jpeg)

Fig 7. Voltage-low detection
The VL flag is intended to detect the situation when $\mathrm{V}_{\mathrm{DD}}$ is decreasing slowly, for example under battery operation. Should the oscillator stop or $\mathrm{V}_{\mathrm{DD}}$ reach $\mathrm{V}_{\text {low }}$ before power is re-asserted, then the VL flag is set. This will indicate that the time may be corrupted.

### 6.4.2 Register Minutes

Table 7. Minutes - minutes register (address 03h) bit description

| Bit | Symbol | Value | Place value | Description |
| :--: | :--: | :--: | :--: | :--: |
| 7 | - | - | - | unused |
| 6 to 4 | MINUTES | 0 to 5 | ten's place | actual minutes coded in BCD format |
| 3 to 0 |  | 0 to 9 | unit place |  |

# Page 10

# 6.4.3 Register Hours 

Table 8. Hours - hours register (address 04h) bit description

| Bit | Symbol | Value | Place value | Description |
| :--: | :--: | :--: | :--: | :--: |
| 7 to 6 | - | - | - | unused |
| 5 to 4 | HOURS | 0 to 2 | ten's place | actual hours coded in BCD format |
| 3 to 0 |  | 0 to 9 | unit place |  |

### 6.4.4 Register Days

Table 9. Days - days register (address 05h) bit description

| Bit | Symbol | Value | Place value | Description |
| :--: | :--: | :--: | :--: | :--: |
| 7 to 6 | - | - | - | unused |
| 5 to 4 | DAYS $^{[1]}$ | 0 to 3 | ten's place | actual day coded in BCD format |
| 3 to 0 |  | 0 to 9 | unit place |  |

[1] The BM8563 compensates for leap years by adding a 29th day to February if the year counter contains a value which is exactly divisible by 4 , including the year 00 .

### 6.4.5 Register Weekdays

Table 10. Weekdays - weekdays register (address 06h) bit description

| Bit | Symbol | Value | Description |
| :--: | :--: | :--: | :--: |
| 7 to 3 | - | - | unused |
| 2 to 0 | WEEKDAYS | 0 to 6 | actual weekday values, see Table 11 |

Table 11. Weekday assignments

| Day $^{[1]}$ | Bit |  |  |
| :--: | :--: | :--: | :--: |
|  | 2 | 1 | 0 |
| Sunday |  |  |  |
| Monday | 0 | 0 | 1 |
| Tuesday | 0 | 1 | 0 |
| Wednesday | 0 | 1 | 1 |
| Thursday | 1 | 0 | 0 |
| Friday | 1 | 0 | 1 |
| Saturday | 1 | 1 | 0 |

[1] Definition may be re-assigned by the user.

### 6.4.6 Register Century_months

Table 12. Century_months - century flag and months register (address 07h) bit description

| Bit | Symbol | Value | Place value | Description |
| :--: | :--: | :--: | :--: | :--: |
| 7 | $C^{[1]}$ | $0^{[2]}$ | - | indicates the century is $x$ |
|  |  | 1 | - | indicates the century is $x+1$ |
| 6 to 5 | - | - | - | unused |
| 4 | MONTHS | 0 to 1 | ten's place | actual month coded in BCD format, see Table 13 |
| 3 to 0 |  | 0 to 9 | unit place |  |

[1] This bit may be re-assigned by the user.
[2] This bit is toggled when the register Years overflows from 99 to 00.

# Page 11

Table 13. Month assignments in BCD format

|  Month | Upper-digit (ten's place) | Digit (unit place) |  |  |   |
| --- | --- | --- | --- | --- | --- |
|   | Bit 4 | Bit 3 | Bit 2 | Bit 1 | Bit 0  |
|  January | 0 | 0 | 0 | 0 | 1  |
|  February | 0 | 0 | 0 | 1 | 0  |
|  March | 0 | 0 | 0 | 1 | 1  |
|  April | 0 | 0 | 1 | 0 | 0  |
|  May | 0 | 0 | 1 | 0 | 1  |
|  June | 0 | 0 | 1 | 1 | 0  |
|  July | 0 | 0 | 1 | 1 | 1  |
|  August | 0 | 1 | 0 | 0 | 0  |
|  September | 0 | 1 | 0 | 0 | 1  |
|  October | 1 | 0 | 0 | 0 | 0  |
|  November | 1 | 0 | 0 | 0 | 1  |
|  December | 1 | 0 | 0 | 1 | 0  |

# 6.4.7 Register Years

Table 14. Years - years register (08h) bit description

|  Bit | Symbol | Value | Place value | Description  |
| --- | --- | --- | --- | --- |
|  7 to 4 | YEARS | 0 to 9 | ten's place | actual year coded in BCD format ${ }^{[1]}$  |
|  3 to 0 |  | 0 to 9 | unit place |   |

[1] When the register Years overflows from 99 to 00, the century bit C in the register Century_months is toggled.

# Page 12

# 6.5 Setting and reading the time 

Figure 8 shows the data flow and data dependencies starting from the 1 Hz clock tick.
![img-5.jpeg](img-5.jpeg)

Fig 8. Data flow for the time function
During read/write operations, the time counting circuits (memory locations 02 h through 08 h ) are blocked. This prevents

- Faulty reading of the clock and calendar during a carry condition
- Incrementing the time registers, during the read cycle

After this read/write access is completed, the time circuit is released again and any pending request to increment the time counters that occurred during the read access is serviced. A maximum of 1 request can be stored; therefore, all accesses must be completed within 1 second (see Figure 9).
![img-6.jpeg](img-6.jpeg)

Fig 9. Access time for read/write operations
As a consequence of this method, it is very important to make a read or write access in one go, that is, setting or reading seconds through to years should be made in one single access. Failing to comply with this method could result in the time becoming corrupted.
As an example, if the time (seconds through to hours) is set in one access and then in a second access the date is set, it is possible that the time may increment between the two accesses. A similar problem exists when reading. A roll over may occur between reads thus giving the minutes from one moment and the hours from the next.
Recommended method for reading the time:

1. Send a START condition and the slave address for write (A2h).
2. Set the address pointer to 2 (VL_seconds) by sending 02h.
3. Send a RESTART condition or STOP followed by START.
4. Send the slave address for read (A3h).
5. Read VL_seconds.
6. Read Minutes.

# Page 13

7. Read Hours.
8. Read Days.
9. Read Weekdays.
10. Read Century_months.
11. Read Years.
12. Send a STOP condition.

# 6.6 Alarm registers 

### 6.6.1 Register Minute_alarm

Table 15. Minute_alarm - minute alarm register (address 09h) bit description

| Bit | Symbol | Value | Place value | Description |
| :--: | :--: | :--: | :--: | :--: |
| 7 | AE_M | 0 | - | minute alarm is enabled |
|  |  | $1^{[1]}$ | - | minute alarm is disabled |
| 6 to 4 | MINUTE_ALARM | 0 to 5 | ten's place | minute alarm information coded in BCD format |
| 3 to 0 |  | 0 to 9 | unit place |  |

[1] Default value.

### 6.6.2 Register Hour_alarm

Table 16. Hour_alarm - hour alarm register (address 0Ah) bit description

| Bit | Symbol | Value | Place value | Description |
| :--: | :--: | :--: | :--: | :--: |
| 7 | AE_H | 0 | - | hour alarm is enabled |
|  |  | $1^{[1]}$ | - | hour alarm is disabled |
| 6 | - | - | - | unused |
| 5 to 4 | HOUR_ALARM | 0 to 2 | ten's place | hour alarm information coded in BCD format |
| 3 to 0 |  | 0 to 9 | unit place |  |

[1] Default value.

### 6.6.3 Register Day_alarm

Table 17. Day_alarm - day alarm register (address 0Bh) bit description

| Bit | Symbol | Value | Place value | Description |
| :--: | :--: | :--: | :--: | :--: |
| 7 | AE_D | 0 | - | day alarm is enabled |
|  |  | $1^{[1]}$ | - | day alarm is disabled |
| 6 | - | - | - | unused |
| 5 to 4 | DAY_ALARM | 0 to 3 | ten's place | day alarm information coded in BCD format |
| 3 to 0 |  | 0 to 9 | unit place |  |

[1] Default value.

### 6.6.4 Register Weekday_alarm

Table 18. Weekday_alarm - weekday alarm register (address 0Ch) bit description

| Bit | Symbol | Value | Description |
| :--: | :--: | :--: | :--: |
| 7 | AE_W | 0 | weekday alarm is enabled |
|  |  | $1^{[1]}$ | weekday alarm is disabled |
| 6 to 3 | - | - | unused |
| 2 to 0 | WEEKDAY_ALARM | 0 to 6 | weekday alarm information |

[1] Default value.

### 6.6.5 Alarm flag

By clearing the alarm enable bit (AE_x) of one or more of the alarm registers, the corresponding alarm condition(s) are active. When an alarm occurs, AF is set to logic 1. The asserted AF can be used to generate an interrupt ( $\overline{\mathrm{INT}}$ ). The AF

# Page 14

is cleared using the interface.
The registers at addresses 09 h through 0 Ch contain alarm information. When one or more of these registers is loaded with minute, hour, day or weekday, and its corresponding $\mathrm{AE} \_x$ is logic 0 , then that information is compared with the current minute, hour, day, and weekday. When all enabled comparisons first match, the alarm flag (AF in register Control_2) is set to logic 1.
The generation of interrupts from the alarm function is controlled via bit AIE. If bit AIE is enabled, the $\overline{\mathrm{INT}}$ pin follows the condition of bit AF. AF will remain set until cleared by the interface. Once AF has been cleared, it will only be set again when the time increments to match the alarm condition once more. Alarm registers which have their $\mathrm{AE} \_x$ bit at logic 1 are ignored.
![img-7.jpeg](img-7.jpeg)
(1) Only when all enabled alarm settings are matching.

It's only on increment to a matched case that the alarm flag is set, see Section 6.6.5.
Fig 10. Alarm function block diagram

# 6.7 Register CLKOUT_control and clock output 

Frequencies of 32.768 kHz (default), $1.024 \mathrm{kHz}, 32 \mathrm{~Hz}$, and 1 Hz can be generated for use as a system clock, microcontroller clock, input to a charge pump, or for calibration of the oscillator.

Table 19. CLKOUT_control - CLKOUT control register (address 0Dh) bit description

| Bit | Symbol | Value | Description |
| :--: | :--: | :--: | :--: |
| 7 | FE | 0 | the CLKOUT output is inhibited and CLKOUT output is set high-impedance |
|  |  | $1^{[1]}$ | the CLKOUT output is activated |
| 6 to 2 | - | - | unused |
| 1 to 0 | FD[1:0] |  | frequency output atpin CLKOUT |
|  |  | $00^{[1]}$ | 32.768 kHz |
|  |  | 01 | 1.024 kHz |
|  |  | 10 | 32 Hz |
|  |  | 11 | 1 Hz |

[1] Default value.

### 6.8 Timer function

The 8 -bit countdown timer at address 0 Fh is controlled by the Timer_control register at address 0Eh. The Timer_control register determines one of 4 source clock frequencies for the timer ( $4096 \mathrm{~Hz}, 64 \mathrm{~Hz}, 1 \mathrm{~Hz}$, or $1 / 60 \mathrm{~Hz}$ ), and enables or disables the timer. The timer counts down from a software-loaded 8 -bit binary value. At the end of

# Page 15

every countdown, the timer sets the timer flag TF. The TF may only be cleared by using the interface. The asserted TF can be used to generate an interrupt on pin $\overline{\mathrm{INT}}$. The interrupt may be generated as a pulsed signal every countdown period or as a permanently active signal which follows the state of TF. Bit TI_TP is used to control this mode selection. When reading the timer, the current countdown value is returned.

# 6.8.1 Register Timer_control 

Table 20. Timer_control - timer control register (address 0Eh) bit description

| Bit | Symbol | Value | Description |
| :--: | :--: | :--: | :--: |
| 7 | TE | $0^{[1]}$ | timer is disabled |
|  |  | 1 | timer is enabled |
| 6 to 2 | - | - | unused |
| 1 to 0 | TD[1:0] |  | timer source clock frequency select ${ }^{[2]}$ |
|  |  | 00 | 4.096 kHz |
|  |  | 01 | 64 Hz |
|  |  | 10 | 1 Hz |
|  |  | $11^{[3]}$ | $1 / 60 \mathrm{~Hz}$ |

[1] Default value.
[2] These bits determine the source clock for the countdown timer; when not in use, TD[1:0] should be set to $1 / 60 \mathrm{~Hz}$ for power saving.

### 6.8.2 Register Timer

Table 21. Timer - timer value register (address 0Fh) bit description

| Bit | Symbol | Value | Description |
| :--: | :--: | :--: | :--: |
| 7 to 0 | TIMER[7:0] | 00 h to FFh | countdown period in seconds: <br> CountdownPeriod $=\frac{n}{\text { Source ClockFrequency }}$ <br> where $n$ is the countdown value |

Table 22. Timer register bits value range

| Bit |  |  |  |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |

The register Timer is an 8-bit binary countdown timer. It is enabled and disabled via the Timer_control register bit TE. The source clock for the timer is also selected by the Timer_control register. Other timer properties such as interrupt generation are controlled via the register Control_status_2.
For accurate read back of the countdown value, it is recommended to read the register twice and check for consistent results, since it is not possible to freeze the countdown timer counter during read back.

### 6.9 EXT_CLK test mode

A test mode is available which allows for on-board testing. In such a mode it is possible to set up test conditions and control the operation of the RTC.

The test mode is entered by setting bit TEST1 in register Control_status_1. Then pin CLKOUT becomes an input. The test mode replaces the internal 64 Hz signal with the signal applied to pin CLKOUT. Every 64 positive edges applied to pin CLKOUT will then generate an increment of one second.
The signal applied to pin CLKOUT should have a minimum pulse width of 300 ns and a maximum period of 1000 ns . The internal 64 Hz clock, now sourced from CLKOUT, is divided down to 1 Hz by a $2^{6}$ divide chain called a prescaler. The prescaler can be set into a known state by using bit STOP. When bit STOP is set, the prescaler is reset to 0 (STOP must be cleared before the prescaler can operate again).

# Page 16

From a STOP condition, the first 1 second increment will take place after 32 positive edges on CLKOUT. Thereafter, every 64 positive edges will cause a one-second increment.

Remark: Entry into EXT_CLK test mode is not synchronized to the internal 64 Hz clock. When entering the test mode, no assumption as to the state of the prescaler can be made.

# 6.9.1 Operation example: 

1. Set EXT_CLK test mode (Control_status_1, bit TEST1 = 1).
2. Set STOP (Control_status_1, bit STOP = 1).
3. Clear STOP (Control_status_1, bit STOP = 0).
4. Set time registers to desired value.
5. Apply 32 clock pulses to CLKOUT.
6. Read time registers to see the first change.
7. Apply 64 clock pulses to CLKOUT.
8. Read time registers to see the second change.

Repeat steps 7 and 8 for additional increments.

### 6.10 STOP bit function

The function of the STOP bit is to allow for accurate starting of the time circuits. The STOP bit function will cause the upper part of the prescaler ( $F_{2}$ to $F_{14}$ ) to beheld in reset and thus no 1 Hz ticks will be generated (see Figure 11). The time circuits can then be set and will not increment until the STOP bit is released (see Figure 12 and Table 23).
![img-8.jpeg](img-8.jpeg)

Fig 11. STOP bit functional diagram
The STOP bit function will not affect the output of 32.768 kHz on CLKOUT, but will stop the generation of $1.024 \mathrm{kHz}, 32$ Hz , and 1 Hz .
The lower two stages of the prescaler $\left(F_{0}\right.$ and $\left.F_{1}\right)$ are not reset; and because the $I^{2} \mathrm{C}$-bus is asynchronous to the crystal oscillator, the accuracy of re-starting the time circuits will be between zero and one 8.192 kHz cycle (see Figure 12).
![img-9.jpeg](img-9.jpeg)

Fig 12. STOP bit release timing

# Page 17

Table 23. First increment of time circuits after STOP bit release

| Bit STOP | Prescaler bits $F_{0} F_{1}-F_{2}$ to $F_{14}$ | 1 Hz tick | Time hh:mm:ss | Comment |
| :--: | :--: | :--: | :--: | :--: |
| Clock is running normally |  |  |  |  |
| 0 | 01-0 0001 1101 0100 |  | $12: 45: 12$ | prescaler counting normally |
| STOP bit is activated by user. $F_{0} F_{1}$ are not reset and values cannot be predicted externally |  |  |  |  |
| 1 | XX-0 000000000000 |  | $12: 45: 12$ | prescaler is reset; time circuits are frozen |
| New time is set by user |  |  |  |  |
| 1 | XX-0 000000000000 |  | 08:00:00 | prescaler is reset; time circuits are frozen |
| STOP bit is released by user |  |  |  |  |
|  | XX-0 0000000000000 |  | 08:00:00 | prescaler is now running |
|  | XX-1 0000000000000 |  | 08:00:00 | - |
|  | XX-0 0000000000000 |  | 08:00:00 | - |
|  | XX-1 0000000000000 |  | : | : |
|  | 11-1 1111111111110 |  | 08:00:00 | - |
|  | 00-0 0000000000001 |  | 08:00:01 | 0 to 1 transition of $F_{14}$ increments the time circuits |
|  | 10-0 000000000001 |  | 08:00:01 | - |
| 0 | : |  | : | : |
|  | 11-1 1111111111111 |  | 08:00:01 | - |
|  | 00-0 0000000000000 |  | 08:00:01 | - |
|  | 10-0 000000000000 |  | 08:00:01 | - |
|  | : |  | : | : |
|  | 11-1 1111111111110 |  | 08:00:01 | - |
|  | 00-0 0000000000001 |  | 08:00:02 | 0 to 1 transition of $F_{14}$ increments the time circuits |

[1] $F_{0}$ is clocked at 32.768 kHz .
The first increment of the time circuits is between 0.507813 s and 0.507935 s after STOP bit is released. The uncertainty is caused by the prescalerbits $F_{0}$ and $F_{1}$ not being reset (see Table 23) and the unknown state of the 32 kHz clock.

# Page 18

# 6.11 Reset 

The BM8563 includes an internal reset circuit which is active whenever the oscillator is stopped. In the reset state the $\mathrm{I}^{2} \mathrm{C}$-bus logic is initialized including the address pointer and all registers are set according to Table 24. $\mathrm{I}^{2} \mathrm{C}$-bus communication is not possible during reset.

Table 24. Register reset value ${ }^{[1]}$

| Address | Register name | Bit |  |  |  |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|  |  | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| 00h | Control_status_1 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 |
| 01h | Control_status_2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 02h | VL_seconds | 1 | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |
| 03h | Minutes | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |
| 04h | Hours | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |
| 05h | Days | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |
| 06h | Weekdays | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |
| 07h | Century_months | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |
| 08h | Years | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |
| 09h | Minute_alarm | 1 | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |
| 0Ah | Hour_alarm | 1 | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |
| 0Bh | Day_alarm | 1 | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |
| 0Ch | Weekday_alarm | 1 | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |
| 0Dh | CLKOUT_control | 1 | $x$ | $x$ | $x$ | $x$ | $x$ | 0 | 0 |
| 0Eh | Timer_control | 0 | $x$ | $x$ | $x$ | $x$ | $x$ | 1 | 1 |
| 0Fh | Timer | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ | $x$ |

[1] Registers marked $x$ are undefined at power-up and unchanged by subsequent resets.

### 6.11.1 Power-On Reset (POR) override

The POR duration is directly related to the crystal oscillator start-up time. Due to the long start-up times experienced by these types of circuits, a mechanism has been built in to disable the POR and hence speed up on-board test of the device. The setting of this mode requires that the $\mathrm{I}^{2} \mathrm{C}$-bus pins, SDA and SCL, are toggled in a specific order as shown in Figure 13. All timings are required minimums.

Once the override mode has been entered, the device immediately stops, being reset, and normal operation may commence i.e. entry into the EXT_CLK test mode via $\mathrm{I}^{2} \mathrm{C}$-bus access. The override mode may be cleared by writing logic 0 to TESTC. TESTC must be set to logic 1 before re-entry into the override mode is possible. Setting TESTC to logic 0 during normal operation has no effect except to prevent entry into the POR override mode.
![img-10.jpeg](img-10.jpeg)

Fig 13. POR override sequence

# Page 19

# 7 Characteristics of the $\mathrm{I}^{2} \mathrm{C}$-bus 

The $\mathrm{I}^{2} \mathrm{C}$-bus is for bidirectional, two-line communication between different ICs or modules. The two lines are a Serial DAta line (SDA) and a Serial CLock line (SCL). Both lines must be connected to a positive supply via a pull-up resistor. Data transfer may be initiated only when the bus is not busy.

### 7.1 Bit transfer

One data bit is transferred during each clock pulse. The data on the SDA line must remain stable during the HIGH period of the clock pulse as changes in the data line at this time will be interpreted as a control signal (see Figure 14).
![img-11.jpeg](img-11.jpeg)

### 7.2 START and STOP conditions

Both data and clock lines remain HIGH when the bus is not busy.
A HIGH-to-LOW transition of the data line while the clock is HIGH is defined as the START condition - S.
A LOW-to-HIGH transition of the data line while the clock is HIGH is defined as the STOP condition - P (see Figure 15).
![img-12.jpeg](img-12.jpeg)

Fig 15. Definition of START and STOP conditions

### 7.3 System configuration

A device generating a message is a transmitter; a device receiving a message is a receiver. The device that controls the message is the master; and the devices which are controlled by the master are the slaves (see Figure 16).
![img-13.jpeg](img-13.jpeg)

Fig 16. System configuration

# Page 20

# 7.4 Acknowledge 

The number of data bytes transferred between the START and STOP conditions from transmitter to receiver is unlimited. Each byte of eight bits is followed by an acknowledge cycle.

- A slave receiver, which is addressed, must generate an acknowledge after the reception of each byte.
- Also a master receiver must generate an acknowledge after the reception of each byte that has been clocked out of the slave transmitter.
- The device that acknowledges must pull-down the SDA line during the acknowledge clock pulse, so that the SDA line is stable LOW during the HIGH period of the acknowledge related clock pulse (set-up and hold times must be taken into consideration).
- A master receiver must signal an end of data to the transmitter by not generating an acknowledge on the last byte that has been clocked out of the slave. In this event, the transmitter must leave the data line HIGH to enable the master to generate a STOP condition.
Acknowledgement on the $\mathrm{I}^{2} \mathrm{C}$-bus is illustrated in Figure 17.
![img-14.jpeg](img-14.jpeg)

Fig 17. Acknowledgement on the $\mathrm{I}^{2} \mathrm{C}$-bus

### 7.5 $\mathrm{I}^{2} \mathrm{C}$-bus protocol

### 7.5.1 Addressing

Before any data is transmitted on the $\mathrm{I}^{2} \mathrm{C}$-bus, the device which should respond is addressed first. The addressing is always carried out with the first byte transmitted after the start procedure.

The BM8563 acts as a slave receiver or slave transmitter. Therefore the clock signal SCL is only an input signal, but the data signal SDA is a bidirectional line.

Two slave addresses are reserved for the BM8563:
Read: A3h (10100011)
Write: A2h (10100010)
The BM8563 slave address is illustrated in Figure 18.
![img-15.jpeg](img-15.jpeg)

### 7.5.2 Clock and calendar READ or WRITE cycles

The $\mathrm{I}^{2} \mathrm{C}$-bus configuration for the different BM8563 READ and WRITE cycles is shown in Figure 19, Figure 20 and Figure 21. The register address is a 4 -bit value that defines which register is to be accessed next. The upper four bits of the register address are not used.

# Page 21

![img-16.jpeg](img-16.jpeg)

Fig 19. Master transmits to slave receiver (WRITE mode)
![img-17.jpeg](img-17.jpeg)
(1) At this moment master transmitter becomes master receiver and PCF8563 slave receiver becomes slave transmitter.

Fig 20. Master reads after setting register address (write register address; READ data)
![img-18.jpeg](img-18.jpeg)

Fig 21. Master reads slave immediately after first byte (READ mode)

# Page 22

# 8 Application information 

![img-19.jpeg](img-19.jpeg)

Fig 23. Application diagram

### 8.1 Quartz frequency adjustment

### 8.1.1 Method 1: fixed OSCI capacitor

By evaluating the average capacitance necessary for the application layout, a fixed capacitor can be used. The frequency is best measured via the 32.768 kHz signal available after power-on at pin CLKOUT. The frequency tolerance depends on the quartz crystal tolerance, the capacitor tolerance and the device-to-device tolerance (on average $\pm 5$ ppm ). Average deviations of $\pm 5$ minutes per year can be easily achieved.

### 8.1.2 Method 2: OSCI trimmer

Using the 32.768 kHz signal available after power-on at pin CLKOUT, fast setting of a trimmer is possible.

### 8.1.3 Method 3: OSCO output

Direct measurement of OSCO out (accounting for test probe capacitance).

# Page 23

# **PACKAGE DIMENSION SOP8-L**

![img-20.jpeg](img-20.jpeg)

![img-21.jpeg](img-21.jpeg)

|  SYMBOLS | DIMENSION (MM) |  | DIMENSION (INCH) |   |
| --- | --- | --- | --- | --- |
|   | MIN | MAX | MIN | MAX  |
|  A | 1.300 | 1.752 | 0.051 | 0.069  |
|  A1 | 0.000 | 0.203 | 0.000 | 0.008  |
|  A2 | 1.350 | 1.550 | 0.053 | 0.061  |
|  b | 0.330 | 0.510 | 0.013 | 0.020  |
|  C | 5.790 | 6.200 | 0.228 | 0.244  |
|  D | 4.700 | 5.110 | 0.185 | 0.201  |
|  E | 3.800 | 4.000 | 0.150 | 0.157  |
|  e | 1.270 BSC |  | 0.050 BSC |   |
|  H | 0.170 | 0.254 | 0.007 | 0.010  |
|  L | 0.400 | 1.270 | 0.016 | 0.050  |
|  θ | 0° | 8° | 0° | 8°  |

# Page 24

# **PACKAGE DIMENSION MSOP8**

![img-22.jpeg](img-22.jpeg)

![img-23.jpeg](img-23.jpeg)

|  DIMENSION SYMBOLS | MIN (mm) | MAX (mm) | DIMENSION SYMBOLS | MIN (mm) | MAX (mm)  |
| --- | --- | --- | --- | --- | --- |
|  A | 2.90 | 3.10 | C3 | 0.152 |   |
|  A1 | 0.28 | 0.35 | C4 | 0.15 | 0.23  |
|  A2 | 0.65TYP |  | H | 0.00 | 0.09  |
|  A3 | 0.375TYP |  | θ | 12" TYP4 |   |
|  B | 2.90 | 3.10 | θ1 | 12" TYP4 |   |
|  B1 | 4.70 | 5.10 | θ2 | 14" TYP |   |
|  B2 | 0.45 | 0.75 | θ3 | 0° ~ 6° |   |
|  C | 0.75 | 0.95 | R | 0.15TYP |   |
|  C1 | -- | 1.10 | R1 | 0.15TYP |   |
|  C2 | 0.328TYP |  |  |  |   |

# Page 25

# **PACKAGE DIMENSION TSSOP8**

![img-24.jpeg](img-24.jpeg)

|  DIMENSION SYMBOLS | MIN (mm) | MAX (mm) | DIMENSION SYMBOLS | MIN (mm) | MAX (mm)  |
| --- | --- | --- | --- | --- | --- |
|  A | 2.90 | 3.10 | C3 | 0.05 | 0.15  |
|  A1 | 0.20 | 0.30 | D | 1.00REF |   |
|  A2 | 0.65 TYP |  | D1 | 0.50 | 0.70  |
|  A3 | 0.36 | 0.46 | R1 | 0.15 TYP |   |
|  B | 4.30 | 4.50 | R2 | 0.15 TYP |   |
|  B1 | 6.30 | 6.50 | θ1 | 12" TYP4 |   |
|  C | 0.95 | 1.05 | θ2 | 12" TYP4 |   |
|  C1 | 0.127 TYP |  | θ3 | 0" ~ 7" |   |
|  C2 | 0.39 | 0.49 |  |  |   |

# Page 26

# GATEMODE

## 㬵茂微电子

## Real-time clock/calendar

Order Information

|  Order number | Package | Operation Temperature
Range | MSL Grade | Ship, Quantity | Green  |
| --- | --- | --- | --- | --- | --- |
|  BM8563ESA | SOP8-L | -40 to $85^{\circ} \mathrm{C}$ | 3 | T\&R, 4000 | Rohs  |
|  BM8563EMA | MSOP8 | -40 to $85^{\circ} \mathrm{C}$ | 3 | T\&R, 4000 | Rohs  |
|  BM8563EHA | TSSOP8 | -40 to $85^{\circ} \mathrm{C}$ | 3 | T\&R, 4000 | Rohs  |

