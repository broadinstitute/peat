use std::collections::BTreeMap;
use crate::value::Value;

pub(crate) type Bindings = BTreeMap<String, Value>;

pub(crate) fn get_empty_bindings() -> Bindings {
    BTreeMap::new()
}
