# x2-feed

Stepper-motor based power feed for X2 mill.

The following features are supported:

1. Power feeding in both directions.
1. Two feed modes: "slow" and "fast".
1. Setting feed speed via rotary encoder (both "slow" and "fast").
1. Spindle tachometer via hall sensor.
1. Emergency stop mode: halts stepper motor driver when emergency stop is pressed.
1. LCD screen displays current spindle speed and feed speed.

## PCB
See PCB (Eagle CAD) in the [pcb/](pcb/) directory.

## Building

You need the following software installed:

1. [Rust](https://www.rust-lang.org) 
1. [xargo](https://github.com/japaric/xargo)

To build the binary, run `xargo build`.

