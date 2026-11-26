// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Closed Circuit Consulting

use anyhow::{Context, Result};
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tokio::signal::unix::{SignalKind, signal};

const BANNER: &str = r#"
          ######   ######   ######  
         ##    ## ##    ## ##    ## 
         ##       ##       ##       
         ##       ##       ##       
         ##       ##       ##       
         ##    ## ##    ## ##    ## 
          ######   ######   ######  

          Closed Circuit Consulting
               Twin Cities, MN            
  
  https://git.closedcircuitconsulting.com
  https://github.com/closedcircuitconsulting

"#;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let app = Router::new().route("/", get(|| async { BANNER }));
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("binding to {addr}"))?;

    let mut sigterm = signal(SignalKind::terminate()).context("creating SIGTERM handler")?;
    let mut sigint = signal(SignalKind::interrupt()).context("creating SIGINT handler")?;

    if let Err(e) = sd_notify::notify(true, &[sd_notify::NotifyState::Ready]) {
        eprintln!("warning: failed to notify systemd: {e}");
    }

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            tokio::select! {
                _ = sigterm.recv() => {}
                _ = sigint.recv() => {}
            }
        })
        .await
        .context("server error")?;

    Ok(())
}
