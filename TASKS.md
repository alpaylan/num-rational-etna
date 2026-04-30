# num-rational — ETNA Tasks

Total tasks: 16

## Task Index

| Task | Variant | Framework | Property | Witness |
|------|---------|-----------|----------|---------|
| 001 | `cmp_overflow_safe_4e66bbe_1` | proptest | `CmpOverflowSafe` | `witness_cmp_overflow_safe_case_big_vs_small` |
| 002 | `cmp_overflow_safe_4e66bbe_1` | quickcheck | `CmpOverflowSafe` | `witness_cmp_overflow_safe_case_big_vs_small` |
| 003 | `cmp_overflow_safe_4e66bbe_1` | crabcheck | `CmpOverflowSafe` | `witness_cmp_overflow_safe_case_big_vs_small` |
| 004 | `cmp_overflow_safe_4e66bbe_1` | hegel | `CmpOverflowSafe` | `witness_cmp_overflow_safe_case_big_vs_small` |
| 005 | `cmp_zero_numer_equal_e10ca81_1` | proptest | `CmpZeroNumerEqual` | `witness_cmp_zero_numer_equal_case_one_two` |
| 006 | `cmp_zero_numer_equal_e10ca81_1` | quickcheck | `CmpZeroNumerEqual` | `witness_cmp_zero_numer_equal_case_one_two` |
| 007 | `cmp_zero_numer_equal_e10ca81_1` | crabcheck | `CmpZeroNumerEqual` | `witness_cmp_zero_numer_equal_case_one_two` |
| 008 | `cmp_zero_numer_equal_e10ca81_1` | hegel | `CmpZeroNumerEqual` | `witness_cmp_zero_numer_equal_case_one_two` |
| 009 | `is_pos_neg_zero_excluded_c22e3bf_1` | proptest | `IsPosNegZeroExcluded` | `witness_is_pos_neg_zero_excluded_case_zero` |
| 010 | `is_pos_neg_zero_excluded_c22e3bf_1` | quickcheck | `IsPosNegZeroExcluded` | `witness_is_pos_neg_zero_excluded_case_zero` |
| 011 | `is_pos_neg_zero_excluded_c22e3bf_1` | crabcheck | `IsPosNegZeroExcluded` | `witness_is_pos_neg_zero_excluded_case_zero` |
| 012 | `is_pos_neg_zero_excluded_c22e3bf_1` | hegel | `IsPosNegZeroExcluded` | `witness_is_pos_neg_zero_excluded_case_zero` |
| 013 | `recip_zero_panic_and_sign_norm_8c75506_1` | proptest | `RecipZeroPanicAndSignNorm` | `witness_recip_zero_panic_and_sign_norm_case_zero_panics` |
| 014 | `recip_zero_panic_and_sign_norm_8c75506_1` | quickcheck | `RecipZeroPanicAndSignNorm` | `witness_recip_zero_panic_and_sign_norm_case_zero_panics` |
| 015 | `recip_zero_panic_and_sign_norm_8c75506_1` | crabcheck | `RecipZeroPanicAndSignNorm` | `witness_recip_zero_panic_and_sign_norm_case_zero_panics` |
| 016 | `recip_zero_panic_and_sign_norm_8c75506_1` | hegel | `RecipZeroPanicAndSignNorm` | `witness_recip_zero_panic_and_sign_norm_case_zero_panics` |

## Witness Catalog

- `witness_cmp_overflow_safe_case_big_vs_small` — base passes, variant fails
- `witness_cmp_overflow_safe_case_close_pairs` — base passes, variant fails
- `witness_cmp_zero_numer_equal_case_one_two` — base passes, variant fails
- `witness_cmp_zero_numer_equal_case_three_five` — base passes, variant fails
- `witness_is_pos_neg_zero_excluded_case_zero` — base passes, variant fails
- `witness_is_pos_neg_zero_excluded_case_neg_one_half` — base passes, variant fails
- `witness_recip_zero_panic_and_sign_norm_case_zero_panics` — base passes, variant fails
- `witness_recip_zero_panic_and_sign_norm_case_neg_one_half` — base passes, variant fails
