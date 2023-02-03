use futures::{
    channel::mpsc::{self, Receiver},
    StreamExt,
};
use sc_service::SpawnTaskHandle;

use crate::{
    error::{CertificateError, Result},
    traits::{DigestStore, Hash},
    types::{Certificate, CertificateStore, Round},
};

/// The default channel capacity for each channel of the primary.
pub const CHANNEL_CAPACITY: usize = 1_000;

pub struct Primary {
    rx_certificates: Receiver<Certificate>,
    certificate_store: CertificateStore,
    gc_round: Round,
}

impl Primary {
    /// spawn a primary service
    pub fn spawn(spawn_handle: SpawnTaskHandle) -> () {
        let (_, rx_certificates) = mpsc::channel(CHANNEL_CAPACITY);

        spawn_handle.spawn("primary", "", async move {
            Self {
                rx_certificates,
                certificate_store: (),
                gc_round: 0,
            }
            .run()
            .await
        })
    }

    async fn run(mut self) {
        match self.run_inner().await {
            Ok(_) => {}
            Err(err) => panic!("{:?}", err),
        }
    }

    async fn run_inner(&mut self) -> Result<()> {
        loop {
            futures::select! {
                certificate = self.rx_certificates.next() =>
                {
                    if let Some(certificate) = certificate {
                        self.process_certificate(certificate).await?
                    }
                }

                complete => continue,
            }
        }
    }

    async fn process_certificate(&mut self, certificate: Certificate) -> Result<()> {
        self.sanitize_certificate(&certificate).await?;
        let digest = certificate.digest();
        if self.certificate_store.read(digest)?.is_some() {}
        Ok(())
    }

    async fn sanitize_certificate(&self, certificate: &Certificate) -> Result<()> {
        if self.gc_round >= certificate.round() {
            return Err(CertificateError::Stale {
                certificate_round: certificate.round(),
                gc_round: self.gc_round,
            }
            .into());
        }
        Ok(())
    }
}
