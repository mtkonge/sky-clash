pub fn change_text_node_content<S: Into<String>>(node: Option<&mut super::Node>, new_text: S) {
    let Some(super::Node {
        kind: super::Kind::Text { ref mut text, .. },
        ..
    }) = node
    else {
        println!("ui warning: tried to change text of non-text node");
        return;
    };
    *text = new_text.into();
}

pub fn change_image_node_content<P: Into<std::path::PathBuf>>(
    node: Option<&mut super::Node>,
    new_path: P,
) {
    let Some(super::Node {
        kind: super::Kind::Image(ref mut image),
        ..
    }) = node
    else {
        println!("ui warning: tried to change texture of non-image node");
        return;
    };
    *image = new_path.into();
}
