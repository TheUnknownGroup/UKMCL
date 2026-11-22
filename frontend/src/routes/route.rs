use freya::{prelude::*};

use crate::{layout::layout, routes::*};

#[derive(PartialEq, Clone)]
pub struct AppLayout;
#[derive(PartialEq, Clone)]
pub struct Home;
#[derive(PartialEq, Clone)]
pub struct Instances;
#[derive(PartialEq, Clone)]
pub struct Store;
#[derive(PartialEq, Clone)]
pub struct Settings;

impl Component for AppLayout {
    fn render(&self) -> impl IntoElement {
        layout()
    }
}

impl Component for Home {
    fn render(&self) -> impl IntoElement {
        home()
    }
}

impl Component for Instances {
    fn render(&self) -> impl IntoElement {
        instances()
    }
}

impl Component for Store {
    fn render(&self) -> impl IntoElement {
        store()
    }
}

impl Component for Settings {
    fn render(&self) -> impl IntoElement {
        settings()
    }
}