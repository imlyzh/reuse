# Reuse Project

The Reuse Analyzer implmention.

## IR Pass Hierarchy

### L1 IR

AST

1. [ ] Type Inference and Type Check
2. [ ] Tempreture Variable Name (including ignored expressions values)
3. [ ] Flattening, To L2 IR

### L2 IR

ANF ir

1. [ ] Shadow Veriable Renaming
2. [ ] Borrow Check
3. [ ] Linearnize
   1. [ ] Insert Dup

### L3 IR

LinearANF ir

1. [x] Insert Drop/Drop-Reuse
   1. [x] Insert Dup-On-Bind
2. [ ] Drop unrolling
3. [ ] Dup/Drop reduction

### Optional

- [ ] AST metainfo
- [ ] Memorize
- [ ] inet target
