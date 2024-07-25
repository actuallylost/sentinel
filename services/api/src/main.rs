use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{routing::get, Extension, Router};
use prisma::PrismaClient;

#[tokio::main]
async fn main() -> Result<()> {
    // create prisma client
    let client = Arc::new(
        PrismaClient::_builder()
            .build()
            .await
            .context("failed to build prisma client"),
    );

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(Extension(client));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
