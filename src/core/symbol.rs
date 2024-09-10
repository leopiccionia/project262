use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use super::id::MagicId;

/// Implements the global [symbols registry](https://tc39.es/ecma262/multipage/fundamental-objects.html#sec-symbol.for).
#[derive(Default)]
pub(crate) struct SymbolRegistry {
    map: RefCell<HashMap<String, Rc<SymbolRep>>>,
}

impl SymbolRegistry {
    /// Returns a symbol from the registry, creating it if not previously available.
    pub fn get(&mut self, description: String) -> Rc<SymbolRep> {
        self.map
            .borrow_mut()
            .entry(description.clone())
            .or_insert_with(|| Rc::new(SymbolRep::named(description.clone())))
            .clone()
    }
}

/// The internal implementation of an ES [Symbol](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-symbol-type) value.
///
/// Each Symbol value have a `[[Description]]` immutable slot that contains an [optional](Option) [String].
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct SymbolRep(MagicId, Option<String>);

impl SymbolRep {
    /// Creates a new [SymbolRep] with an empty (undefined) `[[Description]]` slot.
    pub fn anon() -> Self {
        SymbolRep::new(None)
    }

    /// Creates a new [SymbolRep], setting the `[[Description]]` slot to the provided string.
    pub fn named(description: String) -> Self {
        SymbolRep::new(Some(description))
    }

    /// Creates a new [SymbolRep], with or without a `[[Description]]` slot.
    pub fn new(description: Option<String>) -> Self {
        SymbolRep(MagicId::new(), description)
    }
}

impl fmt::Debug for SymbolRep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SymbolRep").field(&self.1).finish()
    }
}

impl fmt::Display for SymbolRep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn have_identity() {
        let a = SymbolRep::named("foo".to_string());
        let b = SymbolRep::named("foo".to_string());

        assert_eq!(a, a);
        assert_ne!(a, b);
    }

    #[test]
    fn can_be_retrieved() {
        let mut registry = SymbolRegistry::default();
        let a = registry.get("foo".to_string());
        let b = registry.get("foo".to_string());

        assert_eq!(a, b);
    }
}
