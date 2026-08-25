mod calculator;
mod arithmetic;
mod fft;
mod domain_error;
mod bignum;
pub mod test;

use calculator::Calculator;

use rmcp::transport;
use rmcp::ServiceExt;

use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
};

const BIND_ADDRESS: &str = "127.0.0.1";
enum Protocol {
    StdIo,
    Http,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let args = std::env::args().collect::<Vec<_>>();
    let protocol = match args.get(1) {
        Some(s) => match s.as_str() {
            "stdio" => Protocol::StdIo,
            _ => Protocol::Http,
        },
        None => Protocol::StdIo,
    };

    match protocol {
        Protocol::StdIo => {
            let service = Calculator.serve(transport::stdio()).await?;
            service.waiting().await?;
        }
        Protocol::Http => {
            let ct = tokio_util::sync::CancellationToken::new();

            let service = StreamableHttpService::new(
                || Ok(Calculator),
                LocalSessionManager::default().into(),
                StreamableHttpServerConfig::default().with_cancellation_token(ct.child_token()),
            );

            let router = axum::Router::new().nest_service("/mcp", service);
            let tcp_listener = tokio::net::TcpListener::bind(BIND_ADDRESS).await?;
            let _ = axum::serve(tcp_listener, router)
              .with_graceful_shutdown(async move {
                  tokio::signal::ctrl_c().await.unwrap();
                  ct.cancel();
              })
              .await;
        }
    }
    Ok(())
}
