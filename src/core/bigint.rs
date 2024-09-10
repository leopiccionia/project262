use num_bigint::BigInt;

pub(crate) fn e262_equal(x: &BigInt, y: &BigInt) -> bool {
    x.eq(y)
}

#[inline(always)]
pub(crate) fn is_zero(value: BigInt) -> bool {
    value == BigInt::ZERO
}
