use tracing::{instrument, Level};

pub fn test_span() {
    let span = tracing::span!(Level::INFO, "span_name", field1 = 42, field2 = "value",);
    let _a = span.enter();

    tracing::info!("span hello");
    tracing::info!("ssasa")
}

#[instrument]
pub fn test_instrument(name: i32) {
    tracing::info!("hello");
}
