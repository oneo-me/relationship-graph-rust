# oneo-gpui-graph

[English](README.md) | **简体中文**

一个 GPUI Kit 组件，用于渲染可交互的关系图谱。你提供经过校验的节点、边和文案，视图负责力导向布局、平移、缩放、节点拖拽、悬停强调和激活事件。

## 安装

在 GPUI Kit 之外添加该 crate，并使用 Rust 2024 edition：

```toml
[dependencies]
oneo-gpui-graph = { git = "https://github.com/oneo-me/relationship-graph-rust" }
gpui-kit = "0.7.1"
```

宿主应用需要初始化 GPUI Kit，并把它的 `Root` 放在窗口边界处。

## 使用

构造经过校验的 `GraphData` 快照，创建 `GraphView` 实体，并给它的容器一个有界尺寸。订阅 `GraphEvent::NodeActivated`，在应用中解析不透明的 ID。

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

运行独立宿主示例，试用一个小图谱并查看激活结果：

```sh
cargo run -p oneo-gpui-graph --example relationships
```

## 文档

- [架构](ARCHITECTURE.md) 说明模块、边界和更新模型。
- [力导向布局](docs/architecture/layout.md) 说明初始位置、作用力和收敛过程。
- [交互与渲染](docs/architecture/interaction.md) 说明相机、指针和绘制。
- 使用 `cargo doc -p oneo-gpui-graph --open` 生成 API 参考。

## 开发

运行 crate 测试，其中包含无头 GPUI 测试：

```sh
cargo test -p oneo-gpui-graph
```

不变量见 [ARCHITECTURE.md](ARCHITECTURE.md)，协作规则见 [AGENTS.md](AGENTS.md)。

## 许可证

MIT。条款见 [LICENSE](LICENSE)，发布说明见 [CHANGELOG.md](CHANGELOG.md)。
