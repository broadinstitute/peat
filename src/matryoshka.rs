use std::rc::Rc;

pub(crate) enum MatryoshkaMap<K: Eq + Clone, V: Clone> {
    Empty,
    Wrap(Layer<K, V>),
}

pub(crate) struct Layer<K: Eq + Clone, V: Clone> {
    inner: Rc<MatryoshkaMap<K, V>>,
    key: K,
    value: V,
}

struct WithValuesIterator<K: Eq + Clone, V: Clone> {
    inner: Rc<MatryoshkaMap<K, V>>,
    key: K,
    value_iter: Box<dyn Iterator<Item=V>>
}

impl<K: Eq + Clone, V: Clone> MatryoshkaMap<K, V> {
    pub(crate) fn new() -> MatryoshkaMap<K, V> { MatryoshkaMap::Empty }

    pub(crate) fn to_vec(&self) -> Vec<(K, V)> {
        match self {
            MatryoshkaMap::Empty => Vec::new(),
            MatryoshkaMap::Wrap(layer) => {
                let mut inner_vec = layer.inner.to_vec();
                inner_vec.push((layer.key.clone(), layer.value.clone()));
                inner_vec
            }
        }
    }

    pub(crate) fn get(&self, key: &K) -> Option<V> {
        match self {
            MatryoshkaMap::Empty => None,
            MatryoshkaMap::Wrap(layer) => {
                if layer.key == *key {
                    Option::Some(layer.value.clone())
                } else {
                    layer.inner.get(key)
                }
            }
        }
    }

    pub(crate) fn with_value(self, key: K, value: V) -> MatryoshkaMap<K, V> {
        MatryoshkaMap::Wrap(Layer { inner: Rc::new(self), key, value })
    }

    pub(crate) fn recursive_map<'a, T, F>(&self, zero: &'a T, f: &F) -> &'a T
        where F: Fn(&'a T, &K, &V) -> &'a T
    {
        match self {
            MatryoshkaMap::Empty => zero,
            MatryoshkaMap::Wrap(layer) => {
                let previous = layer.inner.recursive_map(zero, &f);
                f(&previous, &layer.key, &layer.value)
            }
        }
    }
}

impl<K: Eq + Clone, V: Clone> Iterator for WithValuesIterator<K, V> {
    type Item = MatryoshkaMap<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.value_iter.next().map(|value|{
           MatryoshkaMap::Wrap(Layer { inner: self.inner.clone(), key: self.key.clone(), value })
        })
    }
}