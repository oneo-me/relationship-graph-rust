use gpui_kit::assets::IconName as AssetIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::toolbar::Toolbar;
use gpui_kit::component::{Icon, Theme, ThemeMode};
use gpui_kit::{
    AppContext, Context, Entity, Render, SharedString, Subscription, Window, WindowOptions, div,
    hsla, prelude::*,
};
use oneo_gpui_graph::{
    GraphData, GraphEdge, GraphEdgeStyle, GraphEvent, GraphLabels, GraphNode, GraphNodeColors,
    GraphView,
};

const RNG_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const BRANCH_HUES: [f32; 6] = [212.0, 150.0, 32.0, 285.0, 190.0, 340.0];

/// One node of the demo tree. Its color is a hue that every descendant inherits.
#[derive(Clone)]
struct DemoNode {
    id: String,
    label: String,
    parent: Option<String>,
    hue: Option<f32>,
    radius: f32,
    cluster: String,
}

struct Relationships {
    graph: Entity<GraphView>,
    status: SharedString,
    nodes: Vec<DemoNode>,
    next_id: usize,
    rng: u64,
    _subscription: Subscription,
}

impl Relationships {
    fn new(cx: &mut Context<Self>) -> Self {
        let nodes = initial_nodes();
        let count = nodes.len();
        let graph = cx.new(|cx| GraphView::new(build_graph(&nodes), GraphLabels::default(), cx));
        let subscription = cx.subscribe(&graph, |this, _, event, cx| {
            let GraphEvent::NodeActivated(id) = event;
            this.status = format!("Activated: {id}").into();
            cx.notify();
        });
        Self {
            graph,
            status: format!("{count} nodes · 双击节点查看激活事件").into(),
            nodes,
            next_id: 0,
            rng: RNG_SEED,
            _subscription: subscription,
        }
    }

    /// Appends `count` children, each inheriting its parent's hue and cluster.
    fn add_nodes(&mut self, count: usize, cx: &mut Context<Self>) {
        let mut pool = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.hue.is_some())
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if pool.is_empty() {
            pool.extend(0..self.nodes.len());
        }
        for _ in 0..count {
            let parent_index = pool[(self.next_random() as usize) % pool.len()];
            let (parent_id, hue, cluster) = {
                let parent = &self.nodes[parent_index];
                (parent.id.clone(), parent.hue, parent.cluster.clone())
            };
            let id = format!("node-{}", self.next_id);
            self.next_id += 1;
            self.nodes.push(DemoNode {
                id: id.clone(),
                label: id,
                parent: Some(parent_id),
                hue,
                radius: 6.0,
                cluster,
            });
            if hue.is_some() {
                pool.push(self.nodes.len() - 1);
            }
        }
        self.apply_graph(format!("添加 {count} 个节点"), false, cx);
    }

    fn fit_to_view(&mut self, cx: &mut Context<Self>) {
        self.graph.update(cx, |graph, cx| graph.fit_view(cx));
        self.status = "已适应视图 · 滚轮缩放，拖拽平移".into();
        cx.notify();
    }

    fn reset(&mut self, cx: &mut Context<Self>) {
        self.nodes = initial_nodes();
        self.next_id = 0;
        self.rng = RNG_SEED;
        self.apply_graph("已重置节点".into(), true, cx);
    }

    fn apply_graph(&mut self, message: String, replace: bool, cx: &mut Context<Self>) {
        let data = build_graph(&self.nodes);
        self.graph.update(cx, |graph, cx| {
            if replace {
                graph.replace_graph(data, cx);
            } else {
                graph.set_graph(data, cx);
            }
        });
        self.status = format!("{message} · {} 个节点", self.nodes.len()).into();
        cx.notify();
    }

    fn next_random(&mut self) -> u64 {
        self.rng = self
            .rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.rng >> 33
    }
}

impl Render for Relationships {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::global(cx);
        let dark = theme.is_dark();
        let background = theme.background;
        let foreground = theme.foreground;
        let muted = theme.muted_foreground;
        let border = theme.border;
        let warning = theme.warning;
        let accent = theme.primary;
        let chart_1 = theme.chart_1;
        let chart_2 = theme.chart_2;
        let chart_3 = theme.chart_3;
        let chart_4 = theme.chart_4;
        let chart_5 = theme.chart_5;

        let entity = cx.entity();
        let add_one = entity.clone();
        let add_hundred = entity.clone();
        let add_thousand = entity.clone();
        let fit = entity.clone();
        let reset = entity.clone();

        let (theme_icon, theme_color, theme_action, theme_label) = if dark {
            (AssetIcon::Sun, warning, ThemeMode::Light, "浅色")
        } else {
            (AssetIcon::Moon, accent, ThemeMode::Dark, "深色")
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(background)
            .text_color(foreground)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .px_3()
                    .py_1()
                    .bg(background)
                    .border_b_1()
                    .border_color(border)
                    .child(
                        Toolbar::new("graph-toolbar")
                            .child(
                                Button::new("graph-add-node")
                                    .ghost()
                                    .icon(Icon::from(AssetIcon::CirclePlus).text_color(chart_1))
                                    .label("添加节点")
                                    .tooltip("添加一个节点")
                                    .on_click(move |_, _, cx| {
                                        add_one.update(cx, |this, cx| this.add_nodes(1, cx));
                                    }),
                            )
                            .child(
                                Button::new("graph-add-100")
                                    .ghost()
                                    .icon(Icon::from(AssetIcon::CopyPlus).text_color(chart_2))
                                    .label("添加100个节点")
                                    .on_click(move |_, _, cx| {
                                        add_hundred.update(cx, |this, cx| this.add_nodes(100, cx));
                                    }),
                            )
                            .child(
                                Button::new("graph-add-1000")
                                    .ghost()
                                    .icon(Icon::from(AssetIcon::LayersPlus).text_color(chart_3))
                                    .label("添加1000个节点")
                                    .on_click(move |_, _, cx| {
                                        add_thousand
                                            .update(cx, |this, cx| this.add_nodes(1000, cx));
                                    }),
                            )
                            .child(
                                Button::new("graph-fit")
                                    .ghost()
                                    .icon(Icon::from(AssetIcon::Scan).text_color(chart_4))
                                    .label("适应视图")
                                    .on_click(move |_, _, cx| {
                                        fit.update(cx, |this, cx| this.fit_to_view(cx));
                                    }),
                            )
                            .child(
                                Button::new("graph-reset")
                                    .ghost()
                                    .icon(Icon::from(AssetIcon::RotateCcw).text_color(chart_5))
                                    .label("重置节点")
                                    .on_click(move |_, _, cx| {
                                        reset.update(cx, |this, cx| this.reset(cx));
                                    }),
                            ),
                    )
                    .child(
                        Toolbar::new("graph-toolbar-theme").child(
                            Button::new("graph-toggle-theme")
                                .ghost()
                                .icon(Icon::from(theme_icon).text_color(theme_color))
                                .label(theme_label)
                                .tooltip("切换主题")
                                .on_click(move |_, _, cx| {
                                    Theme::change(theme_action, None, cx);
                                }),
                        ),
                    ),
            )
            .child(div().flex_1().min_h_0().child(self.graph.clone()))
            .child(
                div()
                    .px_4()
                    .py_2()
                    .text_sm()
                    .text_color(muted)
                    .border_t_1()
                    .border_color(border)
                    .child(self.status.clone()),
            )
    }
}

fn demo_node(
    id: &str,
    label: &str,
    parent: Option<&str>,
    hue: Option<f32>,
    radius: f32,
    cluster: &str,
) -> DemoNode {
    DemoNode {
        id: id.into(),
        label: label.into(),
        parent: parent.map(str::to_owned),
        hue,
        radius,
        cluster: cluster.into(),
    }
}

/// A small tree: one neutral root, colored branches, and descendants that
/// repeat their branch hue until the host adds more nodes.
fn initial_nodes() -> Vec<DemoNode> {
    vec![
        demo_node("neorio", "Neorio", None, None, 13.0, "root"),
        demo_node(
            "src",
            "src",
            Some("neorio"),
            Some(BRANCH_HUES[0]),
            9.0,
            "src",
        ),
        demo_node(
            "src-main",
            "main.rs",
            Some("src"),
            Some(BRANCH_HUES[0]),
            6.0,
            "src",
        ),
        demo_node(
            "src-lib",
            "lib.rs",
            Some("src"),
            Some(BRANCH_HUES[0]),
            6.0,
            "src",
        ),
        demo_node(
            "src-layout",
            "layout.rs",
            Some("src"),
            Some(BRANCH_HUES[0]),
            6.0,
            "src",
        ),
        demo_node(
            "src-paint",
            "paint.rs",
            Some("src"),
            Some(BRANCH_HUES[0]),
            6.0,
            "src",
        ),
        demo_node(
            "src-view",
            "view.rs",
            Some("src"),
            Some(BRANCH_HUES[0]),
            6.0,
            "src",
        ),
        demo_node(
            "assets",
            "assets",
            Some("neorio"),
            Some(BRANCH_HUES[1]),
            9.0,
            "assets",
        ),
        demo_node(
            "assets-icons",
            "icons",
            Some("assets"),
            Some(BRANCH_HUES[1]),
            6.0,
            "assets",
        ),
        demo_node(
            "assets-fonts",
            "fonts",
            Some("assets"),
            Some(BRANCH_HUES[1]),
            6.0,
            "assets",
        ),
        demo_node(
            "assets-themes",
            "themes",
            Some("assets"),
            Some(BRANCH_HUES[1]),
            6.0,
            "assets",
        ),
        demo_node(
            "docs",
            "docs",
            Some("neorio"),
            Some(BRANCH_HUES[2]),
            9.0,
            "docs",
        ),
        demo_node(
            "docs-architecture",
            "architecture",
            Some("docs"),
            Some(BRANCH_HUES[2]),
            6.0,
            "docs",
        ),
        demo_node(
            "docs-readme",
            "README.md",
            Some("docs"),
            Some(BRANCH_HUES[2]),
            6.0,
            "docs",
        ),
        demo_node(
            "docs-changelog",
            "CHANGELOG.md",
            Some("docs"),
            Some(BRANCH_HUES[2]),
            6.0,
            "docs",
        ),
        demo_node(
            "tests",
            "tests",
            Some("neorio"),
            Some(BRANCH_HUES[3]),
            9.0,
            "tests",
        ),
        demo_node(
            "tests-graph-data",
            "graph_data.rs",
            Some("tests"),
            Some(BRANCH_HUES[3]),
            6.0,
            "tests",
        ),
        demo_node(
            "tests-view",
            "view.rs",
            Some("tests"),
            Some(BRANCH_HUES[3]),
            6.0,
            "tests",
        ),
        demo_node(
            "examples",
            "examples",
            Some("neorio"),
            Some(BRANCH_HUES[4]),
            9.0,
            "examples",
        ),
        demo_node(
            "examples-relationships",
            "relationships.rs",
            Some("examples"),
            Some(BRANCH_HUES[4]),
            6.0,
            "examples",
        ),
        demo_node(
            "tools",
            "tools",
            Some("neorio"),
            Some(BRANCH_HUES[5]),
            9.0,
            "tools",
        ),
        demo_node(
            "tools-fmt",
            "fmt",
            Some("tools"),
            Some(BRANCH_HUES[5]),
            6.0,
            "tools",
        ),
        demo_node(
            "tools-clippy",
            "clippy",
            Some("tools"),
            Some(BRANCH_HUES[5]),
            6.0,
            "tools",
        ),
    ]
}

fn node_palette(hue: Option<f32>, dark: bool) -> GraphNodeColors {
    match hue {
        Some(hue) => GraphNodeColors::new(
            hsla(hue / 360.0, 0.72, if dark { 0.60 } else { 0.52 }, 1.0),
            hsla(0.0, 0.0, 1.0, 1.0),
        ),
        None => GraphNodeColors::new(
            hsla(0.0, 0.0, if dark { 0.55 } else { 0.40 }, 1.0),
            hsla(0.0, 0.0, 1.0, 1.0),
        ),
    }
}

fn edge_style(hue: Option<f32>, dark: bool) -> GraphEdgeStyle {
    let color = match hue {
        Some(hue) => hsla(hue / 360.0, 0.60, if dark { 0.68 } else { 0.45 }, 1.0),
        None => hsla(0.0, 0.0, if dark { 0.60 } else { 0.45 }, 1.0),
    };
    GraphEdgeStyle::default()
        .with_color(color)
        .with_opacity(if dark { 0.42 } else { 0.38 })
        .with_width(1.4)
}

fn build_graph(nodes: &[DemoNode]) -> GraphData {
    let graph_nodes = nodes
        .iter()
        .map(|node| {
            GraphNode::new(node.id.clone(), node.label.clone())
                .with_cluster(node.cluster.clone())
                .with_radius(node.radius)
                .with_colors(node_palette(node.hue, false), node_palette(node.hue, true))
        })
        .collect::<Vec<_>>();
    let edges = nodes
        .iter()
        .filter_map(|node| {
            let parent = node.parent.as_deref()?;
            Some(
                GraphEdge::new(parent, node.id.clone())
                    .with_distance(90.0)
                    .with_style(edge_style(node.hue, false), edge_style(node.hue, true)),
            )
        })
        .collect::<Vec<_>>();
    GraphData::new(graph_nodes, edges).expect("the demo tree is a valid graph")
}

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::AllAssets)
        .run(|cx| {
            gpui_kit::init(cx);
            gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
                cx.new(Relationships::new)
            })
            .unwrap();
            cx.activate(true);
        });
}
