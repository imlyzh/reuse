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

1. [ ] Borrow Check
2. [ ] Linearnize
   1. [ ] Insert Dup

### L3 IR

LinearANF ir

1. [ ] Insert Drop/Drop-Reuse
   1. [ ] Drop unrolling
   2. [ ] Dup/Drop reduction

### Optional

- [ ] AST metainfo
- [ ] Memorize
- [ ] inet target
