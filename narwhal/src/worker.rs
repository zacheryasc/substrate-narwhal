use sc_service::SpawnTaskHandle;

pub struct Worker;

impl Worker {
    // spawn a primary service
    pub fn spawn(spawn_handle: SpawnTaskHandle) -> () {
        spawn_handle.spawn("worker", "", async move { Self {}.run().await })
    }

    async fn run(self) -> () {
        loop {}
    }
}
