use super::{ui_context::UiContext, Dom, EventId, InternalNodeId, Kind, Node};

#[derive(Debug, PartialEq)]
pub(super) struct LayoutTreeLeaf<'a> {
    size: (i32, i32),
    pos: (i32, i32),
    node_id: InternalNodeId,
    inner: &'a Node,
}

#[derive(Debug, PartialEq)]
pub(super) enum LayoutTree<'a> {
    Single(LayoutTreeLeaf<'a>),
    Multiple(LayoutTreeLeaf<'a>, Vec<LayoutTree<'a>>),
}

impl LayoutTree<'_> {
    pub fn draw(&self, ctx: &mut impl UiContext) {
        match self {
            LayoutTree::Single(leaf) => leaf.draw(ctx),
            LayoutTree::Multiple(leaf, children) => {
                leaf.draw(ctx);
                for tree in children {
                    tree.draw(ctx);
                }
            }
        }
    }

    pub fn resolve_click(&self, mouse_pos: (i32, i32)) -> Option<(EventId, InternalNodeId)> {
        match self {
            LayoutTree::Single(leaf) => leaf.resolve_click(mouse_pos),
            LayoutTree::Multiple(leaf, children) => {
                let res = leaf.resolve_click(mouse_pos);
                if res.is_some() {
                    return res;
                }
                for tree in children.iter().rev() {
                    let res = tree.resolve_click(mouse_pos);
                    if res.is_some() {
                        return res;
                    }
                }
                None
            }
        }
    }
}

impl LayoutTreeLeaf<'_> {
    fn draw_border(&self, ctx: &mut impl UiContext) {
        let pos = self.pos;
        let size = (self.size.0 as u32, self.size.1 as u32);
        if let Some(thickness) = self.inner.border_thickness {
            let thickness = thickness as u32;
            let border_color = self.inner.border_color.unwrap_or((255, 255, 255));
            ctx.draw_rect(border_color, pos.0, pos.1, size.0, thickness)
                .unwrap();
            ctx.draw_rect(border_color, pos.0, pos.1, thickness, size.1)
                .unwrap();
            ctx.draw_rect(
                border_color,
                pos.0 + size.0 as i32 - thickness as i32,
                pos.1,
                thickness,
                size.1,
            )
            .unwrap();
            ctx.draw_rect(
                border_color,
                pos.0,
                pos.1 + size.1 as i32 - thickness as i32,
                size.0,
                thickness,
            )
            .unwrap();
        }
        if self.inner.focused {
            let thickness = self.inner.focus_thickness;
            let pos = (self.pos.0 - thickness, self.pos.1 - thickness);
            let thickness = thickness as u32;
            let border_color = self.inner.focus_color;
            let size = (size.0 + thickness * 2, size.1 + thickness * 2);
            ctx.draw_rect(border_color, pos.0, pos.1, size.0, thickness)
                .unwrap();
            ctx.draw_rect(border_color, pos.0, pos.1, thickness, size.1)
                .unwrap();
            ctx.draw_rect(
                border_color,
                pos.0 + size.0 as i32 - thickness as i32,
                pos.1,
                thickness,
                size.1,
            )
            .unwrap();
            ctx.draw_rect(
                border_color,
                pos.0,
                pos.1 + size.1 as i32 - thickness as i32,
                size.0,
                thickness,
            )
            .unwrap();
        }
    }

    pub fn resolve_click(&self, mouse_position: (i32, i32)) -> Option<(EventId, InternalNodeId)> {
        if !self.inner.visible {
            return None;
        }

        let event_id = self.inner.on_click?;

        if !((self.pos.0..self.pos.0 + self.size.0).contains(&mouse_position.0)
            && (self.pos.1..self.pos.1 + self.size.1).contains(&mouse_position.1))
        {
            return None;
        }
        Some((event_id, self.node_id))
    }
    pub fn draw(&self, ctx: &mut impl UiContext) {
        if !self.inner.visible {
            return;
        }
        if let Some(color) = self.inner.background_color {
            ctx.draw_rect(
                color,
                self.pos.0,
                self.pos.1,
                self.size.0 as u32,
                self.size.1 as u32,
            )
            .unwrap();
        }

        match &self.inner.kind {
            Kind::Vert(_) | Kind::Hori(_) | Kind::Stack(_) | Kind::Rect => { /* do nothing */ }
            Kind::Text { text, font } => {
                let font_size = self.inner.font_size.unwrap_or(15);
                let font_id = ctx.load_font(font, font_size).unwrap();
                let text = ctx
                    .render_text(font_id, text, self.inner.color.unwrap_or((255, 255, 255)))
                    .unwrap();
                let offset =
                    self.inner.padding.unwrap_or(0) + self.inner.border_thickness.unwrap_or(0);
                ctx.draw_texture(text.texture, self.pos.0 + offset, self.pos.1 + offset)
                    .unwrap();
            }
            Kind::Image(src) => {
                let texture = ctx.load_texture(src).unwrap();
                let texture_size = ctx.texture_size(texture).unwrap();
                let offset =
                    self.inner.padding.unwrap_or(0) + self.inner.border_thickness.unwrap_or(0);
                ctx.draw_texture_sized(
                    texture,
                    self.pos.0 + offset,
                    self.pos.1 + offset,
                    self.inner.width.unwrap_or(texture_size.0 as i32) as u32,
                    self.inner.height.unwrap_or(texture_size.1 as i32) as u32,
                )
                .unwrap();
            }
        }

        self.draw_border(ctx);
    }
}

pub(super) trait CanCreateLayoutTree {
    fn build_layout_tree<'dom>(
        &'dom self,
        node_id: InternalNodeId,
        dom: &'dom Dom,
        ctx: &mut impl UiContext,
        parent_pos: (i32, i32),
        pos_transformer: &mut dyn TransformersRobotsInDisguise,
    ) -> LayoutTree<'dom>;
}

pub(super) trait TransformersRobotsInDisguise {
    fn pos(&mut self, size: (i32, i32)) -> (i32, i32);
    fn boxed(self) -> Box<dyn TransformersRobotsInDisguise>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

pub(super) struct HoriTransform {
    acc: i32,
    content_size: (i32, i32),
    padding: i32,
    gap: i32,
}
impl HoriTransform {
    fn new(content_size: (i32, i32), padding: i32, gap: i32) -> Self {
        Self {
            acc: 0,
            content_size,
            padding,
            gap,
        }
    }
}
impl TransformersRobotsInDisguise for HoriTransform {
    fn pos(&mut self, child_size: (i32, i32)) -> (i32, i32) {
        let x = self.acc;
        let y = (self.content_size.1 - child_size.1) / 2;
        self.acc += child_size.0 + self.gap;
        (x + self.padding, y)
    }
}

pub(super) struct VertTransform {
    acc: i32,
    content_size: (i32, i32),
    padding: i32,
    gap: i32,
}
impl VertTransform {
    fn new(content_size: (i32, i32), padding: i32, gap: i32) -> Self {
        Self {
            acc: 0,
            content_size,
            padding,
            gap,
        }
    }
}
impl TransformersRobotsInDisguise for VertTransform {
    fn pos(&mut self, child_size: (i32, i32)) -> (i32, i32) {
        let x = (self.content_size.0 - child_size.0) / 2;
        let y = self.acc;
        self.acc += child_size.1 + self.gap;
        (x, y + self.padding)
    }
}

struct StackTransform {
    content_size: (i32, i32),
    padding: i32,
}
impl StackTransform {
    fn new(content_size: (i32, i32), padding: i32) -> Self {
        Self {
            content_size,
            padding,
        }
    }
}
impl TransformersRobotsInDisguise for StackTransform {
    fn pos(&mut self, child_size: (i32, i32)) -> (i32, i32) {
        let x = (self.content_size.0 - child_size.0) / 2;
        let y = (self.content_size.1 - child_size.1) / 2;
        (x, y)
    }
}

pub(super) struct NoTransform;
impl TransformersRobotsInDisguise for NoTransform {
    fn pos(&mut self, _child_size: (i32, i32)) -> (i32, i32) {
        (0, 0)
    }
}

impl CanCreateLayoutTree for Node {
    fn build_layout_tree<'dom>(
        &'dom self,
        node_id: InternalNodeId,
        dom: &'dom Dom,
        ctx: &mut impl UiContext,
        parent_pos: (i32, i32),
        pos_transformer: &mut dyn TransformersRobotsInDisguise,
    ) -> LayoutTree<'dom> {
        if !self.visible {
            return LayoutTree::Single(LayoutTreeLeaf {
                size: (0, 0),
                pos: (0, 0),
                inner: self,
                node_id,
            });
        }
        fn build_leaf<'a>(
            node: &'a Node,
            node_id: InternalNodeId,
            pos_offset: &mut dyn TransformersRobotsInDisguise,
            parent_pos: (i32, i32),
            default_size: (i32, i32),
        ) -> LayoutTreeLeaf<'a> {
            let padding = node.padding.unwrap_or(0) + node.border_thickness.unwrap_or(0);
            let size = (
                node.width.unwrap_or(default_size.0) + padding * 2,
                node.height.unwrap_or(default_size.1) + padding * 2,
            );
            let pos = pos_offset.pos(size);
            let pos = (pos.0 + parent_pos.0, pos.1 + parent_pos.1);
            LayoutTreeLeaf {
                size,
                pos,
                inner: node,
                node_id,
            }
        }

        match &self.kind {
            Kind::Text { text, font } => {
                let font_size = self.font_size.unwrap_or(15);
                let font_id = ctx.load_font(font, font_size).unwrap();
                let size = ctx.text_size(font_id, text).unwrap();
                let size = (size.0 as i32, size.1 as i32);
                let leaf = build_leaf(self, node_id, pos_transformer, parent_pos, size);
                LayoutTree::Single(leaf)
            }
            Kind::Rect | Kind::Image(_) => {
                let leaf = build_leaf(self, node_id, pos_transformer, parent_pos, (0, 0));
                LayoutTree::Single(leaf)
            }
            kind @ (Kind::Hori(children) | Kind::Vert(children) | Kind::Stack(children)) => {
                let padding = self.padding.unwrap_or(0) + self.border_thickness.unwrap_or(0);
                let gap = self.gap.unwrap_or(0);

                let calc_content_size = match kind {
                    Kind::Vert(_) => |acc: (i32, i32), (curr, gap): (LayoutTree, i32)| {
                        let (LayoutTree::Single(leaf) | LayoutTree::Multiple(leaf, _)) = curr;
                        (std::cmp::max(acc.0, leaf.size.0), acc.1 + leaf.size.1 + gap)
                    },
                    Kind::Hori(_) => |acc: (i32, i32), (curr, gap): (LayoutTree, i32)| {
                        let (LayoutTree::Single(leaf) | LayoutTree::Multiple(leaf, _)) = curr;
                        (acc.0 + leaf.size.0 + gap, std::cmp::max(acc.1, leaf.size.1))
                    },
                    Kind::Stack(_) => |acc: (i32, i32), (curr, _gap): (LayoutTree, i32)| {
                        let (LayoutTree::Single(leaf) | LayoutTree::Multiple(leaf, _)) = curr;
                        (
                            std::cmp::max(acc.0, leaf.size.0),
                            std::cmp::max(acc.1, leaf.size.1),
                        )
                    },
                    _ => unreachable!("not matched prior"),
                };

                let size = children
                    .iter()
                    .filter_map(|id| dom.select_node(*id).map(|node| (*id, node)))
                    .map(|(id, node)| {
                        node.build_layout_tree(id, dom, ctx, (0, 0), &mut NoTransform)
                    })
                    .map(|tree| (tree, gap))
                    .fold((0, 0), calc_content_size);

                let size = (
                    self.width.unwrap_or(size.0) + padding * 2,
                    self.height.unwrap_or(size.1) + padding * 2,
                );

                let pos = pos_transformer.pos(size);
                let pos = (pos.0 + parent_pos.0, pos.1 + parent_pos.1);

                let mut transformer = match kind {
                    Kind::Vert(_) => VertTransform::new(size, padding, gap).boxed(),
                    Kind::Hori(_) => HoriTransform::new(size, padding, gap).boxed(),
                    Kind::Stack(_) => StackTransform::new(size, padding).boxed(),
                    _ => unreachable!(),
                };
                let children: Vec<_> = children
                    .iter()
                    .filter_map(|id| dom.select_node(*id).map(|node| (*id, node)))
                    .map(|(id, node)| {
                        node.build_layout_tree(id, dom, ctx, pos, transformer.as_mut())
                    })
                    .collect();

                let leaf = LayoutTreeLeaf {
                    size: (size.0, size.1),
                    pos,
                    inner: self,
                    node_id,
                };

                LayoutTree::Multiple(leaf, children)
            }
        }
    }
}

#[test]
fn troller_no_trolling_min() {
    use crate::ui::ui_context::MockContext as MogContext;
    use pretty_assertions::assert_eq;

    let received = {
        use crate::ui::constructors::*;
        crate::ui::Dom::new(
            Hori([Vert([Rect().padding(8).border_thickness(1)])
                .padding(8)
                .border_thickness(1)])
            .padding(8)
            .border_thickness(1),
        )
    };
    let received = received.build_layout_tree(&mut MogContext);

    let expected = {
        use crate::ui::Kind::*;
        use LayoutTree::*;

        let focus_color = (53, 73, 136);
        let focus_thickness = 4;

        Multiple(
            LayoutTreeLeaf {
                size: (16 + 2 + 16 + 2 + 16 + 2, 16 + 2 + 16 + 2 + 16 + 2),
                pos: (0, 0),
                node_id: InternalNodeId(0),
                inner: Box::leak(Box::new(Node {
                    kind: Hori(vec![InternalNodeId(1)]),
                    parent_id: None,
                    user_id: None,
                    width: None,
                    height: None,
                    on_click: None,
                    background_color: None,
                    color: None,
                    border_thickness: Some(1),
                    padding: Some(8),
                    gap: None,
                    border_color: None,
                    font_size: None,
                    visible: true,
                    focused: false,
                    focus_color,
                    focus_thickness,
                })),
            },
            vec![Multiple(
                LayoutTreeLeaf {
                    size: (16 + 2 + 16 + 2, 16 + 2 + 16 + 2),
                    pos: (8 + 1, 8 + 1),
                    node_id: InternalNodeId(1),
                    inner: Box::leak(Box::new(Node {
                        kind: Vert(vec![InternalNodeId(2)]),
                        parent_id: Some(InternalNodeId(0)),
                        user_id: None,
                        width: None,
                        height: None,
                        on_click: None,
                        background_color: None,
                        color: None,
                        border_thickness: Some(1),
                        padding: Some(8),
                        gap: None,
                        border_color: None,
                        font_size: None,
                        visible: true,
                        focused: false,
                        focus_color,
                        focus_thickness,
                    })),
                },
                vec![Single(LayoutTreeLeaf {
                    size: (16 + 2, 16 + 2),
                    pos: (8 + 1 + 8 + 1, 8 + 1 + 8 + 1),
                    node_id: InternalNodeId(2),
                    inner: Box::leak(Box::new(Node {
                        kind: Rect,
                        parent_id: Some(InternalNodeId(1)),
                        user_id: None,
                        width: None,
                        height: None,
                        on_click: None,
                        background_color: None,
                        color: None,
                        border_thickness: Some(1),
                        padding: Some(8),
                        gap: None,
                        border_color: None,
                        font_size: None,
                        visible: true,
                        focused: false,
                        focus_color,
                        focus_thickness,
                    })),
                })],
            )],
        )
    };

    assert_eq!(received, expected);
}
