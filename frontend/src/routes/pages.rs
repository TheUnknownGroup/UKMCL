use crate::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Pages {
    #[layout(AppLayout)]
        #[route("/")]
        Home,
        #[route("/instances")]
        Instances,
        #[route("/store")]
        Store,
        #[route("/settings")]
        Settings,
}