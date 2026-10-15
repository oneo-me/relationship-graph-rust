# Interaction and rendering

`view.rs` owns the `GraphView` entity, its background tasks, the camera, and pointer state. `paint.rs` converts that state into culling decisions, gradients, and shaped labels.

## Camera

`GraphCamera` stores an offset and a zoom. A node's screen position is the canvas center plus `camera + position * zoom`. Zoom is clamped to `1/128..=8` and does not affect graph coordinates.

- The wheel zooms by `exp(delta * ln(1.5) / 120)`, with line deltas treated as 16 pixels.
- Pinch zooms by `max(1 + delta, 0.01)`.
- `zoom_at` keeps the graph point under the pointer fixed while zooming.
- `fit_graph` pads by 36 units and never fits above zoom 1, so it does not enlarge small graphs.

`fit_graph` runs when the canvas bounds change, during layout animation, and on `fit_view`; the first two only while the user has not moved the camera. Panning, node dragging, and zooming set that flag; `replace_graph` clears it, and `fit_view` clears it and fits immediately for a host "fit view" command.

## Nodes on screen

`node_screen_radius` scales a node radius by `min(zoom / 0.4, 1)`, so radii stop growing above zoom 0.4, with a floor of 0.75 screen pixels.

Hit testing walks nodes in reverse order, so the topmost painted node wins. The hit radius is at least 6 screen pixels to keep small nodes clickable. Culling skips nodes outside the canvas and edges whose bounding box misses the canvas.

## Pointer state

Pointer down classifies the target as `Canvas` or `Node(index)` and records the start and last position. A node target is ignored while a newer snapshot is still pending, which prevents dragging stale geometry.

- A drag becomes real once it moves more than 4 pixels from the start.
- Canvas drag adds the frame delta to the camera offset and marks the camera as user-moved.
- Node drag moves the node by `delta / zoom`, pins it, and advances the simulation once.
- Hover without dragging updates the hovered node.

Releasing a dragged node reheats and starts the settle loop. Releasing the canvas only repaints.

## Activation and emphasis

A double-click on an activatable node that did not move emits `GraphEvent::NodeActivated(id)`. Dragging never activates, and non-activatable nodes emit nothing.

Hovering dims unrelated elements: the hovered node stays opaque, adjacent nodes drop to 0.8, and all others drop to 0.08. Edges between highlighted nodes keep their style opacity; unrelated edges drop to at most 0.009.

## Rendering

The view renders centered muted text when there are no visible nodes, choosing the loading label while a layout task runs and the empty label otherwise. Otherwise it renders a full-size canvas with `cursor_grab`, switching to `cursor_pointer` over an activatable node.

Both states use `Role::Image` with the `GraphLabels` accessibility callback and the `RelationshipGraph` key context. `on_prepaint` captures canvas bounds, shapes visible labels, and paints nodes and edges in the canvas closure.

A node is a corner-radius rounded quad filled with a 135° Oklab gradient. The gradient mixes the fill 22% toward white and 8% toward black, then blends both stops over the background by the node opacity. Labels are shaped at 11 pixels, faded in across zoom `0.35..0.65`, truncated to 132 logical pixels with an ellipsis, and painted 4 pixels below the node.

## View lifecycle

| Method | Behavior |
| --- | --- |
| `GraphView::new` | Seeds the source graph and starts a layout at alpha 1 |
| `set_graph` | Keeps camera and previous positions, seeds from matching IDs, reheats at 0.35 |
| `replace_graph` | Resets camera, positions, interaction, and pending work, then relayouts |
| `set_labels` | Replaces labels and repaints |
| `fit_view` | Fits the camera to the visible graph and clears the user-moved flag |

Layout and settle use generation counters, so a newer snapshot or settle request discards older work. A pending snapshot keeps the previous scene on screen until it is ready and cannot be discarded by a node drag. With reduced motion enabled, finished layouts apply without interpolation or the frame settle loop.

`GraphLabels` carries the loading and empty strings plus an accessibility closure that receives the displayed node and edge counts. Use `GraphLabels::default` for English text or `GraphLabels::new` for localized copy.
