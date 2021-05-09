use crate::code::value::Value;
use crate::util::matryoshka;
use crate::util::error::Error;

pub(crate) type Bindings = matryoshka::MatryoshkaMap<String, Value>;
pub(crate) type BindingsIterator<'a> = Box<dyn Iterator<Item=Result<Bindings, Error>> + 'a>;

pub(crate) fn get_empty_bindings() -> Bindings {
    matryoshka::MatryoshkaMap::new()
}

pub(crate) fn new_bindings_iter<'a>() -> BindingsIterator<'a> {
    Box::new(std::iter::once(Ok(get_empty_bindings())))
}