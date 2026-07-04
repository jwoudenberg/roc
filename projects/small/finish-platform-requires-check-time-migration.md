# Finish Platform `requires` Check-Time Migration

## Problem

A Roc platform declares the types it demands from an app in its `requires`
clause (e.g. `requires { main! : List(Arg) => Try({}, [Exit(I32)]) }`).
The checker now sees those signatures: `src/check/Check.zig` processes
platform requirements in `processAppPlatformRequirements`, records
`platform_required_defs`, and checks a required app definition body against
the platform's expected type in `checkDef`.

The migration is not finished. After checking, checked module publication
still runs `applyPlatformRequiredSignatureSubstitutions` and
`specializeResolvedStaticDispatchPlanCallables` in
`src/check/checked_artifact.zig`, rewriting/projecting checked type roots and
dispatch plan callable types after the checker has supposedly finished. That
means the checked boundary is still not a pure freeze point.

Historically, the app module was checked **without** the platform signature. In
an app like

```roc
main! = |args| ...
```

`args` used to stay an unconstrained flex var through the entire check of the
app module. That part is fixed. The remaining problem is the post-check
publication rewrite that grew out of the old architecture. It should disappear,
or shrink to a debug-only assertion that the checked roots already match the
platform requirements.

## Background

The compiler pipeline is: parse → canonicalize → type-check (producing
CheckedModule output per module — all user-facing diagnostics end here) →
postcheck: Monotype IR (monomorphization, `src/postcheck/monotype/`) →
Monotype Lifted (closure lifting) → Lambda Solved → Lambda Mono → LIR lowering
(`src/postcheck/solved_lir_lower.zig`) → ARC insertion (`src/lir/arc*.zig`) →
backends (LLVM/dev/wasm/interpreter). `design.md` at the repo root is the
authoritative post-check design. Its Core Principles state that every stage
after checking consumes explicit data produced by earlier stages, that all
user-facing failures are reported during checking at the latest, and that
post-check stages must not re-derive facts from names or structure.

Platform/app model: a platform module's header has a `requires` clause naming
the definitions the app must provide, with full type annotations. The clause
can also have a *for-clause* naming types the app supplies (e.g. `Model`) that
the requires signature itself refers to. So the relationship is bidirectional:
the platform's scheme is parameterized over app-supplied types. The checker
already has machinery for this: `processRequiresTypes` in `src/check/Check.zig`
processes requires-clause types, and for-clause aliases are tracked in
`for_clause_aliases` (see `isForClauseAliasStatement`), added for alias
handling in PR roc-lang/roc#9850.

The build coordinator (`src/compile/coordinator.zig`) orchestrates per-module
checking and CheckedModule publication. Historically the app was checked as if
standalone, its CheckedModule output was published, and only then did the
platform signature get substituted in — which is why validation had to be
bolted on after the fact. The app is now checked with platform requirements;
the remaining work is deleting the publication-time rewrite.

Error-recovery contract: PR roc-lang/roc#9819 added `allow_user_errors`
plumbing (coordinator + `src/cli/main.zig`) and `runtime_error_inserted`
markers in `src/check/Check.zig` so runs can proceed past user errors by
inserting runtime errors. That contract is orthogonal to this project and
stays.

## Evidence

All symbols below are current.

- `src/check/Check.zig`: `processAppPlatformRequirements`,
  `instantiatePlatformRequiredType`, `platform_required_defs`, and the
  `checkDef` platform-requirement expectation path show that checking now
  consumes the platform `requires` signature before checking required app
  definitions.
- `src/check/checked_artifact.zig`:
  `applyPlatformRequiredSignatureSubstitutions` still runs during artifact
  publication, and the publish path still calls it before building checked
  bodies.
- `src/check/checked_artifact.zig`:
  `specializeResolvedStaticDispatchPlanCallables` still mutates static
  dispatch plan callable types after the plan table is built.
- The old coordinator shadow-reporting layer is gone:
  `PlatformRequiredValidationSnapshot`,
  `appendPlatformRequiredUnresolvedDispatchReports`,
  `appendPlatformRequiredInvalidNumeralDispatchReportIfNeeded`,
  `staticDispatchPlanDispatcherExpr`, and `platformRequiredOkTagPayload` no
  longer exist in `src/compile/coordinator.zig`.
- `builtinNominalAcceptsNumeralLiteral` still exists in
  `checked_artifact.zig`, which means some platform-requirement literal
  validation scaffolding has not fully collapsed into ordinary checking.

## Solution design

Finish the migration by making CheckedModule publication a freeze point.

1. **Prove checker completeness.** Add/debug-enable a publication assertion:
   for each required app definition, the checked root already has the
   platform-required type relation before CheckedModule publication starts. This
   assertion should inspect data produced by checking, not rewrite it.
2. **Delete the post-check rewrite.** Remove
   `applyPlatformRequiredSignatureSubstitutions` or reduce it to the
   debug-only assertion above. Artifact publication must not clone or
   substitute app checked type roots to satisfy platform requirements.
3. **Delete dispatch-plan projection.** Remove
   `specializeResolvedStaticDispatchPlanCallables`. Static dispatch plan
   callable types should already be fixed from checker-owned data, and the
   total-dispatch-plans project should consume those plans without a later
   checked module publication mutation.
4. **Clean remaining scaffolding.** Delete `builtinNominalAcceptsNumeralLiteral`
   and any other platform-requirement literal/dispatch helpers whose callers
   vanish. Diagnostics must come from checking.
5. **Keep default-platform synthesis only as normal input setup.** Headerless
   default apps may still synthesize a default platform; that platform should
   be just another source of check-time requirements, not a reason for
   post-check correction.

Data structures: no new ones. The checker already carries the relevant
platform-requirement data through `platform_required_defs` and the platform
`requires_types`; finishing this should add at most temporary assertion logic
over those existing facts, then delete the publication mutation paths.

## What success looks like

- In an app compiled against a platform, `args` in `main! = |args|` has the
  platform's declared type immediately after the app's check — observable
  before checked module publication starts.
- The repros from issues #9540, #9541, #9542, #9559, #9782, #9857, and #9565
  all produce ordinary check-time type errors with correct source regions.
- `roc check` and `roc run` report byte-identical diagnostics for these
  programs; nothing new appears only at build/finalize time.
- `applyPlatformRequiredSignatureSubstitutions` and
  `specializeResolvedStaticDispatchPlanCallables` no longer exist as rewrite
  paths.
- No platform-requirement function in `src/check/checked_artifact.zig` mutates
  checked type roots or dispatch plan types after checking.

## How to evaluate the result

### Correctness ideal

The invariant: **after the app module's check completes, no checked type root
changes because of platform requirements.** CheckedModule publication is
append/freeze, not rewrite. Enforce it with a debug assertion at publication:
for every required app definition, prove the already-checked root satisfies the
platform requirement before deleting the remaining rewrite path. Secondary
invariant: the coordinator constructs zero `Report` documents describing type
errors; all type diagnostics originate in `src/check/`. A grep-level check
should find no platform-requirement helper that mutates checked type roots or
dispatch plan callable types after checking. The full existing snapshot test
corpus and the platform integration tests must pass with diagnostics owned by
checking, never by publication.

### Performance ideal

The old coordinator snapshot/diff cost is already gone. Finishing this should
remove the remaining checked-type substitution and dispatch-callable projection
work from CheckedModule publication. Measure end-to-end `roc check` and
`roc build` wall time on a platform-based app corpus (examples/ plus a large
app); the expected win is smaller than the original project because the
coordinator shadow-reporting layer has already been deleted, but publication
should still allocate and traverse less.

## Tests to add

- Repro tests (snapshot or integration) for #9540, #9541, #9542, #9559,
  #9782, #9857, #9565, each asserting a check-time diagnostic naming the
  app-side source region — and asserting `roc check` output equals `roc run`
  output.
- A for-clause test: platform requires signature referencing an app-supplied
  `Model`; app provides it; a deliberate mismatch inside `main!` reports at
  check time against the substituted scheme.
- A regression guard for #9890's shape (whatever the coordinator sweep last
  broke) now passing through the checker path.
- A literal-range matrix: each builtin numeric type implied by a platform
  signature × in-range/out-of-range literal → check-time fit error, no silent
  wrap (the #9565 family).
- A debug-build test asserting checked roots are identical before and after
  executable finalization (the publication-freeze invariant).

## Related projects

- [../big/total-dispatch-plans.md](../big/total-dispatch-plans.md) — depends
  on this project: dispatch plans cannot be total while checked module
  publication can still rewrite platform-required checked type roots and
  callable types after checking.
- [../big/exact-numeral-pipeline.md](../big/exact-numeral-pipeline.md) —
  depends on this project: it removes the remaining platform-requirement
  publication rewrite that can obscure literal range facts after checking.
- [../big/unify-build-pipelines.md](../big/unify-build-pipelines.md) —
  independent, but benefits from checked module publication becoming a pure
  freeze step with no platform-requirement correction path.
