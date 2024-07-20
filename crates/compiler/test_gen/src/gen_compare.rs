#[cfg(feature = "gen-llvm")]
use crate::helpers::llvm::assert_evals_to;

#[cfg(feature = "gen-dev")]
use crate::helpers::dev::assert_evals_to;

#[cfg(feature = "gen-wasm")]
use crate::helpers::wasm::assert_evals_to;

use indoc::indoc;

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn compare_u64() {
    assert_evals_to!(
        indoc!(
            r#"
                i : U64
                i = 1

                j : U64
                j = 2

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        2,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : U64
                i = 2

                j : U64
                j = 1

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        1,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : U64
                i = 1

                j : U64
                j = 1

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        0,
        u8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn compare_i64() {
    assert_evals_to!(
        indoc!(
            r#"
                i : I64
                i = -1

                j : I64
                j = 1

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        2,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : I64
                i = 1

                j : I64
                j = -1

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        1,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : I64
                i = -1

                j : I64
                j = -1

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        0,
        u8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn compare_f64() {
    assert_evals_to!(
        indoc!(
            r#"
                i : F64
                i = 1.2

                j : F64
                j = 1.3

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        2,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : F64
                i = 1.3

                j : F64
                j = 1.2

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        1,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : F64
                i = 1.2

                j : F64
                j = 1.2

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        0,
        u8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn compare_bool() {
    assert_evals_to!(
        indoc!(
            r#"
                i : Bool
                i = Bool.false

                j : Bool
                j = Bool.true

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        2,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : Bool
                i = Bool.true

                j : Bool
                j = Bool.false

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        1,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : Bool
                i = Bool.false

                j : Bool
                j = Bool.false

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        0,
        u8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn compare_struct() {
    assert_evals_to!(
        indoc!(
            r#"
                i : { a : Bool }
                i = { a : Bool.false }

                j : { a : Bool }
                j = { a : Bool.false }

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        0,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : { a : Bool }
                i = { a : Bool.true }

                j : { a : Bool }
                j = { a : Bool.false }

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        1,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : { a : I64 }
                i = { a : 2 }

                j : { a : I64 }
                j = { a  : 3 }

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        2,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : { a : Bool, b : F32}
                i = { a : Bool.false, b : 1.2 }

                j : { a : Bool, b : F32 }
                j = { a : Bool.false, b : 1.3 }

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        2,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : { a : Bool, b : List F32}
                i = { a : Bool.false, b : [1.3] }

                j : { a : Bool, b : List F32 }
                j = { a : Bool.false, b : [1.2] }

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        1,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : { a : Bool, b : List U32}
                i = { a : Bool.false, b : [13] }

                j : { a : Bool, b : List U32 }
                j = { a : Bool.false, b : [13] }

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        0,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : { }
                i = { }

                j : { }
                j = { }

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        0,
        u8
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev", feature = "gen-wasm"))]
fn list() {
    assert_evals_to!(
        indoc!(
            r#"
                i : List Bool
                i = []

                j : List Bool
                j = []

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        0,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : List Bool
                i = [Bool.true]

                j : List Bool
                j = []

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        1,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : List Bool
                i = []

                j : List Bool
                j = [Bool.true]

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        2,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : List Bool
                i = [Bool.true]

                j : List Bool
                j = [Bool.true]

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        0,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : List Bool
                i = [Bool.true]

                j : List Bool
                j = [Bool.false]

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        1,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : List Bool
                i = [Bool.false]

                j : List Bool
                j = [Bool.true]

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        2,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : List I64
                i = [1, 2]

                j : List I64
                j = [1, 1]

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        1,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : List I64
                i = [1]

                j : List I64
                j = [1, 1]

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        2,
        u8
    );
    assert_evals_to!(
        indoc!(
            r#"
                i : List (List I64)
                i = [[0], [1, 2]]

                j : List (List I64)
                j = [[0], [1, 1]]

                when List.compare i j is
                    Equal -> 0
                    GreaterThan -> 1
                    LessThan -> 2
            "#
        ),
        1,
        u8
    );
}
