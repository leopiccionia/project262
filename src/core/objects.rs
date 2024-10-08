use ordermap::OrderMap;
use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use super::id::MagicId;
use super::property::Descriptor;
use super::test::e262_same_value;
use super::{Property, SymbolRep, Value};
use crate::errors::CoreResult;

/// An [Object](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type) property key.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PropertyKey {
    /// A [String](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-string-type) key.
    String(String),
    /// A [Symbol](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-ecmascript-language-types-symbol-type) key.
    Symbol(SymbolRep),
}

/// Implements the internal methods of an [Object](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type).
///
/// The default implementation of those methods are defined by the [`BaseObject`] struct, and other structs can leverage them via the [`HasBaseObject`] trait, but one or more internal methods can be overriden by [exotic objects](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#exotic-object).
pub trait Object: Debug {
    /// Get an object slot.
    ///
    /// Due to [object safety constraints](https://doc.rust-lang.org/reference/items/traits.html#object-safety), it returns the slot as [`Any`].
    /// For getting typed slots, use the [`p262_get_slot`] function.
    fn get_slot(self: Rc<Self>, key: String) -> Option<Rc<dyn Any>>;

    /// Set an object slot.
    fn set_slot(self: Rc<Self>, key: String, value: Rc<dyn Any>) -> bool;

    /// Implements the [`[[GetPrototypeOf]]`](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-invariants-of-the-essential-internal-methods) internal method.
    fn get_prototype_of(self: Rc<Self>) -> CoreResult<Option<ObjectRep>>;

    ///Implements the [`[[SetPrototypeOf]]`](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-invariants-of-the-essential-internal-methods) internal method.
    fn set_prototype_of(self: Rc<Self>, proto: Option<ObjectRep>) -> bool;

    /// Implements the [`[[IsExtensible]]`](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-invariants-of-the-essential-internal-methods) internal method.
    fn is_extensible(self: Rc<Self>) -> CoreResult<bool>;

    /// Implements the [`[[PreventExtensions]]`](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-invariants-of-the-essential-internal-methods) internal method.
    fn prevent_extensions(self: Rc<Self>) -> CoreResult<bool>;

    ///Implements the [`[[GetOwnProperty]]`](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-invariants-of-the-essential-internal-methods) internal method.
    fn get_own_property(self: Rc<Self>, key: &PropertyKey) -> CoreResult<Option<Property>>;

    ///Implements the [`[[DefineOwnProperty]]`](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-invariants-of-the-essential-internal-methods) internal method.
    fn define_own_property(self: Rc<Self>, key: PropertyKey, desc: Descriptor) -> CoreResult<bool>;

    ///Implements the [`[[HasProperty]]`](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-invariants-of-the-essential-internal-methods) internal method.
    fn has_property(self: Rc<Self>, key: &PropertyKey) -> CoreResult<bool>;

    // fn get(self: Rc<Self>, key: &PropertyKey, receiver: Value) -> Value;

    // fn set(self: Rc<Self>, key: &PropertyKey, value: Value, receiver: Value) -> bool;

    /// Implements the [`[[Delete]]`](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-invariants-of-the-essential-internal-methods) internal method.
    fn delete(self: Rc<Self>, key: &PropertyKey) -> CoreResult<bool>;

    // fn own_property_keys(self: Rc<Self>) -> Vec<&PropertyKey>;
}

/// The internal implementation for an ES [ordinary object](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#ordinary-object).
#[derive(Debug)]
pub struct BaseObject {
    id: MagicId,
    props: RefCell<OrderMap<PropertyKey, Property>>,
    slots: RefCell<HashMap<String, Rc<dyn 'static + Any>>>,
    prototype: RefCell<Option<ObjectRep>>,
    extensible: Cell<bool>,
}

impl BaseObject {
    fn new(prototype: &Option<ObjectRep>) -> Self {
        BaseObject {
            id: MagicId::new(),
            props: RefCell::new(OrderMap::new()),
            slots: RefCell::new(HashMap::new()),
            prototype: RefCell::new(prototype.clone()),
            extensible: Cell::new(true),
        }
    }
}

impl Object for BaseObject {
    fn get_slot(self: Rc<Self>, key: String) -> Option<Rc<dyn Any>> {
        let slots = self.slots.borrow();
        slots.get(&key).cloned()
    }

    fn set_slot(self: Rc<Self>, key: String, value: Rc<dyn Any>) -> bool {
        let mut slots = self.slots.borrow_mut();
        slots.insert(key, value);
        true
    }

    fn get_prototype_of(self: Rc<Self>) -> CoreResult<Option<ObjectRep>> {
        Ok(e262_ordinary_get_prototype_of(self))
    }

    fn set_prototype_of(self: Rc<Self>, proto: Option<ObjectRep>) -> bool {
        e262_ordinary_set_prototype_of(self, proto)
    }

    fn is_extensible(self: Rc<Self>) -> CoreResult<bool> {
        Ok(e262_ordinary_is_extensible(self))
    }

    fn prevent_extensions(self: Rc<Self>) -> CoreResult<bool> {
        Ok(e262_ordinary_prevent_extensions(self))
    }

    fn get_own_property(self: Rc<Self>, key: &PropertyKey) -> CoreResult<Option<Property>> {
        Ok(e262_ordinary_get_own_property(self, key))
    }

    fn define_own_property(self: Rc<Self>, key: PropertyKey, desc: Descriptor) -> CoreResult<bool> {
        e262_ordinary_define_own_property(self, &key, desc)
    }

    fn has_property(self: Rc<Self>, key: &PropertyKey) -> CoreResult<bool> {
        e262_ordinary_has_property(self, key)
    }

    fn delete(self: Rc<Self>, key: &PropertyKey) -> CoreResult<bool> {
        e262_ordinary_delete(self, key)
    }
}

/// Gets a [`BaseObject`] from an [ordinary](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#ordinary-object) or [exotic](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#exotic-object) Object implementation.
///
/// It's useful for easily implementing all the methods in [`Object`] trait.
#[allow(clippy::too_long_first_doc_paragraph)]
pub trait HasBaseObject: Object {
    /// Gets a reference to a [`BaseObject`].
    fn get_object(self: Rc<Self>) -> Rc<BaseObject>;
}

impl HasBaseObject for BaseObject {
    fn get_object(self: Rc<Self>) -> Rc<BaseObject> {
        self
    }
}

pub(crate) fn e262_is_extensible(obj: Rc<dyn Object>) -> CoreResult<bool> {
    Object::is_extensible(obj.clone())
}

pub(crate) fn e262_ordinary_define_own_property(
    obj: Rc<dyn HasBaseObject>,
    key: &PropertyKey,
    desc: Descriptor,
) -> CoreResult<bool> {
    let current = Object::get_own_property(obj.clone(), key)?;
    let extensible = e262_is_extensible(obj.clone())?;
    Ok(e262_validate_and_apply_property_descriptor(
        Some(obj),
        key,
        extensible,
        &desc,
        current,
    ))
}

pub(crate) fn e262_ordinary_delete(
    obj: Rc<dyn HasBaseObject>,
    key: &PropertyKey,
) -> CoreResult<bool> {
    let prop = Object::get_own_property(obj.clone(), key);
    match prop {
        Err(err) => Err(err),
        Ok(None) => Ok(true),
        Ok(Some(prop)) => {
            if prop.is_configurable() {
                let base = obj.get_object();
                base.props.borrow_mut().remove(key);
                Ok(true)
            } else {
                Ok(false)
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

pub(crate) fn e262_ordinary_get_prototype_of(obj: Rc<dyn HasBaseObject>) -> Option<ObjectRep> {
    let base = obj.get_object();
    let proto = base.prototype.borrow();
    proto.clone()
}

pub(crate) fn e262_ordinary_has_property(
    obj: Rc<dyn HasBaseObject>,
    key: &PropertyKey,
) -> CoreResult<bool> {
    let base = obj.get_object();
    let has_own = base.clone().get_own_property(key)?;
    match has_own {
        Some(_) => Ok(true),
        None => {
            let parent = base.get_prototype_of()?;
            match parent {
                Some(parent) => parent.0.clone().has_property(key),
                None => Ok(false),
            }
        }
    }
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

pub(crate) fn e262_is_compatible_property_descriptor(
    extensible: bool,
    desc: &Descriptor,
    current: Option<Property>,
) -> bool {
    e262_validate_and_apply_property_descriptor(
        None,
        &PropertyKey::String("".into()),
        extensible,
        desc,
        current,
    )
}

pub(crate) fn e262_ordinary_set_prototype_of(
    obj: Rc<dyn HasBaseObject>,
    proto: Option<ObjectRep>,
) -> bool {
    let base = obj.get_object();
    let base_id = base.id;
    let current = base.prototype.borrow_mut();
    if *current == proto {
        true
    } else {
        let mut found_protos: HashSet<MagicId> = HashSet::new();

        if !base.extensible.get() {
            return false;
        }

        let mut p: Option<ObjectRep> = proto;
        let mut done = false;
        while !done {
            match &p {
                None => {
                    done = true;
                }
                Some(rep) => {
                    let curr_id = rep.clone().0.get_object().id;

                    if curr_id == base_id {
                        return false;
                    } else if found_protos.contains(&curr_id) {
                        done = true; // @TODO
                    } else {
                        found_protos.insert(curr_id);
                        p = rep.clone().0.get_object().prototype.borrow().clone();
                    }
                }
            }
        }
        base.prototype.replace(p);
        true
    }
}

#[allow(clippy::if_same_then_else)]
pub(crate) fn e262_validate_and_apply_property_descriptor(
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
                None => return true,
                Some(obj) => {
                    let base = obj.get_object();
                    let prop: Property = desc.clone().into();
                    let mut props = base.props.borrow_mut();
                    props.insert(key.clone(), prop);
                    return true;
                }
            }
        }
        Some(current) => {
            let desc = desc.clone();
            if desc.is_empty() {
                return true;
            } else if !current.is_configurable() {
                if desc.configurable == Some(true) {
                    return false;
                } else if desc.enumerable == Some(current.is_enumerable()) {
                    return false;
                } else if !desc.is_generic() && (desc.is_accessor() != current.is_accessor()) {
                    return false;
                }
                let undefined = Rc::new(Value::Undefined);
                match &current {
                    Property::Accessor {
                        get: current_get,
                        set: current_set,
                        ..
                    } => {
                        if desc.get.is_some()
                            && !e262_same_value(
                                &desc.get.clone().unwrap(),
                                &current_get.clone().unwrap_or(undefined.clone()),
                            )
                        {
                            return false;
                        }
                        if desc.set.is_some()
                            && !e262_same_value(
                                &desc.set.clone().unwrap(),
                                &current_set.clone().unwrap_or(undefined.clone()),
                            )
                        {
                            return false;
                        }
                    }
                    Property::Data {
                        value: current_value,
                        writable: current_writable,
                        ..
                    } => {
                        if !current_writable {
                            if desc.writable == Some(true) {
                                return false;
                            }
                            if desc.value.is_some()
                                && !e262_same_value(&desc.value.clone().unwrap(), current_value)
                            {
                                return false;
                            }
                        }
                    }
                }
            }
            if let Some(obj) = obj {
                let prop = match &current {
                    Property::Accessor {
                        enumerable,
                        configurable,
                        ..
                    } => {
                        if desc.is_accessor() {
                            Property::Accessor {
                                get: desc.get,
                                set: desc.set,
                                enumerable: desc.enumerable.unwrap_or(false),
                                configurable: desc.configurable.unwrap_or(false),
                            }
                        } else {
                            Property::Data {
                                value: desc.value.unwrap_or_else(|| Rc::new(Value::Undefined)),
                                writable: desc.writable.unwrap_or(false),
                                enumerable: desc.enumerable.unwrap_or(*enumerable),
                                configurable: desc.configurable.unwrap_or(*configurable),
                            }
                        }
                    }
                    Property::Data {
                        enumerable,
                        configurable,
                        ..
                    } => {
                        if desc.is_data() {
                            Property::Data {
                                value: desc.value.unwrap_or_else(|| Rc::new(Value::Undefined)),
                                writable: desc.writable.unwrap_or(false),
                                enumerable: desc.enumerable.unwrap_or(false),
                                configurable: desc.configurable.unwrap_or(false),
                            }
                        } else {
                            Property::Accessor {
                                get: desc.get,
                                set: desc.set,
                                enumerable: desc.enumerable.unwrap_or(*enumerable),
                                configurable: desc.configurable.unwrap_or(*configurable),
                            }
                        }
                    }
                };
                let base = obj.get_object();
                let mut props = base.props.borrow_mut();
                props.insert(key.to_owned(), prop);
            }
        }
    }
    true
}

/// Retrieves a slot from the [`Object`], if it exists and matches the provided type.
///
/// Prefer it over the lower-level [`Object::get_slot`] because of the typed return value.
pub fn p262_get_slot<T: 'static>(obj: Rc<dyn Object>, key: String) -> Option<Rc<T>> {
    let slot = obj.get_slot(key);
    slot.and_then(|x| x.downcast::<T>().ok())
}

/// Returns if the [`Object`] has a matching slot.
pub fn p262_has_slot(obj: Rc<dyn Object>, key: String) -> bool {
    let slot = obj.get_slot(key);
    slot.is_some()
}

/// The internal implementation of an ES [Object](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type) value.
#[derive(Clone, Debug)]
pub struct ObjectRep(pub Rc<dyn 'static + HasBaseObject>);

impl ObjectRep {
    /// Create a new [`ObjectRep`] from an [`HasBaseObject`]
    pub fn new(rc: Rc<dyn 'static + HasBaseObject>) -> Self {
        ObjectRep(rc.clone())
    }
}

impl PartialEq for ObjectRep {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
