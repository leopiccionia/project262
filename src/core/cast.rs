use super::p262_has_slot;
use super::Value;
use super::{bigint, numbers};
use crate::errors::{CoreError, CoreResult};

pub(crate) enum IntegerOrInfinity {
    NegativeInfinity,
    Integer(f64),
    PositiveInfinity,
}

pub(crate) fn e262_to_boolean(argument: &Value) -> bool {
    match argument {
        Value::Boolean(value) => *value,
        Value::Null | Value::Undefined => false,
        Value::Number(value) => !numbers::is_zero(*value) && !numbers::is_nan(*value),
        Value::BigInt(value) => !bigint::is_zero(value.clone()),
        Value::String(value) => value.len() > 0,
        Value::Symbol(_) => true,
        Value::Object(value) => {
            if cfg!(feature = "annex-b") {
                !p262_has_slot(value.0.clone(), "IsHTMLDDA".to_string())
            } else {
                true
            }
        }
    }
}

pub(crate) fn e262_to_integer_or_infinity(argument: &Value) -> CoreResult<IntegerOrInfinity> {
    let number = e262_to_number(argument)?;
    if numbers::is_zero(number) || numbers::is_nan(number) {
        Ok(IntegerOrInfinity::Integer(0f64))
    } else if numbers::is_pos_inf(number) {
        Ok(IntegerOrInfinity::PositiveInfinity)
    } else if numbers::is_neg_inf(number) {
        Ok(IntegerOrInfinity::NegativeInfinity)
    } else {
        Ok(IntegerOrInfinity::Integer(number.trunc()))
    }
}

pub(crate) fn e262_to_length(argument: &Value) -> CoreResult<f64> {
    let length = e262_to_integer_or_infinity(argument)?;
    match length {
        IntegerOrInfinity::NegativeInfinity => Ok(0f64),
        IntegerOrInfinity::PositiveInfinity => Ok(numbers::MAX_SAFE_INTEGER),
        IntegerOrInfinity::Integer(value) => Ok(value.clamp(0f64, numbers::MAX_SAFE_INTEGER)),
    }
}

pub(crate) fn e262_to_number(argument: &Value) -> CoreResult<f64> {
    match argument {
        Value::Number(value) => Ok(*value),
        Value::BigInt(_) => Err(CoreError::TypeError(
            "Cannot convert BigInt value into Number".to_string(),
        )),
        Value::Symbol(_) => Err(CoreError::TypeError(
            "Cannot convert Symbol value into Number".to_string(),
        )),
        Value::Undefined => Ok(f64::NAN),
        Value::Null | Value::Boolean(false) => Ok(0f64),
        Value::Boolean(true) => Ok(1f64),
        Value::String(value) => Ok(value.parse::<f64>().unwrap_or(f64::NAN)), // @TODO
        Value::Object(_) => Ok(1f64),                                         // @TODO
    }
}
