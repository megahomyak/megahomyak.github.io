pub trait MayBeEmpty {
    fn is_empty(&self) -> bool;
}

pub struct NonEmpty<T>(T);

pub enum CreationError {
    CollectionIsEmpty,
}

impl<T: MayBeEmpty> NonEmpty<T> {
    pub fn new(collection: T) -> Result<Self, CreationError> {
        if collection.is_empty() {
            Err(CreationError::CollectionIsEmpty)
        } else {
            Ok(Self(collection))
        }
    }

    // Note: mutable access is not allowed because it allows to empty the collection
    pub fn contents(&self) -> &T {
        &self.0
    }
}

impl<T> MayBeEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl MayBeEmpty for String {
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}
