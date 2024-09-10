use std::rc::Rc;

use super::{HasBaseObject, StringRep};
use crate::core::p262_get_slot;

pub(crate) fn p262_is_document_dot_all(obj: Rc<dyn HasBaseObject>) -> bool {
    if cfg!(feature = "annex-b") {
        let base = obj.get_object();
        p262_get_slot::<bool>(base, &StringRep::Borrowed("IsHTMLDDA"))
            .map(|is_dda| *is_dda)
            .unwrap_or(false)
    } else {
        false
    }
}
