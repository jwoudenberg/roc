//! Stable ids for checked artifact payload stores.

const std = @import("std");

/// Public `CheckedBodyId` declaration.
pub const CheckedBodyId = enum(u32) { _ };
/// Public `CheckedExprId` declaration.
pub const CheckedExprId = enum(u32) { _ };
/// Public `CheckedPatternId` declaration.
pub const CheckedPatternId = enum(u32) { _ };
/// Public `CheckedStatementId` declaration.
pub const CheckedStatementId = enum(u32) { _ };
/// Public `CheckedExhaustivenessSiteId` declaration.
pub const CheckedExhaustivenessSiteId = enum(u32) { _ };
/// Public `CheckedStringLiteralId` declaration.
pub const CheckedStringLiteralId = enum(u32) { _ };
/// Public `CheckedTypeId` declaration.
pub const CheckedTypeId = enum(u32) { _ };
/// Public `CheckedTypeSchemeId` declaration.
pub const CheckedTypeSchemeId = enum(u32) { _ };
/// Public `PatternBinderId` declaration.
pub const PatternBinderId = enum(u32) { _ };

/// One canonical identity for a closure capture, carried immutably through
/// every post-check IR so that operand↔slot joins are exact key lookups
/// instead of fuzzy multi-key matches.
///
/// The `u32` is split into two disjoint ranges by the high bit:
///
///  - **canonical** (high bit clear, `[0, 2^31)`): the identity of a captured
///    checked binding. The index is exactly the `PatternBinderId` of the
///    binder, so a canonical id is a pure function of (module name, source
///    bytes) and is cache-safe to serialize in checked artifacts. Because the
///    mapping is the identity function, the originating binder is always
///    recoverable via `binder()`.
///  - **generated** (high bit set, `[2^31, 2^32)`): the identity of a
///    compiler-synthesized capturable local that has no checked binder —
///    allocated deterministically by the pass that synthesizes it (compile-time
///    evaluation during checking, closure lifting / spec_constr afterwards).
///    The index is a per-owner counter (a `ConstStore` closure during checking,
///    a Monotype/Lifted program afterwards).
pub const CaptureId = enum(u32) {
    _,

    /// High bit distinguishing generated ids from canonical ids.
    const generated_bit: u32 = 0x8000_0000;
    /// Largest index representable in either range.
    pub const max_index: u32 = generated_bit - 1;

    /// The canonical capture id for a captured binder.
    pub fn fromBinder(id: PatternBinderId) CaptureId {
        const raw = @intFromEnum(id);
        std.debug.assert(raw <= max_index);
        return @enumFromInt(raw);
    }

    /// The canonical capture id for a raw binder index.
    pub fn canonical(index: u32) CaptureId {
        std.debug.assert(index <= max_index);
        return @enumFromInt(index);
    }

    /// The generated capture id for a per-owner counter value.
    pub fn generated(index: u32) CaptureId {
        std.debug.assert(index <= max_index);
        return @enumFromInt(index | generated_bit);
    }

    /// Whether this id names a captured checked binder.
    pub fn isCanonical(self: CaptureId) bool {
        return (@intFromEnum(self) & generated_bit) == 0;
    }

    /// Whether this id names a compiler-synthesized capturable local.
    pub fn isGenerated(self: CaptureId) bool {
        return !self.isCanonical();
    }

    /// The `PatternBinderId` this canonical id was derived from. Asserts the id
    /// is canonical.
    pub fn binder(self: CaptureId) PatternBinderId {
        std.debug.assert(self.isCanonical());
        return @enumFromInt(@intFromEnum(self));
    }

    /// The per-owner counter value of a generated id. Asserts the id is
    /// generated.
    pub fn generatedIndex(self: CaptureId) u32 {
        std.debug.assert(self.isGenerated());
        return @intFromEnum(self) & ~generated_bit;
    }
};
