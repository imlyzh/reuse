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

ANF ir

1. [x] Insert Drop-Reuse
2. [ ] Liveness Analysis, if move then Insert Drop-Reuse else Insert Drop
3. [ ] Insert Dup to Bind after
4. [ ] Drop unrolling
5. [ ] Dup/Drop reduction

### Optional

- [ ] AST metainfo
- [ ] Memorize
- [ ] inet target
