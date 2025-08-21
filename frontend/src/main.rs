use app::App;
use tracing::{info, Level as TracingLevel};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::layer as tracing_layer;
use tracing_subscriber::{prelude::*, registry as tracing_registry};
use tracing_web::MakeWebConsoleWriter;
use yew::Renderer;

mod app;

fn main() {
    let fmt_layer = tracing_layer()
        .with_ansi(true)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new())
        .with_filter(
            Targets::new()
                .with_target("yew", TracingLevel::DEBUG)
                .with_default(TracingLevel::TRACE),
        );

    tracing_registry()
        .with(fmt_layer)
        .init();

    info!("Starting Yew application");

    Renderer::<App>::new()
        .render();
}

