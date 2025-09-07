use axum::{
    extract::State,
    routing::get,
    Router,
    Json,
};
use std::net::SocketAddr;
use std::env;
use tracing::info;

// Import own módulos
mod models;
mod kube_client;
mod health_checker;

use models::{ApiInfo, ClusterSummary};

const VERSION: &str = "0.1.0";
const API_NAME: &str = "Health Dashboard API";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Port 3000
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT should be a number");

    // Create client of Kubernetes
    let kube_client = kube_client::create_client().await?;

    // Create router with sharing state
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/pods", get(get_pods))  // NUEVA RUTA
        .with_state(kube_client);  // Compartir el cliente

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> Json<ApiInfo> {
    Json(ApiInfo {
        name: API_NAME.to_string(),
        version: VERSION.to_string(),
        status: "running".to_string(),
    })
}

async fn health_check() -> Json<ApiInfo> {
    Json(ApiInfo {
        name: API_NAME.to_string(),
        version: VERSION.to_string(),
        status: "healthy".to_string(),
    })
}

// Nwe handler State<Client>
async fn get_pods(
    State(client): State<kube::Client>
) -> Result<Json<ClusterSummary>, String> {
    match health_checker::get_cluster_pods(client).await {
        Ok(summary) => Ok(Json(summary)),
        Err(e) => {
            tracing::error!("Error getting pods: {}", e);
            Err(format!("Error: {}", e))
        }
    }
}