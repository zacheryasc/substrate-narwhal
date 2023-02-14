use crate::{
    error::Error,
    traits::{DigestStore, Hash},
};

/// A certificate of block availability
pub struct Certificate {
    round: Round,
    epoch: Epoch,
}

impl Certificate {
    pub fn round(&self) -> Round {
        self.round
    }
    pub fn epoch(&self) -> Round {
        self.epoch
    }
}

/// The hash type that the certificate is processed into
pub type CertificateDigest = ();

impl Hash for Certificate {
    type Digest = CertificateDigest;
    fn digest(&self) -> Self::Digest {
        ()
    }
}

/// The database type for storing certificates
pub type CertificateStore = ();

impl DigestStore for CertificateStore {
    type Item = Certificate;
    type Digest = CertificateDigest;
    type StoreError = Error;

    fn read(&self, _digest: CertificateDigest) -> Result<Option<Certificate>, Error> {
        Ok(None)
    }
}

/// The epoch number
pub type Epoch = u64;

/// The counter for successive synchronization rounds
pub type Round = u64;
