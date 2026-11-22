use freya::{prelude::*, router::Outlet};
use utils::C;
use crate::{components::*, routes::*};

pub fn layout() -> impl IntoElement {
    rect()
        .expanded()
        .horizontal()
        .background(C::TB.value())
        .corner_radius(CornerRadius::new_all(8.))
        .child(panel())
        .child(
            rect()
                .expanded()
                .vertical()
                .background(C::TB.value())
                .corner_radius(CornerRadius::new_all(8.))
                .child(title_bar())
                .child(rect()
                    .expanded()
                    .background(C::TB.value())
                    .corner_radius(CornerRadius::new_all(8.))
                    .child(rect()
                        .expanded()
                        .background(C::BG.value())
                        .corner_radius(CornerRadius::new(8., 0., 8., 0.))
                        .child(Outlet::<Pages>::new()))))
        .rounded_lg()
}