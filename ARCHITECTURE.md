# oneo-gpui-graph architecture

`oneo-gpui-graph` is one GPUI Kit component in one crate. It turns a validated snapshot of nodes and edges into an interactive viewport: a background force layout produces positions, and the view paints, animates, and handles input. IDs, labels, clusters, and colors are opaque data with no document, filesystem, or application meaning.

## Code map

| Path | Responsibility |
| --- | --- |
| `src/lib.rs` | Public surface, module wiring, and the crate contract |
| `src/model.rs` | Validated `GraphData`, nodes, edges, styles, and `GraphError` |
| `src/labels.rs` | Loading and empty strings plus the accessibility callback |
| `src/layout.rs` | Seeded initial positions, force simulation, and bounds |
| `src/paint.rs` | Camera, paint snapshot, culling, node gradients, and label shaping |
| `src/view.rs` | `GraphView` entity, `GraphEvent`, tasks, pointer state, and render |
| `examples/relationships.rs` | Standalone host for manual review |
| `tests/graph_data.rs` | Public validation tests |

Paths are relative to `crates/oneo-gpui-graph`.

## Boundaries

- `model.rs` owns snapshot facts and validation; it never depends on rendering.
- `layout.rs` is pure geometry and physics and never references GPUI, pixels, or themes.
- `paint.rs` converts themed state into drawing primitives without changing layout or interaction.
- `view.rs` owns the entity, background tasks, camera, pointer state, and emitted events.
- The host gives the viewport a bounded size, retains the subscription, and resolves opaque IDs.
- The viewport uses the `RelationshipGraph` key context and owns pointer focus.
- Node IDs are opaque and stable; matching IDs seed the next layout from prior positions.

## Core flows

**New data.** `GraphView::new` and `set_graph` spawn `schedule_layout`, which runs `layout_graph` on the background executor and installs a new visible graph. Position changes animate over 260 ms unless GPUI reduced motion is enabled.

**Input.** Pointer down records a canvas or node target. Canvas drag pans the camera; node drag pins the node and reheats the simulation. Release settles the layout each frame until alpha reaches `ALPHA_MIN`.

**Activation.** A double-click on an activatable node that did not move emits `GraphEvent::NodeActivated(id)`; dragging never activates.

## Verification

`cargo test -p oneo-gpui-graph` runs unit tests for layout, painting, and the view plus public validation tests. Headless GPUI tests cover snapshot updates, pointer activation, dragging, focus, and zoom clamping. Interface and visual changes still need review in the example host on the target platform.

## Subjects

| Document | When to read |
| --- | --- |
| [Force layout](docs/architecture/layout.md) | Changing seeding, forces, alpha, or settling |
| [Interaction and rendering](docs/architecture/interaction.md) | Changing camera, pointer handling, activation, or painting |
