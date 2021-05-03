pub(crate) enum MatryoshkaMap<K: Eq + Clone, V: Clone> {
    Empty,
    Wrap(Layer<K, V>),
}

pub(crate) struct Layer<K: Eq + Clone, V: Clone> {
    inner: Box<MatryoshkaMap<K, V>>,
    key: K,
    value: V,
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

    pub(crate) fn with(self, key: K, value: V) -> MatryoshkaMap<K, V> {
        MatryoshkaMap::Wrap(Layer{ inner: Box::new(self), key, value })
    }

}