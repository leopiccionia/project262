use rand::random;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MagicId(u64);

impl MagicId {
    pub(crate) fn new() -> Self {
        MagicId(random::<u64>())
    }
}

impl fmt::Display for MagicId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!("Id<{:x}>", &self.0).as_str())
    }
}
