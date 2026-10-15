# Changelog

## [Unreleased]

- Add the relationship graph component with force layout, pan, zoom, node dragging, hover emphasis, and node activation events.
- Render validated graph data with opaque IDs, cluster seeding, per-appearance node colors, and per-appearance edge styles.
- Run layout on a background executor with frame-driven settling that honors GPUI reduced motion.
- Expose `GraphView::fit_view` for host "fit view" commands, and turn the relationships example into a themed toolbar demo over a colorful tree whose descendants inherit their branch color.
