# Part 19 — Motion Planning Algorithms

## 19.1 — The Analogy

Imagine finding a path through a dark forest at night. You can't see the whole forest at once (the C-space is too big), but you have a flashlight: you can test any specific spot and see if it's clear or blocked.

**Grid search** would be tiling the entire forest with a grid — impossible if the forest is huge. **Sampling-based planning** is different: you throw random darts into the forest, check if each landing spot is clear, and gradually build a map of passable terrain by connecting clear spots with tested corridors.

---

## 19.2 — Why Not Grid Search?

Part 18 showed that a 6-DOF grid with 0.1 rad resolution has ~60 billion cells. But the problem is deeper than memory:

### Computational complexity

The motion planning problem is **PSPACE-hard** in general:
- No polynomial-time exact algorithm exists
- Complete grid search grows exponentially with DOF
- Even for simple robots, exact cell decomposition is impractical above 3-4 DOF

### The solution: probabilistic completeness

Sampling-based planners trade **completeness** for **tractability**:

```
Deterministic planner: "I will find a path if one exists, or prove none exists"
  → Requires exploring the entire C-space. Intractable in high dimensions.

Probabilistically complete planner: "If a path exists, I will find it 
  with probability → 1 as the number of samples → ∞"
  → Works in practice with thousands of samples in seconds.
```

---

## 19.3 — Rapidly-exploring Random Trees (RRT)

RRT is the foundation of most planners in MoveIt/OMPL. It builds a **tree** rooted at the start configuration, growing toward random samples.

### Algorithm

```
RRT(q_start, q_goal, max_iterations):
    T = Tree(root = q_start)
    
    for i = 1 to max_iterations:
        q_rand = random_configuration()          // Step 1: Sample
        q_near = nearest_node(T, q_rand)         // Step 2: Find nearest
        q_new  = steer(q_near, q_rand, step)     // Step 3: Extend
        
        if collision_free(q_near, q_new):         // Step 4: Check
            T.add_edge(q_near, q_new)
            
            if distance(q_new, q_goal) < threshold:
                return path(q_start → ... → q_new → q_goal)
    
    return FAILURE
```

### Step by step

**Step 1 — Random sampling:**
```
q_rand = [uniform(-π, π), uniform(-π/2, π/2), ..., uniform(-π, π)]

Each joint sampled independently within its limits.
```

**Step 2 — Nearest neighbor:**
```
q_near = argmin_{q ∈ T} d(q, q_rand)

Using the weighted distance from Part 18:
d(q_a, q_b) = √( Σᵢ (q_a_i - q_b_i)² )

Efficient implementation: kd-tree for O(log n) lookup instead of O(n).
```

**Step 3 — Steer (extend):**
```
direction = (q_rand - q_near) / ||q_rand - q_near||
q_new = q_near + step_size · direction

step_size is typically 0.05 - 0.2 rad.
```

This moves from q_near **toward** q_rand, but only by `step_size`. This ensures small increments that can be collision-checked.

**Step 4 — Collision check:**

Test the edge from q_near to q_new by interpolating and checking discrete points:
```
for t = 0, Δ, 2Δ, ..., 1:
    q_test = q_near + t · (q_new - q_near)
    if collision(q_test):
        return NOT collision_free
return collision_free

Δ = collision_check_resolution (typically 0.01-0.05 rad)
```

### Visualization (2D example)

```
Iteration 1:          Iteration 10:         Iteration 50:
                                              
  q_goal *              q_goal *              q_goal *-----+
                                                     |     |
  ██obstacle██         ██obstacle██         ██obstacle██   |
                             /                   /    |    |
                            /                   /     |    |
  q_start *          q_start *---+       q_start *---+----+
                              \               |
                               \              +--+
                                                  \
  (1 node)            (tree growing)         (reached goal!)
```

### Properties

| Property | Value |
|----------|-------|
| Complete? | Probabilistically complete |
| Optimal? | No — finds **a** path, not the shortest |
| Time complexity | O(n log n) per iteration (nearest neighbor) |
| Space complexity | O(n) for n samples |
| Works in | Any dimension (tested up to 100+ DOF) |

---

## 19.4 — RRT-Connect (Bidirectional RRT)

MoveIt's default planner. It grows **two trees** — one from start, one from goal — and tries to connect them. Much faster than single-tree RRT.

### Algorithm

```
RRT-Connect(q_start, q_goal):
    T_a = Tree(root = q_start)    // Forward tree
    T_b = Tree(root = q_goal)     // Backward tree
    
    for i = 1 to max_iterations:
        q_rand = random_configuration()
        
        // Extend T_a toward q_rand
        if EXTEND(T_a, q_rand) ≠ TRAPPED:
            // Try to CONNECT T_b to T_a's new node
            if CONNECT(T_b, q_new_a) == REACHED:
                return extract_path(T_a, T_b)    // Trees met!
        
        SWAP(T_a, T_b)    // Alternate which tree extends
    
    return FAILURE
```

### EXTEND vs CONNECT

```
EXTEND: take ONE step toward the target
  q_near ----step----> q_new    (stop after one step)

CONNECT: take MANY steps until you reach or get blocked  
  q_near ----step----> ----step----> ----step----> q_target
  (greedy — keeps going until collision or arrival)
```

The **CONNECT** step is what makes RRT-Connect fast: once one tree gets close to the other, CONNECT greedily bridges the gap.

### Why bidirectional is faster

```
Single RRT:                    RRT-Connect:
  Start *                        Start *          * Goal
        |                              |          |
        |                              |          |
        +---+                          +---+  +---+
            |                              |  |
            +---+                          +--+  ← meet in the middle!
                |
                +---+
                    |  (must reach goal by luck)
                    
  Tree must explore toward the     Both trees explore toward each other.
  goal blindly. Slow if goal is    Effective search space roughly halved.
  in a narrow passage.
```

### Performance

For a 6-DOF arm with typical obstacles:
- **RRT-Connect**: 0.01 - 0.5 seconds for most problems
- **Single RRT**: 0.1 - 5 seconds for the same problems
- Factor **5-50x speedup** from bidirectional search

---

## 19.5 — RRT* (Asymptotically Optimal RRT)

RRT finds **a** path but not the **best** path. RRT* improves the path quality over time.

### Key additions over RRT

**1. Rewiring:**
After adding q_new, check if nearby nodes would have a shorter path through q_new:

```
Standard RRT:
  Parent of q_new = q_near (always)

RRT*:
  1. Find all nodes within radius r of q_new
  2. Choose parent = node that gives LOWEST COST path from start to q_new
  3. Rewire: for each neighbor, check if routing through q_new is shorter

  Before rewiring:         After rewiring:
    A ---5--- B              A ---5--- B
    |         |              |       / |
    3         4              3     2   4
    |         |              |   /     |
    C ---6--- D              C ---6--- D ---2--- q_new
              |                         
              2              B's parent changed from A to q_new
              |              if cost(start→q_new→B) < cost(start→A→B)
              q_new
```

**2. Search radius:**
The rewiring radius shrinks as the tree grows:

```
r_n = γ · (log(n) / n)^(1/d)

Where:
  n = number of nodes in the tree
  d = dimension of C-space (6 for our arm)
  γ = constant depending on the free space volume
```

### Convergence guarantee

```
As n → ∞:  cost(RRT* path) → cost(optimal path)

This is asymptotic optimality: given enough time, RRT* finds
the shortest collision-free path.
```

### Trade-off

| | RRT-Connect | RRT* |
|---|---|---|
| First solution | Fast (milliseconds) | Fast |
| Solution quality | Any valid path | Improves over time |
| Use case | "Get there safely" | "Get there efficiently" |
| Anytime? | No (returns first path) | Yes (keeps improving) |

---

## 19.6 — PRM (Probabilistic Roadmap)

PRM is a **multi-query** planner. Instead of building a tree for each query, it builds a **reusable graph** (roadmap) of the free space.

### Two phases

**Phase 1 — Construction (offline, once):**
```
PRM_Build(N_samples):
    V = {}  // vertices
    E = {}  // edges
    
    for i = 1 to N_samples:
        q = random_configuration()
        if collision_free(q):
            V.add(q)
            for each q_near in k_nearest(V, q):
                if collision_free_edge(q, q_near):
                    E.add(q, q_near)
    
    return Graph(V, E)
```

**Phase 2 — Query (online, per request):**
```
PRM_Query(q_start, q_goal, G):
    // Connect start and goal to the roadmap
    connect q_start to nearest nodes in G
    connect q_goal to nearest nodes in G
    
    // Graph search (A* or Dijkstra)
    return shortest_path(G, q_start, q_goal)
```

### When to use PRM vs RRT

| | PRM | RRT |
|---|---|---|
| **Queries** | Many queries in same environment | Single query |
| **Preprocessing** | Expensive (build roadmap) | None |
| **Per-query cost** | Fast (graph search) | Moderate (build tree) |
| **Dynamic obstacles** | Must rebuild roadmap | Naturally handles changes |
| **Best for** | Fixed workcell, repeated tasks | Changing environments |

For a robot arm doing repeated pick-and-place in a fixed workcell, PRM is ideal. For a mobile manipulator in a dynamic environment, RRT is better.

---

## 19.7 — OMPL: The Planning Library

MoveIt uses **OMPL** (Open Motion Planning Library) for sampling-based planning. OMPL provides:

### Available planners in MoveIt

| Planner | Type | Optimal? | Best for |
|---------|------|----------|----------|
| RRTConnect | Tree (bidirectional) | No | General use, fast |
| RRT | Tree (single) | No | Simple problems |
| RRT* | Tree (asymptotic) | Yes | Quality-sensitive |
| PRM | Roadmap | No | Multi-query |
| PRM* | Roadmap | Yes | Multi-query + quality |
| LazyPRM | Roadmap (lazy collision check) | No | Expensive collision checks |
| KPIECE | Tree (projection-based) | No | High-DOF, constrained |
| EST | Tree (expansion) | No | Narrow passages |
| BKPIECE | Tree (bidirectional KPIECE) | No | Very constrained spaces |
| FMT* | Tree (forward) | Yes | Known bounds on optimal cost |

### OMPL configuration in MoveIt

From `kinematics.yaml` and the planning pipeline config:

```yaml
# Planning pipeline selection (in move_group launch)
planning_pipelines:
  - ompl
  - pilz_industrial_motion_planner

# OMPL planner config (auto-generated)
arm:
  planner_configs:
    - RRTConnect      # default
    - RRTstar
    - PRM
```

### Planning request parameters

```python
# Typical MoveIt Python API call
move_group.set_planner_id("RRTConnect")
move_group.set_planning_time(5.0)          # max seconds
move_group.set_num_planning_attempts(10)    # retry if first attempt fails
move_group.set_goal_tolerance(0.01)         # joint tolerance (rad)
```

---

## 19.8 — Narrow Passages: The Hard Problem

The hardest problems for sampling-based planners are **narrow passages** — corridors in C-space where free space is very thin:

```
C-space cross-section:

  ████████████████████████████████████████
  ████████████████████████████████████████
  ████████████         ████████████████
  ████████████   GAP   ████████████████    <- narrow passage
  ████████████         ████████████████
  ████████████████████████████████████████
  ████████████████████████████████████████
  
  Start *                                * Goal
  (free)        (must pass through       (free)
                 the tiny gap)
```

Physical example: moving an arm through a gap between shelf boards, or reaching behind an obstacle.

### Why they're hard

The probability of randomly sampling inside a narrow passage of width w in d dimensions:

```
P(sample in passage) ≈ (w / L)^d

Where:
  w = passage width
  L = total range of each dimension
  d = number of dimensions

For w = 0.1 rad, L = 2π, d = 6:
  P ≈ (0.1 / 6.28)^6 ≈ 4 × 10⁻¹¹

Expected samples needed: ~25 billion. Infeasible.
```

### Solutions

| Strategy | How it works |
|----------|-------------|
| **Gaussian sampling** | Sample near obstacle surfaces (where passages are) |
| **Bridge sampling** | Sample pairs of points in C_obs, use midpoint if it's free |
| **Retraction-based** | Push collision samples to the boundary of C_free |
| **Adaptive sampling** | Increase density in unexplored regions |

MoveIt planners like **KPIECE** and **EST** are specifically designed for narrow passages by biasing samples toward under-explored regions.

---

## 19.9 — Path Smoothing

Raw paths from RRT are jagged (they follow the random tree structure). **Post-processing** smooths the path:

### Shortcutting

```
Original path: q0 → q1 → q2 → q3 → q4 → q5

Try connecting non-adjacent nodes:
  q0 → q3 collision free? Yes → skip q1, q2
  q0 → q4 collision free? No  → keep q3
  q3 → q5 collision free? Yes → skip q4

Smoothed path: q0 → q3 → q5

Repeat until no more shortcuts possible.
```

### B-spline smoothing

After shortcutting, fit a smooth curve through the remaining waypoints:

```
Original (jagged):     Smoothed (B-spline):
  *                      *
  |                       \
  *--*                     *--*
     |                         \
     *---*                      *---*
         |                           \
         *                            *

The B-spline ensures continuous velocity and acceleration
(no infinite jerks at waypoints).
```

OMPL applies shortcutting and B-spline smoothing by default after finding a path.

---

## 19.10 — The Pilz Industrial Motion Planner

Unlike OMPL (which plans in joint space), **Pilz** plans **Cartesian motions** — straight lines and circles in workspace:

| Command | Motion type | Use case |
|---------|------------|----------|
| PTP | Point-to-point (joint space) | Fast repositioning |
| LIN | Linear (straight line in Cartesian) | Welding, gluing |
| CIRC | Circular arc | Polishing, painting |

### LIN planning

A straight line from pose A to pose B in Cartesian space:

```
Cartesian path:
  p(t) = p_A + t · (p_B - p_A)       for position (x, y, z)
  R(t) = R_A · exp(t · log(R_A⁻¹ · R_B))  for orientation (SLERP)

At each point along the line:
  1. Solve IK: q(t) = IK(p(t), R(t))
  2. Check collision: collision_free(q(t))?
  3. Check joint limits: q_min ≤ q(t) ≤ q_max?

If any step fails → LIN motion is infeasible.
```

### Cartesian velocity limits

From `pilz_cartesian_limits.yaml`:
```yaml
max_trans_vel: 1.0       # m/s
max_trans_acc: 2.25      # m/s²
max_trans_dec: -5.0      # m/s²
max_rot_vel: 1.57        # rad/s (≈ 90°/s)
```

These limit how fast the end-effector moves in workspace, independent of joint velocities.

---

## 19.11 — Example: Comparing Planners

Same target, different planners — observe how planning time and path quality change:

```python
#!/usr/bin/env python3
"""compare_planners.py — Plan the same motion with different OMPL planners."""

import time
import rclpy
from moveit.planning import MoveItPy

def plan_with(arm, planner_id, moveit):
    """Plan to pose_1 using the given planner, return (success, time, waypoints)."""
    arm.set_start_state_to_current_state()
    arm.set_goal_state(configuration_name="pose_1")

    # Configure the planner
    arm.set_planner_id(planner_id)
    arm.set_planning_time(5.0)           # max 5 seconds

    t0 = time.time()
    result = arm.plan()
    dt = time.time() - t0

    if result:
        n_points = len(result.trajectory.joint_trajectory.points)
        return True, dt, n_points
    return False, dt, 0

def main():
    rclpy.init()
    moveit = MoveItPy(node_name="planner_comparison")
    arm = moveit.get_planning_component("arm")

    # First, go to home (known start state)
    arm.set_start_state_to_current_state()
    arm.set_goal_state(configuration_name="home")
    result = arm.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])

    # Compare planners
    planners = ["RRTConnect", "RRTstar", "PRM", "KPIECE", "EST"]

    print(f"{'Planner':<15} {'Success':<10} {'Time (s)':<12} {'Waypoints'}")
    print("-" * 50)

    for planner in planners:
        ok, dt, pts = plan_with(arm, planner, moveit)
        print(f"{planner:<15} {str(ok):<10} {dt:<12.3f} {pts}")

    rclpy.shutdown()

if __name__ == "__main__":
    main()
```

### Typical output (varies — planners are randomized):

```
Planner         Success    Time (s)     Waypoints
--------------------------------------------------
RRTConnect      True       0.034        12
RRTstar         True       1.003        28       <- uses full time to optimize
PRM             True       0.128        15
KPIECE          True       0.067        18
EST             True       0.089        20
```

Key observations:
- **RRTConnect** is fastest — bidirectional trees meet quickly (Part 19.4)
- **RRT\*** uses more time to find a shorter path — more waypoints but smoother (Part 19.5)
- **PRM** is slower on first call but would be fast for repeated queries in the same scene (Part 19.6)

---

## 19.12 — Example: Cartesian Straight-Line Path

When you need the end-effector to follow a straight line (e.g. drawing, welding), you plan in workspace instead of joint space:

```python
#!/usr/bin/env python3
"""cartesian_line.py — Move the end-effector in a straight line (10cm forward)."""

import rclpy
from moveit.planning import MoveItPy
from geometry_msgs.msg import Pose, Point, Quaternion

def main():
    rclpy.init()
    moveit = MoveItPy(node_name="cartesian_example")
    arm = moveit.get_planning_component("arm")

    # Go to a known starting pose first
    arm.set_start_state_to_current_state()
    arm.set_goal_state(configuration_name="pose_1")
    result = arm.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])

    # Define a series of waypoints in Cartesian space
    # The end-effector will follow a straight line through these points
    robot_state = moveit.get_robot_model()
    robot_state = arm.get_start_state()

    waypoints = []

    # Current end-effector pose
    current_pose = arm.get_start_state().get_pose("tool_link")

    # Waypoint 1: 10cm forward (+X)
    target1 = Pose()
    target1.position.x = current_pose.position.x + 0.10
    target1.position.y = current_pose.position.y
    target1.position.z = current_pose.position.z
    target1.orientation = current_pose.orientation
    waypoints.append(target1)

    # Waypoint 2: 10cm to the right (+Y) 
    target2 = Pose()
    target2.position.x = target1.position.x
    target2.position.y = target1.position.y + 0.10
    target2.position.z = target1.position.z
    target2.orientation = current_pose.orientation
    waypoints.append(target2)

    # Compute Cartesian path (returns fraction of path achieved)
    trajectory, fraction = arm.compute_cartesian_path(
        waypoints,
        max_step=0.01,      # 1cm between interpolation points
        jump_threshold=0.0   # disable jump detection
    )

    print(f"Cartesian path computed: {fraction * 100:.1f}% achieved")
    # fraction = 1.0 means the entire line is feasible
    # fraction < 1.0 means the line hits a singularity, collision, or joint limit

    if fraction > 0.9:
        moveit.execute(trajectory, controllers=[])
    else:
        print("Path not fully feasible — the line may cross a singularity")

    rclpy.shutdown()
```

### What happens vs joint-space planning:

```
Joint-space plan (RRTConnect):       Cartesian path:
  End-effector trace:                  End-effector trace:
                                       
      ?                                    B
     / \    (curved, unpredictable)       /
    /   \                                / (straight line)
   A     B                              A

  - Fast to compute                    - Slower (IK at every 1cm step)
  - No control over workspace path     - Exact control over workspace path
  - Always finds a solution            - Can fail at singularities
```

The `max_step=0.01` parameter means IK is solved every 1cm along the line. For a 20cm path, that's ~20 IK calls — each one using the Jacobian method from Part 20.

---

## 19.14 — Summary: Choosing a Planner

```
Need a fast, general path?
  → RRTConnect (MoveIt default)

Need an optimal path?
  → RRT* (give it more planning time)

Same environment, many queries?
  → PRM (build roadmap once)

Straight line in Cartesian space?
  → Pilz LIN

Narrow passages?
  → KPIECE or EST

Circular arc?
  → Pilz CIRC
```

---

## 19.15 — C++ Example: Setting a Planner

```cpp
#include <moveit/move_group_interface/move_group_interface.h>

// Set the OMPL planner
arm.setPlannerId("RRTConnectkConfigDefault");  // or "RRTstarkConfigDefault", "PRMkConfigDefault"
arm.setPlanningTime(5.0);  // seconds
arm.setNumPlanningAttempts(10);

// Plan to a joint target
std::vector<double> joint_goal = {0.0, -1.57, 1.57, 0.0, 0.0, 0.0};
arm.setJointValueTarget(joint_goal);

auto [success, plan] = [&]() {
    moveit::planning_interface::MoveGroupInterface::Plan p;
    bool ok = (arm.plan(p) == moveit::core::MoveItErrorCode::SUCCESS);
    return std::make_pair(ok, p);
}();

if (success) arm.execute(plan);
```

---

## 19.16 — Quick Reference

| Concept | Key Point |
|---|---|
| RRT | Randomly grows a tree from start toward goal — probabilistically complete |
| RRT-Connect | Two trees (start + goal) growing toward each other — fast in practice |
| RRT* | Optimal RRT — rewires tree to find shorter paths (slower, asymptotically optimal) |
| PRM | Builds a roadmap graph offline — fast multi-query in static environments |
| OMPL | Open Motion Planning Library — C++ library behind MoveIt planners |
| Probabilistic completeness | If a solution exists, probability of finding it → 1 as time → ∞ |
| Narrow passages | Hard for sampling-based planners — KPIECE/EST can help |
| Pilz LIN/CIRC | Deterministic Cartesian planners — straight lines and arcs |
| Planning time | `setPlanningTime(seconds)` — more time = better paths (for RRT*) |
| C++ API | `arm.setPlannerId("RRTConnectkConfigDefault")` |

---

**Prev:** [Part 18 — Configuration Space](18-configuration-space.md)
**Next:** [Part 20 — Inverse Kinematics](20-inverse-kinematics.md)
**See also:** [Mathematics — Linear Algebra](../../mathematics/01-linear-algebra/README.md) — distance metrics for nearest-neighbor lookups in RRT / RRT* / PRM
