//! A module implementing ES [completion records](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-completion-record-specification-type).
//!
//! Native code typically return [`CoreResult`], that can be converted into a [`Completion`] via the [`From`] trait: [`Ok`] are converted into [`Normal`](Completion::Normal) completions, while [`Err`] are converted into [`Throw`](Completion::Throw) completions.

use crate::errors::{CoreError, CoreResult};

/// A [completion](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-completion-record-specification-type) record.
///
/// It can be created from a [`CoreResult`] via the [`From`] trait: [`Ok`] are converted into [`Normal`](Self::Normal) completions, while [`Err`] are converted into [`Throw`](Self::Throw) completions.
pub enum Completion<T> {
    /// A normal completion.
    Normal(T),
    /// A throw completion, related to the `throw` keyword.
    Throw(CoreError),
    /// A return completion, related to the `return` keyword.
    Return(T),
    /// A break completion, related to the `break` keyword.
    Break(Option<String>),
    /// A continue completion, related to the `continue` keyword.
    Continue(Option<String>),
}

impl<T> From<CoreResult<T>> for Completion<T> {
    fn from(result: CoreResult<T>) -> Self {
        match result {
            Ok(value) => Self::Normal(value),
            Err(err) => Self::Throw(err),
        }
    }
}
