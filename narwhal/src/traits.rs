pub trait Hash {
    type Digest;
    fn digest(&self) -> Self::Digest;
}

pub trait DigestStore {
    type Digest;
    type Item;
    type StoreError;

    fn read(&self, digest: Self::Digest) -> Result<Option<Self::Item>, Self::StoreError>;
    fn write(&self, item: Self::Item) -> Result<(), Self::StoreError>;
}
