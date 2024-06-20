# Rust State Machine Example

The goal is to test Finite State Machines in Rust using the [`statig`](https://crates.io/crates/statig) crate.

The sample program demonstrates that it's possible to:
- [x] Use the state machine across different threads
- [x] Achieve observability of dispatched events and states transitions
- [x] Delegate unhandled events to a superstate
- [x] Have a shared storage across states
- [x] Associate additional data with events and / or states

## Usage

You can run the program with:

```bash
cargo run
```