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

1. Construct Move-Reuse Pair
2. Insert Drop
3. Unrolling Copy/Move to Bind and Dup/Bind and Optional Reuse
4. Drop unrolling
5. Dup/Drop reduction
