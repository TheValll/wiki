# ROS2 — Review Progress

**Flow:** M (Mastery / spaced repetition) — see [`AGENT.md`](../AGENT.md) §3.M.

**Mode:** maintenance — the 24 concepts are already covered in the wiki. The goal is **retention**, not curriculum advancement. There is no "Current position" or "next concept to discover" — the agent rotates through the existing list, biased by block weight, picking what's been least-recently seen.

**Initialized:** 2026-04-26 — all 24 concepts entered at Level 0 (untested in this system).

---

## Bias for warm-up draw

| Bloc | Poids | Concepts |
|---|---|---|
| `setup/` | 5% | 1 |
| `basics/` | 25% | 7 |
| **`ros2-control/`** | **50%** | **9** ← Valentin's flagged weakness, priority drilling |
| `moveit/` | 20% | 7 |

**At warm-up Step 1**: pick block weighted by table above, then within the chosen block pick the concept with oldest "Last seen" (or any Level-0 concept if multiple are equally old). Pull 1-2 concepts per session.

---

## In review (active rotation)

### `setup/`

| # | Concept | Level | Last seen |
|---|---------|-------|-----------|
| 0 | Environment, Workspace & Essential CLI | 0 | — |

### `basics/`

| # | Concept | Level | Last seen |
|---|---------|-------|-----------|
| 1 | Nodes, DDS & the Graph | 0 | — |
| 2 | Topics & Pub/Sub | 0 | — |
| 3 | Services (Request/Reply) | 0 | — |
| 4 | Custom Interfaces (`.msg`, `.srv`) | 0 | — |
| 5 | Parameters | 0 | — |
| 6 | Launch Files | 0 | — |
| 7 | URDF & Visualization | 0 | — |

### `ros2-control/`

| # | Concept | Level | Last seen |
|---|---------|-------|-----------|
| 8 | ros2_control Architecture | 0 | — |
| 9 | Hardware Interface | 0 | — |
| 10 | ros2_control URDF Tags | 0 | — |
| 11 | Controllers — DiffDrive & JointStateBroadcaster | 0 | — |
| 12 | LX-225 Driver | 0 | — |
| 13 | Writing a Custom Controller | 0 | — |
| 14 | Controller Manager Internals | 0 | — |
| 15 | Lifecycle & State Machines | 0 | — |
| 16 | Transmissions, Sensors & GPIO | 0 | — |

### `moveit/`

| # | Concept | Level | Last seen |
|---|---------|-------|-----------|
| 17 | MoveIt Architecture | 0 | — |
| 18 | Configuration Space | 0 | — |
| 19 | Motion Planning Algorithms | 0 | — |
| 20 | Inverse Kinematics | 0 | — |
| 21 | Trajectory Generation | 0 | — |
| 22 | MoveIt Bringup | 0 | — |
| 23 | MoveIt C++ API (MoveGroupInterface) | 0 | — |

---

## Mastered (Level 4) — long-interval cold-checks only

*Empty for now.* In maintenance mode, concepts only reach Level 4 after multiple successful warm-ups across several sessions. Once at Level 4, they stay in the rotation but at very low weight (~once every 4-6 weeks for a cold-check — see AGENT.md §3.M Step 1 "Aged-concept variant").

---

## Intuition drills

*Empty.* ROS2 has no dedicated `*-intuition.md` companion file — analogies live inline in each wiki file's §X.1 section. Intuition mode is still available (AGENT.md §1.4) but uses those inline analogies as reference material rather than a separate companion. When the user runs an intuition drill on a ROS2 concept, log it here with date + status + gap notes.

| Concept | Source | First validated | Consolidated | Notes |
|---------|--------|-----------------|--------------|-------|

---

## Session history

| Date | Focus | Concepts touched | Notes |
|------|-------|------------------|-------|
