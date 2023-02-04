use std::collections::HashMap;

use futures::{
    channel::mpsc::{self, Receiver, Sender},
    SinkExt, StreamExt,
};
use sc_service::SpawnTaskHandle;

use crate::{
    error::{CertificateError, Result, SendError},
    traits::{DigestStore, Hash},
    types::{Certificate, CertificateStore, CertificatesAggregator, Round},
};

/// The default channel capacity for each channel of the primary.
pub const CHANNEL_CAPACITY: usize = 1_000;

pub struct Primary {
    rx_certificates: Receiver<Certificate>,
    certificate_store: CertificateStore,
    gc_round: Round,
    gc_depth: Round,
    highest_received_round: Round,
    highest_processed_round: Round,
    /// Transports certificates to the consensus layer
    tx_new_certificates: Sender<Certificate>,
    /// Aggregates certificates to use as parents for new headers.
    certificates_aggregators: HashMap<Round, Box<CertificatesAggregator<()>>>,
}

impl Primary {
    /// spawn a primary service
    pub fn spawn(spawn_handle: SpawnTaskHandle) {
        let (_, rx_certificates) = mpsc::channel(CHANNEL_CAPACITY);
        let (tx_new_certificates, _) = mpsc::channel(CHANNEL_CAPACITY);

        let gc_depth = 100;

        spawn_handle.spawn("primary", "", async move {
            Self {
                rx_certificates,
                certificate_store: (),
                gc_round: 0,
                gc_depth,
                highest_received_round: 0,
                highest_processed_round: 0,
                tx_new_certificates,
                certificates_aggregators: HashMap::with_capacity(2 * gc_depth as usize),
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
            // duplicate
            return Ok(());
        }
        self.highest_received_round = self.highest_received_round.max(certificate.round());

        // Instruct workers to download any missing batches referenced in this certificate.
        // Since this header got certified, we are sure that all the data it refers to (ie. its batches and its parents) are available.
        // We can thus continue the processing of the certificate without blocking on batch synchronization.
        // TODO

        // Ensure either we have all the ancestors of this certificate, or the parents have been garbage collected.
        // If we don't, the synchronizer will start fetching missing certificates.
        // TODO

        // Store the certificate. Afterwards, the certificate must be sent to consensus
        // or Narwhal needs to shutdown, to avoid insistencies certificate store and
        // consensus dag.
        self.certificate_store.write(certificate.clone())?;

        self.highest_processed_round = self.highest_processed_round.max(certificate.round());

        self.append_certificate_in_aggregator(certificate.clone())
            .await?;

        // Send to the consensus layer
        self.tx_new_certificates
            .send(certificate)
            .await
            .map_err(SendError::CertificateNew)?;

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

    async fn append_certificate_in_aggregator(&self, _certificate: Certificate) -> Result<()> {
        todo!()
    }
}
