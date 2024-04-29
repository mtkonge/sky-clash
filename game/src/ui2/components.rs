use super::{builder, constructors::Text};

#[allow(non_snake_case)]
pub fn Button<S: Into<String>>(text: S) -> builder::Box<builder::Node> {
    Text(text)
        .with_padding(15)
        .with_border_thickness(2)
        .with_border_color((255, 255, 255))
}
