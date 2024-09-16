pub(crate) const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991f64;

pub(crate) fn e262_equal(x: f64, y: f64) -> bool {
    if is_nan(x) || is_nan(y) {
        false
    } else if is_zero(x) && is_zero(y) {
        true
    } else {
        x == y
    }
}

pub(crate) fn e262_same_value(x: f64, y: f64) -> bool {
    if is_nan(x) && is_nan(y) {
        true
    } else if is_zero(x) && is_zero(y) {
        (is_neg_zero(x) && is_neg_zero(y)) || (is_pos_zero(x) && is_pos_zero(y))
    } else {
        x == y
    }
}

#[allow(clippy::if_same_then_else)]
pub(crate) fn e262_same_value_zero(x: f64, y: f64) -> bool {
    if is_nan(x) && is_nan(y) {
        true
    } else if is_zero(x) && is_zero(y) {
        true
    } else {
        x == y
    }
}

#[inline(always)]
pub(crate) fn is_inf(value: f64) -> bool {
    value.is_infinite()
}

#[inline(always)]
pub(crate) fn is_nan(value: f64) -> bool {
    value.is_nan()
}

#[inline(always)]
pub(crate) fn is_neg_inf(value: f64) -> bool {
    value == f64::NEG_INFINITY
}

#[inline(always)]
pub(crate) fn is_neg_zero(value: f64) -> bool {
    value == 0.0 && value.is_sign_negative()
}

#[inline(always)]
pub(crate) fn is_pos_inf(value: f64) -> bool {
    value == f64::INFINITY
}

#[inline(always)]
pub(crate) fn is_pos_zero(value: f64) -> bool {
    value == 0.0 && value.is_sign_positive()
}

#[inline(always)]
pub(crate) fn is_zero(value: f64) -> bool {
    value == 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_nan_works() {
        assert!(is_nan(f64::NAN));
        assert!(!is_nan(0.0));
    }

    #[test]
    fn is_neg_zero_works() {
        assert!(is_neg_zero(-0.0));
        assert!(!is_neg_zero(0.0));
    }

    #[test]
    fn is_pos_zero_works() {
        assert!(is_pos_zero(0.0));
        assert!(!is_pos_zero(-0.0));
    }

    #[test]
    fn is_zero_works() {
        assert!(is_zero(0.0));
        assert!(is_zero(-0.0));
        assert!(!is_zero(1.0));
    }
}
