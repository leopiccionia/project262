use num_bigint::{BigInt, ToBigInt};

use super::objects::ObjectRep;
use super::string::StringRep;
use super::symbol::SymbolRep;

/// An ES value of any type.
#[derive(Debug, PartialEq)]
pub enum Value {
    /// Holds a [null](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-null-type) value.
    Null,
    /// Holds an [undefined](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-undefined-type) value.
    Undefined,
    /// Holds a [Boolean](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-boolean-type) value.
    Boolean(bool),
    /// Holds a [Number](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-number-type) value.
    Number(f64),
    /// Holds a [BigInt](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-bigint-type) value.
    BigInt(BigInt),
    /// Holds a [Symbol](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-symbol-type) value.
    Symbol(SymbolRep),
    /// Holds a [String](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-string-type) value.
    String(StringRep),
    /// Holds an [Object](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type) value.
    Object(ObjectRep),
}

/// Creates an ES [BigInt](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-bigint-type) value.
pub fn p262_bigint(value: &dyn ToBigInt) -> Option<Value> {
    value.to_bigint().map(Value::BigInt)
}

/// Creates an ES [Boolean](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-boolean-type) value.
pub fn p262_boolean(value: bool) -> Value {
    Value::Boolean(value)
}

/// Creates an ES [Number](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-number-type) value.
pub fn p262_number(value: f64) -> Value {
    Value::Number(value)
}

/// Creates an ES [null](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-null-type) value.
pub fn p262_null() -> Value {
    Value::Null
}

/// Creates an ES [String](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-string-type) value from either a [`String`] or a [`&str`](str).
pub fn p262_str(value: StringRep) -> Value {
    Value::String(value)
}

/// Creates an ES [String](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-string-type) value from a [`String`].
pub fn p262_string(value: String) -> Value {
    Value::String(value.into())
}

/// Creates an ES [Symbol](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-symbol-type) value from an [optional](Option) [`String`].
pub fn p262_symbol(description: Option<String>) -> Value {
    Value::Symbol(SymbolRep::new(description))
}

/// Creates an ES [undefined](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-undefined-type) value.
pub fn p262_undefined() -> Value {
    Value::Undefined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bigint_works() {
        assert_eq!(
            p262_bigint(&1001).unwrap(),
            Value::BigInt(1001.to_bigint().unwrap())
        );
        assert_eq!(p262_bigint(&f64::INFINITY), None);
        assert_eq!(p262_bigint(&f64::NAN), None);

        assert_eq!(
            Value::BigInt(1002.to_bigint().unwrap()),
            Value::BigInt(1002.to_bigint().unwrap())
        );
    }

    #[test]
    fn boolean_works() {
        assert_eq!(p262_boolean(true), Value::Boolean(true));
        assert_eq!(p262_boolean(false), Value::Boolean(false));

        assert_eq!(Value::Boolean(true), Value::Boolean(true));
        assert_eq!(Value::Boolean(false), Value::Boolean(false));
        assert_ne!(Value::Boolean(true), Value::Boolean(false));
    }

    #[test]
    fn number_works() {
        assert_eq!(p262_number(42f64), Value::Number(42f64));

        assert_eq!(Value::Number(42f64), Value::Number(42f64));
    }

    #[test]
    fn null_works() {
        assert_eq!(p262_null(), Value::Null);

        assert_eq!(Value::Null, Value::Null);
    }

    #[test]
    fn string_works() {
        assert_eq!(p262_str("foo".into()), Value::String("foo".into()));

        assert_eq!(p262_string("foo".to_string()), Value::String("foo".into()));

        assert_eq!(Value::String("bar".into()), Value::String("bar".into()));
        assert_eq!(
            Value::String("bar".into()),
            Value::String("bar".to_string().into())
        );
    }

    #[test]
    fn undefined_works() {
        assert_eq!(p262_undefined(), Value::Undefined);

        assert_eq!(Value::Undefined, Value::Undefined);
        assert_ne!(Value::Null, Value::Undefined);
    }
}
