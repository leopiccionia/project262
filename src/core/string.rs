use std::borrow::Cow;

/// A [copy-on-write](Cow) string.
pub type StringRep = Cow<'static, str>;
