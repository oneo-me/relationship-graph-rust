# Force layout

The layout turns a `GraphData` snapshot into node positions. It runs on a background executor and never blocks rendering. `layout.rs` is pure geometry and physics: no GPUI, pixels, or themes.

## Pipeline

1. `layout_graph` indexes nodes by ID and translates edges into `LayoutEdge` values.
2. Each edge stores a normalized `strength` and a `bias` derived from endpoint degrees.
3. `initial_positions` seeds every node, reusing prior positions by ID.
4. `simulate_layout` iterates the forces until alpha reaches `ALPHA_MIN`.
5. `LayoutResult` carries the graph, positions, edges, and adjacency to the view.

Edge `strength` is the requested strength divided by the smaller endpoint degree, with a floor of 1. `bias` is `source_degree / (source_degree + target_degree)`. Parallel edges stay separate edges but share endpoints.

## Seeding

Previous positions win, so re-layouts and node reordering stay stable. A new node with an already laid-out neighbor starts 24 units from the average neighbor position, offset by a hash-derived angle.

Without a neighbor, `seeded_position` places the node from an FNV-1a hash of its ID. Clusters spread around a ring at 72% of the outer radius; unclustered nodes fill the outer radius. Both use a hash-derived angle and radius, so positions are reproducible across runs.

The outer radius is `sqrt(nodes * 60² / π)`. A cluster radius is `max(sqrt(nodes_per_cluster * 60² / π) * 0.95, 64)`.

## Forces

Each iteration applies four forces in order. Alpha scales centering, link, and many-body forces; collision uses the frame time scale instead.

| Force | Constant | Effect |
| --- | --- | --- |
| Position | `POSITION_STRENGTH = 0.1` | Pulls every node toward the origin |
| Link | edge `distance` and `strength` | Moves endpoints toward the target distance |
| Many body | `MANY_BODY_STRENGTH = -1000` | Pushes every node pair apart, minimum distance 30 |
| Collision | `COLLISION_RADIUS = 60`, strength 0.5 | Separates nodes closer than 120 using a grid |

Many-body force is pairwise `1000 * alpha / distance²` and therefore `O(n²)`. Collision buckets nodes into 120-unit cells and only checks the surrounding cells. When two nodes coincide, `nonzero_delta` applies a tiny deterministic offset so distances stay finite.

## Alpha schedule

`ALPHA_DECAY` is `0.02276278` and `ALPHA_MIN` is `0.001`. `simulate_layout` computes its iteration count from the closing alpha, then decays alpha toward zero.

| Entry point | Pinned node | Alpha target | Iterations |
| --- | --- | --- | --- |
| `simulate_layout` | none | 0 | until `ALPHA_MIN` |
| `advance_layout` | drag node | 0.25 | caller-supplied |
| `advance_layout_scaled` | none | 0 | 1 per frame |

Velocity decays by `0.6^time_scale` and positions advance by `velocity * time_scale`. The view clamps settle `time_scale` to `0.25..=2.0` from frame elapsed time, so refresh rate does not change the settling speed.

## View integration

The view reheats to `alpha = max(current, 0.4)` when a drag starts or a node is released. A drag pins the dragged node and targets alpha `0.25`; release settles frame by frame until alpha falls below `ALPHA_MIN`, then zeroes alpha and velocity.

With GPUI reduced motion enabled, the view skips interpolation and the frame-driven settle loop. It applies the finished layout directly and re-runs the background layout once for release.

`graph_bounds` returns the position extent used by the camera fit. See [Interaction and rendering](interaction.md) for how the view consumes these positions.
