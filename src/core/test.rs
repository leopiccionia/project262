use super::{bigint, numbers};
use super::{StringRep, Value};
use crate::errors::{CoreError, CoreResult};

pub(crate) fn e262_is_integral_number(argument: Value) -> bool {
    if let Value::Number(value) = argument {
        value.is_finite() && value.floor() == value
    } else {
        false
    }
}

pub(crate) fn e262_is_strictly_equal(x: &Value, y: &Value) -> bool {
    if e262_type(x) != e262_type(y) {
        false
    } else {
        match (x, y) {
            (&Value::Number(x), &Value::Number(y)) => numbers::e262_equal(x, y),
            (_, _) => e262_same_value_non_number(x, y),
        }
    }
}

pub(crate) fn e262_is_property_key(argument: &Value) -> bool {
    matches!(argument, Value::String(_) | Value::Symbol(_))
}

pub(crate) fn e262_require_object_coercible(argument: Value) -> CoreResult {
    match argument {
        Value::Null => Err(CoreError::TypeError(StringRep::Borrowed(
            "Null value cannot be converted to object",
        ))),
        Value::Undefined => Err(CoreError::TypeError(StringRep::Borrowed(
            "Undefined value cannot be converted to object",
        ))),
        _ => Ok(argument),
    }
}

pub(crate) fn e262_same_value(x: &Value, y: &Value) -> bool {
    if e262_type(x) != e262_type(y) {
        false
    } else {
        match (x, y) {
            (&Value::Number(x), &Value::Number(y)) => numbers::e262_same_value(x, y),
            (_, _) => e262_same_value_non_number(x, y),
        }
    }
}

pub(crate) fn e262_same_value_zero(x: &Value, y: &Value) -> bool {
    if e262_type(x) != e262_type(y) {
        false
    } else {
        match (x, y) {
            (&Value::Number(x), &Value::Number(y)) => numbers::e262_same_value_zero(x, y),
            (_, _) => e262_same_value_non_number(x, y),
        }
    }
}

pub fn e262_same_value_non_number(x: &Value, y: &Value) -> bool {
    match (x, y) {
        (Value::Null, Value::Null) => true,
        (Value::Undefined, Value::Undefined) => true,
        (Value::BigInt(x), Value::BigInt(y)) => bigint::e262_equal(x, y),
        (Value::Boolean(x), Value::Boolean(y)) => *x == *y,
        (Value::String(x), Value::String(y)) => *x == *y,
        (Value::Symbol(x), Value::Symbol(y)) => *x == *y,
        (Value::Object(x), Value::Object(y)) => *x == *y,
        (_, _) => unreachable!(),
    }
}

#[derive(PartialEq, Eq)]
pub(crate) enum Type {
    BigInt,
    Boolean,
    Number,
    Null,
    Object,
    String,
    Symbol,
    Undefined,
}

pub(crate) fn e262_type(value: &Value) -> Type {
    match value {
        Value::BigInt(_) => Type::BigInt,
        Value::Boolean(_) => Type::Boolean,
        Value::Number(_) => Type::Number,
        Value::Null => Type::Null,
        Value::Object(_) => Type::Object,
        Value::String(_) => Type::String,
        Value::Symbol(_) => Type::Symbol,
        Value::Undefined => Type::Undefined,
    }
}
