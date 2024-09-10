use ordermap::OrderMap;
use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use super::id::MagicId;
use super::property::Descriptor;
use super::{Property, StringRep, SymbolRep};

/// An [Object](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type) property key.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PropertyKey {
    /// A [String](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-string-type) key.
    String(StringRep),
    /// A [Symbol](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-symbol-type) key.
    Symbol(SymbolRep),
}

/// Implements the internal methods of an [Object](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type).
///
/// The default implementation of those methods are defined by the [`BaseObject`] struct, and other structs can leverage them via the [`HasBaseObject`] trait, but one or more internal methods can be overriden by [exotic objects](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#exotic-object).
pub trait Object: Debug {
    /// Implements the [`[[GetPrototypeOf]]`](https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-ordinary-object-internal-methods-and-internal-slots-getprototypeof) internal method.
    fn get_prototype_of(self: Rc<Self>) -> Option<Rc<ObjectRep>>;

    // fn set_prototype_of(self: Rc<Self>, proto: Rc<ObjectRep>) -> bool;

    /// Implements the [`[[IsExtensible]]`](https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-ordinary-object-internal-methods-and-internal-slots-isextensible) internal method.
    fn is_extensible(self: Rc<Self>) -> bool;

    /// Implements the [`[[PreventExtensions]]`](https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-ordinary-object-internal-methods-and-internal-slots-preventextensions) internal method.
    fn prevent_extensions(self: Rc<Self>) -> bool;

    ///Implements the [`[[GetOwnProperty]]`](https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-ordinary-object-internal-methods-and-internal-slots-getownproperty-p) internal method.
    fn get_own_property(self: Rc<Self>, key: &PropertyKey) -> Option<Property>;

    // fn define_own_property(self: Rc<Self>, key: PropertyKey, prop: Property) -> bool;

    // fn has_property(self: Rc<Self>, key: &PropertyKey) -> bool;

    // fn get(self: Rc<Self>, key: &PropertyKey, receiver: Value) -> Value;

    // fn set(self: Rc<Self>, key: &PropertyKey, value: Value, receiver: Value) -> bool;

    /// Implements the [`[[Delete]]`](https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-ordinary-object-internal-methods-and-internal-slots-delete-p) internal method.
    fn delete(self: Rc<Self>, key: &PropertyKey) -> bool;

    // fn own_property_keys(self: Rc<Self>) -> Vec<&PropertyKey>;
}

/// The internal implementation for an ES [ordinary object](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#ordinary-object).
#[derive(Debug)]
pub struct BaseObject {
    props: RefCell<OrderMap<PropertyKey, Property>>,
    slots: RefCell<HashMap<String, Rc<dyn 'static + Any>>>,
    prototype: RefCell<Option<Rc<ObjectRep>>>,
    extensible: Cell<bool>,
}

impl Object for BaseObject {
    fn get_prototype_of(self: Rc<Self>) -> Option<Rc<ObjectRep>> {
        e262_ordinary_get_prototype_of(self)
    }

    fn is_extensible(self: Rc<Self>) -> bool {
        e262_ordinary_is_extensible(self)
    }

    fn prevent_extensions(self: Rc<Self>) -> bool {
        e262_ordinary_prevent_extensions(self)
    }

    fn get_own_property(self: Rc<Self>, key: &PropertyKey) -> Option<Property> {
        e262_ordinary_get_own_property(self, key)
    }

    fn delete(self: Rc<Self>, key: &PropertyKey) -> bool {
        e262_ordinary_delete(self, key)
    }
}

/// Gets a [`BaseObject`] from an [ordinary](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#ordinary-object) or [exotic](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#exotic-object) Object implementation.
///
/// It's useful for easily implementing all the methods in [`Object`] trait.
pub trait HasBaseObject: Object {
    /// Gets a reference to a [`BaseObject`].
    fn get_object(self: Rc<Self>) -> Rc<BaseObject>;
}

impl HasBaseObject for BaseObject {
    fn get_object(self: Rc<Self>) -> Rc<BaseObject> {
        self
    }
}

pub(crate) fn e262_ordinary_delete(obj: Rc<dyn HasBaseObject>, key: &PropertyKey) -> bool {
    match Object::get_own_property(obj.clone(), key) {
        None => true,
        Some(desc) => {
            if desc.is_configurable() {
                let base = obj.get_object();
                base.props.borrow_mut().remove(key);
                true
            } else {
                false
            }
        }
    }
}

pub(crate) fn e262_ordinary_get_own_property(
    obj: Rc<dyn HasBaseObject>,
    key: &PropertyKey,
) -> Option<Property> {
    let base = obj.get_object();
    let props = base.props.borrow();
    props.get(key).cloned()
}

pub(crate) fn e262_ordinary_get_prototype_of(obj: Rc<dyn HasBaseObject>) -> Option<Rc<ObjectRep>> {
    let base = obj.get_object();
    let proto = base.prototype.borrow();
    proto.clone()
}

pub(crate) fn e262_ordinary_is_extensible(obj: Rc<dyn HasBaseObject>) -> bool {
    let base = obj.get_object();
    base.extensible.get()
}

pub(crate) fn e262_ordinary_prevent_extensions(obj: Rc<dyn HasBaseObject>) -> bool {
    let base = obj.get_object();
    base.extensible.set(false);
    true
}

#[allow(clippy::if_same_then_else)]
fn e262_validate_and_apply_property_descriptor(
    obj: Option<Rc<dyn HasBaseObject>>,
    key: &PropertyKey,
    extensible: bool,
    desc: &Descriptor,
    current: Option<Property>,
) -> bool {
    match current {
        None => {
            if !extensible {
                return false;
            }
            match obj {
                None => true,
                Some(obj) => {
                    let base = obj.get_object();
                    let prop: Property = desc.clone().into();
                    let mut props = base.props.borrow_mut();
                    props.insert(key.clone(), prop);
                    true
                }
            }
        }
        Some(current) => {
            let desc = desc.clone();
            if desc.is_empty() {
                true
            } else if !current.is_configurable() {
                if matches!(desc.configurable, Some(true)) {
                    false
                } else if let Some(enumerable) = desc.enumerable {
                    false
                } else {
                    true
                    // @TODO
                }
            } else {
                true // @TODO
            }
            // @TODO
            false
        }
    }
}

pub(crate) fn p262_get_slot<T: 'static>(obj: Rc<dyn HasBaseObject>, slot: String) -> Option<Rc<T>> {
    let base = obj.get_object();
    let slots = base.slots.borrow();
    slots
        .get(&slot)
        .and_then(|x| x.clone().downcast::<T>().ok())
}

pub(crate) fn p262_has_slot(obj: Rc<dyn HasBaseObject>, slot: String) -> bool {
    let base = obj.get_object();
    let slots = base.slots.borrow();
    slots.contains_key(&slot)
}

/// The internal implementation of an ES [Object](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type) value.
#[derive(Debug)]
pub struct ObjectRep(MagicId, pub Rc<dyn 'static + Object>);

impl PartialEq for ObjectRep {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
