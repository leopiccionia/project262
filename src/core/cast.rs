use super::Value;
use super::{bigint, numbers};

pub(crate) enum IntegerOrInfinity {
    NegativeInfinity,
    Integer(f64),
    PositiveInfinity,
}

pub(crate) fn e262_to_boolean(argument: Value) -> bool {
    match argument {
        Value::Boolean(value) => value,
        Value::Null | Value::Undefined => false,
        Value::Number(value) => !numbers::is_zero(value) && !numbers::is_nan(value),
        Value::BigInt(value) => !bigint::is_zero(value),
        Value::String(value) => value.len() > 0,
        Value::Symbol(_) => true,
        Value::Object(_) => {
            if cfg!(feature = "annex-b") {
                false // @TODO
            } else {
                true
            }
        }
    }
}
