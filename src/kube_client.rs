use kube::{Client, Config};
use tracing::info;

// Kubernetes client
// Detects cluster or local development

pub async fn create_client() -> anyhow::Result<Client> {

    // Try when it is running in a pod
    let config = match Config::incluster() {
        Ok(config) => {
            info!("🔧 Using config in-cluster");
            config
        }
        Err(_) => {
            info!("🔧 Using config  ~/.kube/config");
            Config::infer().await?
        }
    };

    // Get the client
    let client = Client::try_from(config)?;
    info!("🔧 Kubernetes client connected");

    Ok(client)
}