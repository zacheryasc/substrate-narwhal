use std::collections::HashSet;

use crate::{
    error::Error,
    traits::{DigestStore, Hash},
};

/// A certificate of block availability
#[derive(Debug, Clone)]
pub struct Certificate {
    round: Round,
}

impl Certificate {
    pub fn round(&self) -> Round {
        self.round
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

    fn write(&self, _item: Self::Item) -> Result<(), Self::StoreError> {
        Ok(())
    }
}

/// Aggregate certificates and check if we reach a quorum.
pub struct CertificatesAggregator<S: Ord> {
    /// Staked weight in PoS
    weight: S,
    certificates: Vec<Certificate>,
    used: HashSet<PublicKey>,
}

/// The counter for successive synchronization rounds
pub type Round = u64;

/// The public key type for signatures
pub type PublicKey = ();
