use crate::value::Value;
use crate::matryoshka;

pub(crate) type Bindings = matryoshka::MatryoshkaMap<String, Value>;

pub(crate) fn get_empty_bindings() -> Bindings {
    matryoshka::MatryoshkaMap::new()
}
