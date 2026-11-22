pub mod main_page;
pub mod pages;
pub mod route;
pub mod instances;
pub mod settings;
pub mod store;

pub use main_page::home;
pub use route::*;
pub use pages::Pages;
pub use instances::instances;
pub use store::store;
pub use settings::settings;