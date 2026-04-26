# ROS2 — Visual Atlas

A growing collection of multi-frame ASCII schemas covering the **logic and flow** of core ROS2 mechanisms.

**Role:** companion artifact for ROS2 — captures the *logic and flow* of core mechanisms in a form that survives months without touching the code. Each section captures one concept, decomposed frame-by-frame so the mental motion can be replayed offline (train ride, pre-sleep review). Frames are added on demand during articulation drills (see [`CLAUDE.md`](../CLAUDE.md) §7) or when the user requests a recap.

**Convention per concept:**

- One `## N — Concept name` section, where `N` matches the page number in [`README.md`](./README.md) (e.g. `08` for ros2_control architecture)
- Short orientation paragraph (1-2 sentences)
- **Multi-frame ASCII schema** — one frame per step of the mechanism, not one dense diagram. Rounded boxes (`┌─────┐`)
- Optional small comparative table for case analysis
- **No code, no API names.** Pure logic and mechanism — the kind of content that survives months without touching ROS2

This file grows as Flow E sessions cover concepts. It is **not** generated wholesale up front — each frame lands only after the user has articulated the concept cleanly in a session.

---

*Empty for now. The first frame lands at the first Flow E session.*
