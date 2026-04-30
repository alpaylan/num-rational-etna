//! ETNA benchmark harness.
//!
//! Framework-neutral `PropertyResult` enum plus one `property_*` function per
//! mined bug. Every framework adapter in `src/bin/etna.rs` and every witness
//! test calls into these functions.

#![allow(missing_docs)]

use crate::Ratio;
use num_traits::Signed;
use std::format;
use std::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyResult {
    Pass,
    Fail(String),
    Discard,
}

/// `Ratio::recip` must (a) panic when called on a zero ratio and (b) return a
/// reciprocal whose denominator is positive (i.e. normalize the sign so that
/// `recip(-1/2) == -2/1`, not `1/-2`). The fixed implementation routes through
/// `into_recip`, which panics on zero numerator and flips both signs when the
/// numerator is negative.
///
/// Bug `recip_zero_panic_and_sign_norm_8c75506_1` reverts `into_recip` to the
/// naive `Ratio::new_raw(self.denom, self.numer)` body. That returns
/// `1/0` (no panic) for `recip(0/1)` and leaves the denominator negative for
/// negative inputs.
pub fn property_recip_zero_panic_and_sign_norm(inputs: (i32, i32)) -> PropertyResult {
    let (numer, denom) = inputs;
    if denom == 0 {
        return PropertyResult::Discard;
    }
    if numer == 0 {
        // Verify recip panics on zero.
        let r: Ratio<i32> = Ratio::new(0, denom);
        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| r.recip())).is_err();
        if panicked {
            return PropertyResult::Pass;
        }
        return PropertyResult::Fail(format!(
            "Ratio::new(0, {}).recip() did not panic",
            denom
        ));
    }
    // Avoid i32::MIN which has no positive negation.
    if numer == i32::MIN || denom == i32::MIN {
        return PropertyResult::Discard;
    }
    let r: Ratio<i32> = Ratio::new(numer, denom);
    let rec = r.recip();
    if *rec.denom() <= 0 {
        return PropertyResult::Fail(format!(
            "Ratio::new({}, {}).recip() = {}/{} has non-positive denominator",
            numer,
            denom,
            rec.numer(),
            rec.denom()
        ));
    }
    PropertyResult::Pass
}

/// `Signed::is_positive` and `Signed::is_negative` for `Ratio<i8>` must both
/// return `false` when the ratio is zero, matching the integer `Signed` impl
/// (zero is neither positive nor negative).
///
/// Bug `is_pos_neg_zero_excluded_c22e3bf_1` reverts the impls to the buggy
/// `is_positive(self) == !self.is_negative()` and
/// `is_negative(self) == self.numer.is_negative() ^ self.denom.is_negative()`,
/// which makes zero report as positive.
///
/// We use `i8` for the input type so a uniform `Arbitrary` draw hits the
/// `numer == 0` case (1 in 256) within the standard 200-trial PBT budget.
pub fn property_is_pos_neg_zero_excluded(inputs: (i8, i8)) -> PropertyResult {
    let (numer, denom) = inputs;
    if denom == 0 {
        return PropertyResult::Discard;
    }
    if numer == i8::MIN || denom == i8::MIN {
        return PropertyResult::Discard;
    }
    let r: Ratio<i8> = Ratio::new(numer, denom);
    let pos = r.is_positive();
    let neg = r.is_negative();
    let actual_zero = numer == 0;
    let actual_pos = (numer > 0 && denom > 0) || (numer < 0 && denom < 0);
    let actual_neg = (numer < 0 && denom > 0) || (numer > 0 && denom < 0);
    if actual_zero {
        if pos || neg {
            return PropertyResult::Fail(format!(
                "Ratio::new({}, {}) reports is_positive={} is_negative={}, expected both false (zero)",
                numer, denom, pos, neg
            ));
        }
    } else if pos != actual_pos || neg != actual_neg {
        return PropertyResult::Fail(format!(
            "Ratio::new({}, {}) reports is_positive={} is_negative={}, expected pos={} neg={}",
            numer, denom, pos, neg, actual_pos, actual_neg
        ));
    }
    PropertyResult::Pass
}

/// `Ratio::cmp` on two ratios with equal numerators that are zero must report
/// `Equal`, regardless of the denominators. This is the special case patched
/// in commit e10ca81 — without the guard, `Ratio::new_raw(0, 1).cmp(&Ratio::new_raw(0, 2))`
/// reports `Less` because the equal-numerators branch falls through to a
/// denominator comparison that does not understand zero numerators.
///
/// Bug `cmp_zero_numer_equal_e10ca81_1` removes the special-case zero check
/// inside the equal-numerators branch.
pub fn property_cmp_zero_numer_equal(inputs: (i32, i32)) -> PropertyResult {
    let (d1, d2) = inputs;
    if d1 == 0 || d2 == 0 {
        return PropertyResult::Discard;
    }
    if d1 == i32::MIN || d2 == i32::MIN {
        return PropertyResult::Discard;
    }
    // Use new_raw to keep the denominators distinct (new() reduces zero to 0/1).
    let a: Ratio<i32> = Ratio::new_raw(0, d1);
    let b: Ratio<i32> = Ratio::new_raw(0, d2);
    let ord_ab = a.cmp(&b);
    let ord_ba = b.cmp(&a);
    use core::cmp::Ordering;
    if ord_ab != Ordering::Equal {
        return PropertyResult::Fail(format!(
            "Ratio::new_raw(0,{}).cmp(Ratio::new_raw(0,{})) = {:?}, expected Equal",
            d1, d2, ord_ab
        ));
    }
    if ord_ba != Ordering::Equal {
        return PropertyResult::Fail(format!(
            "Ratio::new_raw(0,{}).cmp(Ratio::new_raw(0,{})) = {:?}, expected Equal",
            d2, d1, ord_ba
        ));
    }
    PropertyResult::Pass
}

/// `Ratio::cmp` on `Ratio<i8>` must not overflow when comparing values whose
/// cross product (`a*d` vs `b*c`) wraps i8. Concretely, comparing
/// `Ratio::new(a, b)` with `Ratio::new(c, d)` must agree with the
/// mathematically correct ordering of the rationals `a/b` and `c/d` for every
/// nonzero `(a,b,c,d): i8` quadruple.
///
/// Bug `cmp_overflow_safe_4e66bbe_1` reverts `Ord::cmp` (and its companion
/// `PartialEq` / `PartialOrd` / `Eq` impls) to the original macro-generated
/// `(a*d).cmp(&(b*c))` form. With i8 values, the cross product overflows
/// silently and produces wrong ordering for inputs like `(127,1)` vs `(1,127)`.
pub fn property_cmp_overflow_safe(inputs: (i8, i8, i8, i8)) -> PropertyResult {
    let (a, b, c, d) = inputs;
    if b == 0 || d == 0 {
        return PropertyResult::Discard;
    }
    if a == i8::MIN || b == i8::MIN || c == i8::MIN || d == i8::MIN {
        return PropertyResult::Discard;
    }
    let lhs: Ratio<i8> = Ratio::new(a, b);
    let rhs: Ratio<i8> = Ratio::new(c, d);
    let got = lhs.cmp(&rhs);
    // Compute the truth using i64 cross-product (no overflow possible at i8 scale).
    let na = a as i64;
    let nb = b as i64;
    let nc = c as i64;
    let nd = d as i64;
    let lhs_x = na * nd;
    let rhs_x = nb * nc;
    let same_sign = (nb > 0) == (nd > 0);
    let expected = if same_sign {
        lhs_x.cmp(&rhs_x)
    } else {
        rhs_x.cmp(&lhs_x)
    };
    if got != expected {
        return PropertyResult::Fail(format!(
            "Ratio::new({},{}).cmp(Ratio::new({},{})) = {:?}, expected {:?}",
            a, b, c, d, got, expected
        ));
    }
    PropertyResult::Pass
}
