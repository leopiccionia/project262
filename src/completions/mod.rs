use crate::errors::{CoreError, CoreResult};

pub enum BaseCompletion<T> {
    Normal(T),
    Throw(CoreError),
}

pub enum Completion<T> {
    Normal(T),
    Throw(CoreError),
    Return(T),
    Break(Option<String>),
    Continue(Option<String>),
}

impl<T> From<CoreResult<T>> for BaseCompletion<T> {
    fn from(result: CoreResult<T>) -> Self {
        match result {
            Ok(value) => Self::Normal(value),
            Err(err) => Self::Throw(err),
        }
    }
}

impl<T> From<CoreResult<T>> for Completion<T> {
    fn from(result: CoreResult<T>) -> Self {
        match result {
            Ok(value) => Self::Normal(value),
            Err(err) => Self::Throw(err),
        }
    }
}

impl<T> From<BaseCompletion<T>> for Completion<T> {
    fn from(completion: BaseCompletion<T>) -> Self {
        match completion {
            BaseCompletion::Normal(value) => Self::Normal(value),
            BaseCompletion::Throw(err) => Self::Throw(err),
        }
    }
}
