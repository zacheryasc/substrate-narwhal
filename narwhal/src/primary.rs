use futures::{
    channel::mpsc::{self, Receiver, Sender},
    SinkExt, StreamExt,
};
use sc_service::SpawnTaskHandle;

use crate::{
    error::{from_error, CertificateError, Result, Error},
    traits::{DigestStore, Hash},
    types::{Certificate, CertificateStore, Epoch, Round},
};

/// The default channel capacity for each channel of the primary.
pub const CHANNEL_CAPACITY: usize = 1_000;

pub struct Primary {
    rx_certificates: Receiver<Certificate>,
    certificate_store: CertificateStore,
    gc_round: Round,
    highest_received_round: Round,
    /// Send valid a quorum of certificates' ids to the `Proposer` (along with their round).
    tx_parents: Sender<(Vec<Certificate>, Round, Epoch)>,
}

impl Primary {
    /// spawn a primary service
    pub fn spawn(spawn_handle: SpawnTaskHandle) -> () {
        let (_, rx_certificates) = mpsc::channel(CHANNEL_CAPACITY);
        let (tx_parents, _) = mpsc::channel(CHANNEL_CAPACITY);

        spawn_handle.spawn("primary", "", async move {
            Self {
                rx_certificates,
                certificate_store: (),
                gc_round: 0,
                highest_received_round: 0,
                tx_parents,
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
        if self.certificate_store.read(digest)?.is_some() {
            // Certificate has already been processed
            return Ok(());
        }

        self.highest_received_round = self.highest_received_round.max(certificate.round());

        // NOTE: Assumes cert is well signed
        let minimal_round_for_parents = certificate.round().saturating_sub(1);
        self.tx_parents
            .send((vec![], minimal_round_for_parents, certificate.epoch()))
            .await
            .map_err(from_error!(Error::Sending))?;

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
