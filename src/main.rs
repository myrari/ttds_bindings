use log::{error, info};
use ttds_bindings::{Color, Pane};

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Starting");

    let pane = Pane::init("myra", Color { r: 0, g: 0, b: 0 })
        .await
        .unwrap();

    info!("given pane: {:?}", pane);

    let rect_resp = pane
        .rect(32, 32, 32, 32, Color { r: 255, g: 0, b: 0 })
        .await;

    match rect_resp {
        Ok(resp) => info!("Rect response: {:?}", resp),
        Err(err) => error!("Rect error: {err}"),
    }
}
