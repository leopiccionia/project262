use std::rc::Rc;

use super::Value;

/// An [Object](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type) [property](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-property-attributes).
#[derive(Debug, PartialEq)]
pub enum Property {
    /// A [data property](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type).
    Data {
        /// The value retrieved by an access of the property.
        value: Rc<Value>,
        /// If true, the property's [`value`](Self::Data::value) can be re-assigned.
        writable: bool,
        /// If true, the property will be enumerated by a `for` … `in` enumeration.
        enumerable: bool,
        /// If false, neither the property can be deleted, nor it can be converted to an [accessor property](Self::Accessor), nor any other property can be changed (if [`writable`](Self::Data::writable) is true, [`value`](Self::Data::value) and [`writable`](Self::Data::writable) can still be changed).
        configurable: bool,
    },
    /// An [accessor property](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type).
    Accessor {
        /// The getter for the value of the property.
        get: Option<Rc<Value>>,
        /// The setter for the value of the property.
        set: Option<Rc<Value>>,
        /// If true, the property will be enumerated by a `for` … `in` enumeration.
        enumerable: bool,
        /// If false, neither the property can be deleted, nor it can be converted to a [data property](Self::Data), nor any other property attribute can be changed.
        configurable: bool,
    },
}

impl Property {
    /// Returns if property is an [accessor property](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type).
    pub fn is_accessor(&self) -> bool {
        matches!(self, Self::Accessor { .. })
    }

    /// Returns if property is a [data property](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-object-type).
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data { .. })
    }

    /// Returns if property is enumerable.
    pub fn is_enumerable(&self) -> bool {
        match self {
            Self::Accessor { enumerable, .. } => *enumerable,
            Self::Data { enumerable, .. } => *enumerable,
        }
    }

    /// Returns if property is configurable.
    pub fn is_configurable(&self) -> bool {
        match self {
            Self::Accessor { configurable, .. } => *configurable,
            Self::Data { configurable, .. } => *configurable,
        }
    }
}

impl Clone for Property {
    fn clone(&self) -> Self {
        match self {
            Self::Accessor {
                get,
                set,
                enumerable,
                configurable,
            } => Self::Accessor {
                get: get.clone(),
                set: set.clone(),
                enumerable: *enumerable,
                configurable: *configurable,
            },
            Self::Data {
                value,
                writable,
                enumerable,
                configurable,
            } => Self::Data {
                value: value.clone(),
                writable: *writable,
                enumerable: *enumerable,
                configurable: *configurable,
            },
        }
    }
}

/// A [property descriptor](https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html#sec-property-descriptor-specification-type).
///
/// A `Descriptor` is used to define a [`Property`] in an [`Object`](super::Object).
///
/// A `Descriptor` can be easily converted into a `Property`, and vice-versa, via the [`From`] trait.
#[derive(Clone, Debug, Default)]
pub struct Descriptor {
    /// The value retrieved by an access of the property.
    pub value: Option<Rc<Value>>,
    /// If true, the property's value can be re-assigned.
    pub writable: Option<bool>,
    /// The getter for the value of the property.
    pub get: Option<Rc<Value>>,
    /// The getter for the value of the property.
    pub set: Option<Rc<Value>>,
    /// If true, the property will be enumerated by a `for` … `in` enumeration.
    pub enumerable: Option<bool>,
    /// If false, neither the property can be deleted, nor any property attribute can be changed.
    pub configurable: Option<bool>,
}

impl Descriptor {
    pub(crate) fn is_accessor(&self) -> bool {
        self.get.is_some() || self.set.is_some()
    }

    pub(crate) fn is_data(&self) -> bool {
        self.value.is_some() || self.writable.is_some()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.value.is_none()
            && self.writable.is_none()
            && self.get.is_none()
            && self.set.is_none()
            && self.enumerable.is_none()
            && self.configurable.is_none()
    }

    pub(crate) fn is_generic(&self) -> bool {
        !self.is_accessor() && !self.is_data()
    }
}

impl From<Descriptor> for Property {
    fn from(desc: Descriptor) -> Self {
        e262_complete_property_descriptor(desc)
    }
}

impl From<Property> for Descriptor {
    fn from(prop: Property) -> Self {
        match prop {
            Property::Accessor {
                get,
                set,
                enumerable,
                configurable,
            } => Descriptor {
                value: None,
                writable: None,
                get,
                set,
                enumerable: Some(enumerable),
                configurable: Some(configurable),
            },
            Property::Data {
                value,
                writable,
                enumerable,
                configurable,
            } => Descriptor {
                value: Some(value),
                writable: Some(writable),
                get: None,
                set: None,
                enumerable: Some(enumerable),
                configurable: Some(configurable),
            },
        }
    }
}

pub(crate) fn e262_is_accessor_descriptor(desc: Option<Descriptor>) -> bool {
    match desc {
        None => false,
        Some(desc) => desc.is_accessor(),
    }
}

pub(crate) fn e262_is_data_descriptor(desc: Option<Descriptor>) -> bool {
    match desc {
        None => false,
        Some(desc) => desc.is_data(),
    }
}

pub(crate) fn e262_is_generic_descriptor(desc: Option<Descriptor>) -> bool {
    match desc {
        None => false,
        Some(desc) => desc.is_generic(),
    }
}

pub(crate) fn e262_complete_property_descriptor(desc: Descriptor) -> Property {
    if e262_is_accessor_descriptor(Some(desc.clone())) {
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
            enumerable: desc.enumerable.unwrap_or(false),
            configurable: desc.enumerable.unwrap_or(false),
        }
    }
}
