//! A module implementing helpers for handling ES exceptions.

/// Implements an error object that can be converted to an ES [NativeError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-error-objects) object.
#[derive(Debug)]
pub enum CoreError {
    /// Convertible to an [EvalError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-evalerror) object.
    EvalError(String),
    /// Convertible to a [RangeError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-rangeerror) object.
    RangeError(String),
    /// Convertible to a [ReferenceError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-referenceerror) object.
    ReferenceError(String),
    /// Convertible to a [SyntaxError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-syntaxerror) object.
    SyntaxError(String),
    /// Convertible to a [TypeError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-typeerror) object.
    TypeError(String),
    /// Convertible to an [URIError](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-native-error-types-used-in-this-standard-urierror) object.
    URIError(String),
}

/// A [Result] wrapping either a `T` or a [CoreError].
///
/// It can be converted into a [`Completion`](crate::completions::Completion) via the [`From`] trait.
pub type CoreResult<T> = Result<T, CoreError>;
