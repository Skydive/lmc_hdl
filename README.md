# lmc_hdl 

Super simple 16-bit CPU written in Verilog and based on the LMC (Little Man Computer). 
Very simple design, single FSM. Written over a weekend in July 2020.
An assembler (`lmcc`) is written in Rust. Example programs include: `fib.lmc`, `mult.lmc`, and `gauss.lmc`.
Verilator simulations are included and are used for testing.

## Overview
Instructions are as follows:
```
+--------------------|-------------+
| 13 (INST ARGUMENT) | 3 (op-code) |
+--------------------|-------------+
```
All instruction arguments are memory addresses. There is a single accumulator register.
The only datatype is unsigned integer.

```
CMD -- op-code -- description
HLT -- 000 -- HALT PROCESSOR
ADD -- 001 -- Add MEM[ARG] to ACC
SUB -- 010 -- Sub MEM[ARG] from ACC
STA -- 011 -- Copy MEM[ARG] to ACC
BRP -- 100 -- no-op (unimplemented)
LDA -- 101 -- Copy ACC to MEM[ARG] 
BRA -- 110 -- Set PC to ARG
BRZ -- 111 -- Set PC to ARG if ACC is 0
```

## Example
The `.lmc` assembly code is mapped directly onto memory. Execution starts from address 0.
Simple example of code, sum integers from 0 to 100. Accumulator is set to 5050 after 1602 cycles.
```
loop:
  // R1 = R1 + R2
  LDA R1
  ADD R2
  STA R1
  
  // R2 = R2 - 1
  LDA R2
  SUB N1
  STA R2
  
  // exit when we hit zero
  BRZ exit
  BRA loop
exit:
  LDA R1
  HLT
R1: 0
R2: 100
N1: 1
```
## Requirements
- pkg-config (`pkg-config`)
- Verilator (`verilator`)
- GCC (`g++`)
- Rust (`cargo`)

## Getting started
Compile the code into memory:
```
cd lmcc
cargo run -- examples/gauss.lmc ../out.hex
cd ..
```

Build CPU and simulate with verilator:
```
make
./test
```
