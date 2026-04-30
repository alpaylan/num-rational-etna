//! Deterministic witness tests, one per mined variant.
//!
//! Each witness calls into a `property_*` function in `num_rational::etna` with
//! frozen inputs that were chosen to be the smallest reproduction of the bug.
//! On base HEAD all witnesses pass; with the corresponding mutation active
//! (marauders or patch), the witness for that variant fails.

use num_rational::etna::{
    property_cmp_overflow_safe, property_cmp_zero_numer_equal,
    property_is_pos_neg_zero_excluded, property_recip_zero_panic_and_sign_norm,
    PropertyResult,
};

fn assert_pass(result: PropertyResult) {
    match result {
        PropertyResult::Pass => {}
        PropertyResult::Discard => panic!("witness produced Discard, expected Pass"),
        PropertyResult::Fail(m) => panic!("witness failed: {m}"),
    }
}

// ---- recip_zero_panic_and_sign_norm_8c75506_1 ----

#[test]
fn witness_recip_zero_panic_and_sign_norm_case_zero_panics() {
    assert_pass(property_recip_zero_panic_and_sign_norm((0, 1)));
}

#[test]
fn witness_recip_zero_panic_and_sign_norm_case_neg_one_half() {
    assert_pass(property_recip_zero_panic_and_sign_norm((-1, 2)));
}

// ---- is_pos_neg_zero_excluded_c22e3bf_1 ----

#[test]
fn witness_is_pos_neg_zero_excluded_case_zero() {
    assert_pass(property_is_pos_neg_zero_excluded((0i8, 1i8)));
}

#[test]
fn witness_is_pos_neg_zero_excluded_case_neg_one_half() {
    assert_pass(property_is_pos_neg_zero_excluded((-1i8, 2i8)));
}

// ---- cmp_zero_numer_equal_e10ca81_1 ----

#[test]
fn witness_cmp_zero_numer_equal_case_one_two() {
    assert_pass(property_cmp_zero_numer_equal((1, 2)));
}

#[test]
fn witness_cmp_zero_numer_equal_case_three_five() {
    assert_pass(property_cmp_zero_numer_equal((3, 5)));
}

// ---- cmp_overflow_safe_4e66bbe_1 ----

#[test]
fn witness_cmp_overflow_safe_case_big_vs_small() {
    // Issue #7's example: 127/1 vs 1/127. Naive cross product (a*d) vs (b*c)
    // for i8: 127*127 = 16129, overflows i8. Correct ordering is Greater.
    assert_pass(property_cmp_overflow_safe((127, 1, 1, 127)));
}

#[test]
fn witness_cmp_overflow_safe_case_close_pairs() {
    // 125/127 vs 63/64.
    assert_pass(property_cmp_overflow_safe((125, 127, 63, 64)));
}
