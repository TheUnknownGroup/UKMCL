use freya::{borderless::BorderlessPlugin, prelude::*, router::*};
use crate::routes::*;

pub mod components;
pub mod routes;
pub mod layout;

fn main() {
    launch(
        LaunchConfig::new()
            .with_plugin(BorderlessPlugin::new())
            .with_window(WindowConfig::new(app)
                .with_min_size(1366., 768.)
                .with_size(1366., 768.)
                .with_transparency(true)
                .with_decorations(false)
                .with_background(Color::TRANSPARENT)
                .with_title("UKMCL")
                .with_icon(LaunchConfig::window_icon(include_bytes!("../assets/icons/icon.png")))
            )
    );
}

fn app() -> impl IntoElement {   
    Router::<Pages>::new(|| RouterConfig::default())
}
