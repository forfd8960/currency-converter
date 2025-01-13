use currency_converter::routers::get_router;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let addr = format!("0.0.0.0:{}", 8989);

    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    let app = get_router().await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
