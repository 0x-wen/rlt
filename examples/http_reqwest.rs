use anyhow::{Context, Result};
use async_trait::async_trait;
use clap::Parser;
use log::error;
use reqwest::{Client, Url};
use rlt::{
    cli::BenchCli,
    IterReport, {BenchSuite, IterInfo},
};
use serde_json::json;
use tokio::time::Instant;

#[derive(Parser, Clone)]
pub struct HttpBench {
    /// Target URL.
    pub url: Url,

    /// HTTP method (GET or POST).
    #[arg(short, long, default_value = "GET")]
    pub method: String,

    /// JSON data for POST requests.
    #[arg(long)]
    pub data: Option<String>,

    /// Embed BenchCli into this Opts.
    #[command(flatten)]
    pub bench_opts: BenchCli,
}

#[async_trait]
impl BenchSuite for HttpBench {
    type WorkerState = Client;

    async fn state(&self, _: u32) -> Result<Self::WorkerState> {
        Ok(Client::new())
    }

    async fn bench(&mut self, client: &mut Self::WorkerState, _: &IterInfo) -> Result<IterReport> {
        let t = Instant::now();
        // let resp = client
        //     .get(self.url.clone())
        //     .send()
        //     .await
        //     .context("Failed to send request")?;
        let resp = match self.method.to_uppercase().as_str() {
            "POST" => {
                let data = self.data.clone().unwrap_or_default();
                let body = json!(data);
                client
                    .post(self.url.clone())
                    .json(&body)
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .context("Failed to send POST request")?
            }
            _ => client
                .get(self.url.clone())
                .send()
                .await
                .context("Failed to send GET request")?,
        };
        let status = resp.status().into();
        let bytes = resp.bytes().await.context("Failed to read response bytes")?.len() as u64;
        let duration = t.elapsed();
        // println!("status: {:?}, bytes: {:?}, duration: {:?}", status, bytes, duration);
        Ok(IterReport { duration, status, bytes, items: 1 })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let bs = HttpBench::parse();

    match rlt::cli::run(bs.bench_opts, bs).await {
        Ok(_) => log::info!("Benchmark completed successfully."),
        Err(e) => {
            error!("Benchmark failed: {:?}", e);
            return Err(e);
        }
    }
    Ok(())
}

