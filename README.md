# Reuse Project

The Reuse Analyzer implmention.

## IR Pass Hierarchy

### L1 IR

Nested Blocks, Nested Expressions

1. Scope Analysis
2. Shadow Veriable Renaming
3. Tempreture Variable Name (including ignored expressions values)
4. Flattening, To L2 IR

### L2 IR

Flatted IR: Nested Blocks, Flattened Expressions in Blocks

1. Insert Drop-Reuse
2. Lifetime Analysis and Insert Duplication and Drop
3. Drop unrolling
4. Dup/Drop reduction
