# Formal Verification

## Files

There are three main folders:

- `chips` for the definition of the RISC-V instructions
- `gadgets` for all the helpers used to define the circuits
- `precompiles` for the precompiles

## Individual files

Constraints are defined in Plonky3. A nice addition is the `builder.with_scope` primitive which adds a string message in the logs, so that we might not need to add additional logs to compare with the Rocq representation.

- `chips`
  - `alu`
    - `add_sub` There are flags to know in which operation we are. We use a lookup to connect the operands and result. There is an opcode number associated to each operation. They use the addition gadget with a function call. There is a nice trick to represent the addition and substraction in the same way: ordering the parameters in the lookup in a different way. There are 8 additions/substractions that can be made in parallel.
    - `bitwise` Parallelism of 2. It uses the convention of representing words of 32 bits with 4 bytes of 8 bits (this is like that in all of the code). It uses lookups to compute the result of each possible bitwise operation, byte by byte.
    - `divrem` The columns are quite complex. There is a flag for division by zero. There are the operations `DIV`, `DIVU`, `REM`, and `REMU`. There is a `is_real` column to know if this operation is active, like in most other operations. We often talk about multiplicity (we need to understand what is it exactly, probably for lookups). The quotient/remainder are computed in 64 bits to avoid overflow, writing the equations linking a, b, q, and r. Then there are many steps to handle all the overflow cases, sign calculations, ... There is a primitive for range check, to force a value to be a byte.
    - `lt`
    - `mul` There are four different forms of instructions for the multiplication. The code is not fully trivial, but there are nice comments.
    - `sll`
    - `sr`
  - `alu_base` `+`, `-`, `*`, and `/` directly on field elements. It also uses accesses to the memory.
  - `alu_ext` Similar to `alu_base`, with binamila extensions (unclear to us what this is).
  - `batch_fri` Unclear what it does.
  - `byte` Compute many byte operations using lookups, in parallel. Interestingly, there is an additional `event.rs` file.
  - `exp_reverse_bits` There is a notion of "pre-processed" columns, that we have already seen in other chips. We make calculations of exponentiations.
  - `poseidon2` Wrapping the Poseidon2 gadget.
  - `public_values` Unclear but very short. Maybe just to ensure we can feed in global constants to the circuit?
  - `recursion_memory` Unclear what it does, but also very short. They use the "pre-processed" matrix, we need to understand what it is. Do we have two matrices in parallel?
  - `risv_cpu` One of the most interesting folder. It might be the main folder redirecting all the instructions to what is needed to evaluate them.
  - `risv_global` Rather small.
  - `riscv_memory` All what is related to memory handling.
  - `riscv_poseidon2` Another wrapper around the Poseidon2 gadget. Small.
  - `riscv_program` Small.
  - `select`
  - `syscall`
  - `toys`
- `gadgets` Half the size of the code for chips.
- `precompile` Also half the size of the code for chips. Dependencies on the precompiles from Plonky3.
