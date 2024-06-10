use crate::{max, V2};

use super::{ui_context::UiContext, Dom, EventId, InternalNodeId, Kind, Node};

#[derive(Debug, PartialEq)]
pub(super) struct LayoutTreeLeaf<'a> {
    size: V2,
    pos: V2,
    node_id: InternalNodeId,
    inner: &'a Node,
}

#[derive(Debug, PartialEq)]
pub(super) struct LayoutTree<'a> {
    leaf: LayoutTreeLeaf<'a>,
    children: Vec<LayoutTree<'a>>,
}

impl LayoutTree<'_> {
    pub fn single<'a>(leaf: LayoutTreeLeaf<'a>) -> LayoutTree<'a> {
        LayoutTree {
            leaf,
            children: Vec::new(),
        }
    }
    pub fn multiple<'a>(leaf: LayoutTreeLeaf<'a>, children: Vec<LayoutTree<'a>>) -> LayoutTree<'a> {
        LayoutTree { leaf, children }
    }
    pub fn draw(&self, ctx: &mut impl UiContext) {
        self.leaf.draw(ctx);
        for tree in &self.children {
            tree.draw(ctx);
        }
    }

    pub fn resolve_click(&self, mouse_pos: V2) -> Option<(EventId, InternalNodeId)> {
        let res = self.leaf.resolve_click(mouse_pos);
        if res.is_some() {
            return res;
        }
        for tree in self.children.iter().rev() {
            let res = tree.resolve_click(mouse_pos);
            if res.is_some() {
                return res;
            }
        }
        None
    }
}

impl LayoutTreeLeaf<'_> {
    fn draw_border(&self, ctx: &mut impl UiContext) {
        let pos = self.pos;
        let size = self.size;
        if let Some(thickness) = self.inner.border_thickness {
            let border_color = self.inner.border_color.unwrap_or((255, 255, 255));
            ctx.draw_rect(border_color, pos, V2::new(size.x, thickness))
                .unwrap();
            ctx.draw_rect(border_color, pos, V2::new(thickness, size.y))
                .unwrap();
            ctx.draw_rect(
                border_color,
                V2::new(pos.x + size.x - thickness, pos.y),
                V2::new(thickness, size.y),
            )
            .unwrap();
            ctx.draw_rect(
                border_color,
                V2::new(pos.x, pos.y + size.y - thickness),
                V2::new(size.x, thickness),
            )
            .unwrap();
        }
        if self.inner.focused {
            let thickness = self.inner.focus_thickness;
            let pos = V2::new(self.pos.x - thickness, self.pos.y - thickness);
            let border_color = self.inner.focus_color;
            let size = size + V2::new(thickness, thickness).extend(2.0);
            ctx.draw_rect(border_color, pos, V2::new(size.x, thickness))
                .unwrap();
            ctx.draw_rect(border_color, pos, V2::new(thickness, size.y))
                .unwrap();
            ctx.draw_rect(
                border_color,
                V2::new(pos.x + size.x - thickness, pos.y),
                V2::new(thickness, size.y),
            )
            .unwrap();
            ctx.draw_rect(
                border_color,
                V2::new(pos.x, pos.y + size.y - thickness),
                V2::new(size.x, thickness),
            )
            .unwrap();
        }
    }

    pub fn resolve_click(&self, mouse_position: V2) -> Option<(EventId, InternalNodeId)> {
        if !self.inner.visible {
            return None;
        }

        let event_id = self.inner.on_click?;

        if !((self.pos.x..self.pos.x + self.size.x).contains(&mouse_position.x)
            && (self.pos.y..self.pos.y + self.size.y).contains(&mouse_position.y))
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
            ctx.draw_rect(color, self.pos, self.size).unwrap();
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
                    self.inner.padding.unwrap_or(0.0) + self.inner.border_thickness.unwrap_or(0.0);
                ctx.draw_texture(text.texture, self.pos + V2::new(offset, offset))
                    .unwrap();
            }
            Kind::Image(src) => {
                let texture = ctx.load_texture(src).unwrap();
                let texture_size = ctx.texture_size(texture).unwrap();
                let offset =
                    self.inner.padding.unwrap_or(0.0) + self.inner.border_thickness.unwrap_or(0.0);
                ctx.draw_texture_sized(
                    texture,
                    self.pos + V2::new(offset, offset),
                    V2::new(
                        self.inner.width.unwrap_or(f64::from(texture_size.0)),
                        self.inner.height.unwrap_or(f64::from(texture_size.1)),
                    ),
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
        parent_pos: V2,
        pos_transformer: &mut dyn TransformersRobotsInDisguise,
    ) -> LayoutTree<'dom>;
}

pub(super) trait TransformersRobotsInDisguise {
    fn pos(&mut self, size: V2) -> V2;
    fn boxed(self) -> Box<dyn TransformersRobotsInDisguise>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

pub(super) struct HoriTransform {
    acc: f64,
    content_size: V2,
    padding: f64,
    gap: f64,
}
impl HoriTransform {
    fn new(content_size: V2, padding: f64, gap: f64) -> Self {
        Self {
            acc: 0.0,
            content_size,
            padding,
            gap,
        }
    }
}
impl TransformersRobotsInDisguise for HoriTransform {
    fn pos(&mut self, child_size: V2) -> V2 {
        let x = self.acc;
        let y = (self.content_size.y - child_size.y) / 2.0;
        self.acc += child_size.x + self.gap;
        V2::new(x + self.padding, y)
    }
}

pub(super) struct VertTransform {
    acc: f64,
    content_size: V2,
    padding: f64,
    gap: f64,
}
impl VertTransform {
    fn new(content_size: V2, padding: f64, gap: f64) -> Self {
        Self {
            acc: 0.0,
            content_size,
            padding,
            gap,
        }
    }
}
impl TransformersRobotsInDisguise for VertTransform {
    fn pos(&mut self, child_size: V2) -> V2 {
        let x = (self.content_size.x - child_size.x) / 2.0;
        let y = self.acc;
        self.acc += child_size.y + self.gap;
        V2::new(x, y + self.padding)
    }
}

struct StackTransform {
    content_size: V2,
}
impl StackTransform {
    fn new(content_size: V2) -> Self {
        Self { content_size }
    }
}
impl TransformersRobotsInDisguise for StackTransform {
    fn pos(&mut self, child_size: V2) -> V2 {
        (self.content_size - child_size).div_comps(2.0)
    }
}

pub(super) struct NoTransform;
impl TransformersRobotsInDisguise for NoTransform {
    fn pos(&mut self, _child_size: V2) -> V2 {
        V2::new(0.0, 0.0)
    }
}

impl CanCreateLayoutTree for Node {
    fn build_layout_tree<'dom>(
        &'dom self,
        node_id: InternalNodeId,
        dom: &'dom Dom,
        ctx: &mut impl UiContext,
        parent_pos: V2,
        pos_transformer: &mut dyn TransformersRobotsInDisguise,
    ) -> LayoutTree<'dom> {
        if !self.visible {
            return LayoutTree::single(LayoutTreeLeaf {
                size: V2::new(0.0, 0.0),
                pos: V2::new(0.0, 0.0),
                inner: self,
                node_id,
            });
        }
        fn build_leaf<'a>(
            node: &'a Node,
            node_id: InternalNodeId,
            pos_offset: &mut dyn TransformersRobotsInDisguise,
            parent_pos: V2,
            default_size: V2,
        ) -> LayoutTreeLeaf<'a> {
            let padding =
                (node.padding.unwrap_or(0.0) + node.border_thickness.unwrap_or(0.0)) * 2.0;
            let size = V2::new(
                node.width.unwrap_or(default_size.x),
                node.height.unwrap_or(default_size.y),
            ) + V2::new(padding, padding);
            let pos = pos_offset.pos(size);
            let pos = pos + parent_pos;
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
                let size = V2::new(f64::from(size.0), f64::from(size.1));
                let leaf = build_leaf(self, node_id, pos_transformer, parent_pos, size);
                LayoutTree::single(leaf)
            }
            Kind::Rect | Kind::Image(_) => {
                let leaf = build_leaf(
                    self,
                    node_id,
                    pos_transformer,
                    parent_pos,
                    V2::new(0.0, 0.0),
                );
                LayoutTree::single(leaf)
            }
            kind @ (Kind::Hori(children) | Kind::Vert(children) | Kind::Stack(children)) => {
                let padding = self.padding.unwrap_or(0.0) + self.border_thickness.unwrap_or(0.0);
                let gap = self.gap.unwrap_or(0.0);

                let size = children
                    .iter()
                    .filter_map(|id| dom.select_node(*id).map(|node| (*id, node)))
                    .map(|(id, node)| {
                        node.build_layout_tree(id, dom, ctx, V2::new(0.0, 0.0), &mut NoTransform)
                    })
                    .map(|tree| (tree, gap))
                    .fold(
                        (true, V2::new(0.0, 0.0)),
                        |(first, acc), (curr, gap)| match kind {
                            Kind::Vert(_) => {
                                let mut result = acc.y + curr.leaf.size.y;
                                if !first {
                                    result += gap;
                                }
                                (false, V2::new(max(acc.x, curr.leaf.size.x), result))
                            }
                            Kind::Hori(_) => {
                                let mut result = acc.x + curr.leaf.size.x;
                                if !first {
                                    result += gap;
                                }
                                (false, V2::new(result, max(acc.y, curr.leaf.size.y)))
                            }
                            Kind::Stack(_) => (
                                false,
                                V2::new(max(acc.x, curr.leaf.size.x), max(acc.y, curr.leaf.size.y)),
                            ),
                            _ => unreachable!("not matched prior"),
                        },
                    )
                    .1;

                let size = V2::new(self.width.unwrap_or(size.x), self.height.unwrap_or(size.y))
                    + V2::new(padding * 2.0, padding * 2.0);

                let pos = pos_transformer.pos(size);
                let pos = pos + parent_pos;

                let mut transformer = match kind {
                    Kind::Vert(_) => VertTransform::new(size, padding, gap).boxed(),
                    Kind::Hori(_) => HoriTransform::new(size, padding, gap).boxed(),
                    Kind::Stack(_) => StackTransform::new(size).boxed(),
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
                    size,
                    pos,
                    inner: self,
                    node_id,
                };

                LayoutTree::multiple(leaf, children)
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
        let multiple = LayoutTree::multiple;
        let single = LayoutTree::single;

        let focus_color = (50, 160, 190);
        let focus_thickness = 6.0;

        multiple(
            LayoutTreeLeaf {
                size: V2::new(
                    16.0 + 2.0 + 16.0 + 2.0 + 16.0 + 2.0,
                    16.0 + 2.0 + 16.0 + 2.0 + 16.0 + 2.0,
                ),
                pos: V2::new(0.0, 0.0),
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
                    border_thickness: Some(1.0),
                    padding: Some(8.0),
                    gap: None,
                    border_color: None,
                    font_size: None,
                    visible: true,
                    focused: false,
                    focus_color,
                    focus_thickness,
                })),
            },
            vec![multiple(
                LayoutTreeLeaf {
                    size: V2::new(16.0 + 2.0 + 16.0 + 2.0, 16.0 + 2.0 + 16.0 + 2.0),
                    pos: V2::new(8.0 + 1.0, 8.0 + 1.0),
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
                        border_thickness: Some(1.0),
                        padding: Some(8.0),
                        gap: None,
                        border_color: None,
                        font_size: None,
                        visible: true,
                        focused: false,
                        focus_color,
                        focus_thickness,
                    })),
                },
                vec![single(LayoutTreeLeaf {
                    size: V2::new(16.0 + 2.0, 16.0 + 2.0),
                    pos: V2::new(8.0 + 1.0 + 8.0 + 1.0, 8.0 + 1.0 + 8.0 + 1.0),
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
                        border_thickness: Some(1.0),
                        padding: Some(8.0),
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
