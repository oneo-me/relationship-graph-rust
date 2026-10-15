# oneo-gpui-graph

**English** | [简体中文](README.zh-CN.md)

A GPUI Kit component that renders an interactive relationship graph. You supply validated nodes, edges, and labels; the viewer owns force layout, pan, zoom, node dragging, hover emphasis, and activation events.

## Install

Add the crate next to GPUI Kit and use Rust edition 2024:

```toml
[dependencies]
oneo-gpui-graph = { git = "https://github.com/oneo-me/relationship-graph-rust" }
gpui-kit = "0.7.1"
```

The host application must initialize GPUI Kit and place its `Root` at the window boundary.

## Usage

Build a validated `GraphData` snapshot, create a `GraphView` entity, and give its container a bounded size. Subscribe to `GraphEvent::NodeActivated` and resolve the opaque ID in your application.

```rust
use oneo_gpui_graph::{GraphData, GraphEdge, GraphEvent, GraphLabels, GraphNode, GraphView};

let data = GraphData::new(
    vec![
        GraphNode::new("service", "API"),
        GraphNode::new("store", "Database"),
    ],
    vec![GraphEdge::new("service", "store").with_distance(120.0)],
)?;
let graph = cx.new(|cx| GraphView::new(data, GraphLabels::default(), cx));
```

Run the standalone host to try a small graph and see activations reported:

```sh
cargo run -p oneo-gpui-graph --example relationships
```

## Documentation

- [Architecture](ARCHITECTURE.md) maps the modules, boundaries, and update model.
- [Force layout](docs/architecture/layout.md) explains seeding, forces, and settling.
- [Interaction and rendering](docs/architecture/interaction.md) covers camera, pointer, and paint.
- Generate the API reference with `cargo doc -p oneo-gpui-graph --open`.

## Development

Run the crate tests, including headless GPUI tests:

```sh
cargo test -p oneo-gpui-graph
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for invariants and [AGENTS.md](AGENTS.md) for working rules.

## License

MIT. See [LICENSE](LICENSE) for terms and [CHANGELOG.md](CHANGELOG.md) for release notes.
