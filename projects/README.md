# Compiler Improvement Projects

This folder contains self-contained project specifications for structural
improvements to the compiler. Each `.md` file is written so that someone brand
new to the codebase (human or agent) can read that one file and understand the
problem, the solution approach, what success looks like, how to evaluate the
result for long-term correctness and performance, and what tests to add.

- `small/` — projects on the order of days each: localized, mostly additive
  checks or deletions, low design risk.
- `big/` — projects on the order of weeks each: cross-cutting, and several
  require a design decision before implementation starts.

The projects came out of a root-cause analysis of eight weeks of bug fixes
(May–June 2026). The recurring disease across independent bug clusters was:
facts proven during checking get re-derived downstream from type, name, or
structure content instead of traveling as explicit data, keyed by fragile
identity (name strings, positional order, mutable keys) and enforced only by
panics at the consumption site. Most of these projects either move a fact into
an explicit artifact, assign an identity once and carry it, or delete a
duplicated computation. `design.md` at the repo root is the authoritative
post-check design; these projects implement its stated principles more
completely.

## Recommended order

### Start here

1. [small/finish-platform-requires-check-time-migration.md](small/finish-platform-requires-check-time-migration.md)
   — highest leverage per unit of work in the current set. The coordinator's
   shadow-reporting layer is gone, but checked module publication still applies
   platform-required substitutions after checking. Finishing this removes that
   post-publication rewrite and unblocks total dispatch plans and the exact
   numeral pipeline.

### Dependency chains

**Chain A — dispatch:**
1. `small/finish-platform-requires-check-time-migration.md` (prerequisite: dispatch
   plans cannot be total while checked module publication can still rewrite
   platform-required checked type roots and callable types after checking)
2. [big/total-dispatch-plans.md](big/total-dispatch-plans.md)
3. [big/generalization-time-ambiguity.md](big/generalization-time-ambiguity.md)
   — shares the constraint-provenance foundation with total dispatch plans;
   doable before it, but cheaper after.

**Chain B — identity:**
1. [big/immutable-specialization-identity.md](big/immutable-specialization-identity.md)
   — the remaining identity project. Content-based nominal identity has landed,
   but specialization records still rekey themselves during Monotype lowering.

Chain B also strengthens Chain A (stable cross-module references for resolved
dispatch targets) and the glue project, but does not block them.

**Chain C — ARC:**
1. [big/arc-certifier-lattice-join.md](big/arc-certifier-lattice-join.md)
   — removes the remaining certifier skip path and centralizes
   ownership-transfer keying. The current code already warns on skips and
   fails CI when skips occur; this project closes the hole for real.

**Chain D — numerics:**
1. `small/finish-platform-requires-check-time-migration.md` (removes the remaining
   platform-requirement publication rewrite that can obscure literal range
   facts)
2. [big/exact-numeral-pipeline.md](big/exact-numeral-pipeline.md)
- [small/checked-arithmetic-lir-ops.md](small/checked-arithmetic-lir-ops.md)
  is independent of both and can land any time.

### Independent — start any time, in any order

Small:
- [small/cross-phase-coverage-parity-tests.md](small/cross-phase-coverage-parity-tests.md)
  — cheap insurance; ideally land early so later projects inherit the harness.
- [small/centralize-slice-reuse-predicate.md](small/centralize-slice-reuse-predicate.md)
- [small/store-generation-counters.md](small/store-generation-counters.md)
- [small/checked-arithmetic-lir-ops.md](small/checked-arithmetic-lir-ops.md)
- [small/shared-checked-type-traversal.md](small/shared-checked-type-traversal.md)
- [small/cache-hardening.md](small/cache-hardening.md)
- [small/glue-consumes-committed-layouts.md](small/glue-consumes-committed-layouts.md)
- [small/structural-hoist-contexts.md](small/structural-hoist-contexts.md)

Big:
- [big/decision-tree-match-compiler.md](big/decision-tree-match-compiler.md)
  — independent; benefits from landing the coverage-parity test harness first,
  and pairs naturally with pipeline unification (below) since today every
  match-lowering change must be made twice.
- [big/unify-build-pipelines.md](big/unify-build-pipelines.md) — independent;
  package identity is already centralized, but the run path still hand-wires
  coordinator setup and report rendering.
- [big/row-subsumption.md](big/row-subsumption.md) — independent of all other
  projects, but requires a language-semantics decision before implementation
  starts (see the file).

### Suggested overall sequence

If one person or agent works through everything serially, this order front-loads
leverage and keeps prerequisites satisfied:

1. `small/finish-platform-requires-check-time-migration.md`
2. `small/cross-phase-coverage-parity-tests.md`
3. `small/centralize-slice-reuse-predicate.md`
4. `small/store-generation-counters.md`
5. `small/checked-arithmetic-lir-ops.md`
6. `big/total-dispatch-plans.md`
7. `big/immutable-specialization-identity.md`
8. `small/shared-checked-type-traversal.md`
9. `small/cache-hardening.md`
10. `big/arc-certifier-lattice-join.md`
11. `big/exact-numeral-pipeline.md`
12. `big/generalization-time-ambiguity.md`
13. `big/unify-build-pipelines.md`
14. `big/decision-tree-match-compiler.md`
15. `small/glue-consumes-committed-layouts.md`
16. `small/structural-hoist-contexts.md`
17. `big/row-subsumption.md` (whenever the language decision is made; nothing
    blocks on it)
