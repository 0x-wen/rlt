use anyhow::{Context, Result};
use async_trait::async_trait;
use clap::Parser;
use log::error;
use reqwest::{Client, Url};
use rlt::{
    cli::BenchCli,
    IterReport, {BenchSuite, IterInfo},
};
use tokio::time::Instant;

#[derive(Parser, Clone)]
pub struct HttpBench {
    /// Target URL.
    pub url: Url,

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
        let resp = client
            .get(self.url.clone())
            .send()
            .await
            .context("Failed to send request")?;
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
    // 添加日志记录
    log::info!("Starting benchmark with options: {:?}", bs.bench_opts);

    match rlt::cli::run(bs.bench_opts, bs).await {
        Ok(_) => log::info!("Benchmark completed successfully."),
        Err(e) => {
            error!("Benchmark failed: {:?}", e);
            return Err(e);
        }
    }
    Ok(())
}
