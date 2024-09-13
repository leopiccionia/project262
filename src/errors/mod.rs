//! A module implementing helpers for handling ES exceptions.

use crate::core::StringRep;

/// Implements an error object that can be converted to an ES [NativeError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-error-objects) object.
#[derive(Debug)]
pub enum CoreError {
    /// Convertible to an [EvalError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-evalerror) object.
    EvalError(StringRep),
    /// Convertible to a [RangeError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-rangeerror) object.
    RangeError(StringRep),
    /// Convertible to a [ReferenceError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-referenceerror) object.
    ReferenceError(StringRep),
    /// Convertible to a [SyntaxError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-syntaxerror) object.
    SyntaxError(StringRep),
    /// Convertible to a [TypeError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-typeerror) object.
    TypeError(StringRep),
    /// Convertible to an [URIError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-urierror) object.
    URIError(StringRep),
}

/// A [Result] wrapping either a `T` or a [CoreError].
pub type CoreResult<T> = Result<T, CoreError>;
