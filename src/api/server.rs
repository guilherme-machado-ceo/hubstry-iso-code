// src/api/server.rs
use anyhow::Result;
use std::net::SocketAddr;
use tracing::{info, error};

pub async fn start(host: String, port: u16) -> Result<()> {
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    // Create the router
    let app = super::routes::create_router()
        .layer(super::middleware::cors_layer());

    info!("Starting Hubstry CaaS Engine API on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    if let Err(e) = axum::serve(listener, app).await {
        error!("Server error: {}", e);
        return Err(anyhow::anyhow!("Server crashed"));
    }

    Ok(())
}
