# Wiki — Agent Schema

This file is the single source of truth for any LLM agent (Claude Code or other) working on this wiki. Read it fully before making any changes.

It is organized in four parts:

| Part | Scope | When you need it |
|------|-------|------------------|
| **I — Who I am** | User identity, goals, current work, collaboration expectations | Every conversation, first read |
| **II — How I learn** | Pedagogical profile: formats that land, articulation loop, signals | Every conversation, especially when explaining or drilling a concept |
| **III — Wiki rules** | Page conventions, ingest workflow, cross-references, Anki, what not to do | Every edit to wiki content |
| **IV — Adding a new domain** | Playbook for creating a new course folder (Pattern A vs B, READMEs, checklist) | When the user says *"I want to add a new domain X"* |

> Parts I and II describe *who* and *how*. Parts III and IV describe *the wiki* and *how to extend it*. Read I and II at the start of every new conversation; refer to III and IV as needed.

---

# Part I — Who I am

## 1. Profile

- **Name:** Valentin
- **Language:** French native · English improving
- **Studies:** M1 of *Mastère Intelligence Artificielle & Data Engineering* at **Efrei** — full alternance through M1 and M2 (no separate internships; M1 ends with a soutenance, M2 ends with a soutenance + mémoire + dossier professionnel)
- **Current role:** Alternance Ingénieur IA / Data Scientist at **Iceberg Data Lab** (2 years of alternance experience)
- **Location:** France

---

## 2. Long-term goal

**PhD in Robotics + AI at EPFL (Lausanne, Switzerland)** — target start **September 2028**.

I manage the multi-year roadmap myself; agents don't need to track it.

---

## 3. What I'm currently doing

### DeepSight-Nebula — active robotics project

- Repo: <https://github.com/TheValll/DeepSight-Nebula>
- Goal: transform an educational robot into a real-time capable robot
- Stack: ESP32 firmware (new), coming from Arduino background
- **Key deliverable:** a proper technical document / mini-thesis structure that will serve as a portfolio piece and an interview asset for PhD applications

### Learning tracks

| Track | Rhythm | State |
|---|---|---|
| **Math (Mathematics for Machine Learning — Deisenroth, Faisal, Ong)** | 2–3 concepts / evening | In progress — currently chapter 2 (linear algebra) |
| **Rust (official 2024 book)** | done | Finished — all 21 chapters |
| **Embedded Rust (*Simplified Embedded Rust* — Omar Hiari)** | ~20 pages / day (train commute) | In progress, for DeepSight-Nebula |
| **Robotics block (ROS2 review + DeepSight-Nebula)** | evenings, after train | ROS2 review bundled with DeepSight project work — single robotics slot |
| **Rust katas** | as time / fatigue allows | Side-fluency only; no fixed cadence |
| **Robotics papers** | 1 / week, weekends or free pockets | Starter pack defined (see `raw/papers/shortlist.md`) |
| **English** | immersion at work + daily journal entries (`daily/`) written in English | Natural progression |
| **Daily journal** (`daily/`) | 20 min / evening (log + articulation + gaps) | Started 2026-05-03 |

---

## 4. How I work / What I expect from an agent

- **Validation-driven:** I review every significant change before it lands. Propose before acting on non-trivial edits.
- **Structured:** I use this wiki as my long-term memory — synthesize, don't dump.
- **Learning orientation:** this wiki is a *learning tool*, not a reference database. Every page is manually validated.
- **Concise responses preferred:** I read diffs myself; no need to re-explain what changed.
- **French for conversation, English for wiki content.**
- **Don't be a yes-man** — challenge my plans honestly when you see blind spots.
- **When I share a new input** (paper, book, project idea), connect it to my long-term goals and flag what matters most.

---

# Part II — How I learn

This section is **maintained by the agent**. It captures observations about how I absorb, articulate, and retain information, so sessions can adapt automatically. I can read and correct it at any time; the agent updates it whenever a new pattern is observed.

## 5. Formats that land

### Step-by-step visual decomposition

Multi-frame ASCII schemas (one frame per step) land far better than a single dense diagram. Each frame isolates one transformation — e.g., for the dot product: (1) the two vectors, (2) drop the perpendicular, (3) split into parallel + perpendicular pieces, (4) read the result. Breaking the motion into discrete frames lets me mentally replay the operation later.

### Physical / mental analogies (scales beyond geometry)

Mechanical images stick: a cart being pushed, a rope being pulled, a rubber sheet being deformed, a shadow on the floor. Once the physical image is in place, the formal mechanism slots in cleanly. Favor analogies involving forces, shadows, projections, and motion — mechanical, not abstract.

This pattern works **even for non-geometric concepts**. Validated on Rust (2026-04-18): ownership = holding a package, borrowing = lending a book, lifetimes = expiration dates on loans, iterators = a conveyor belt, closures = a chef who grabs from the pantry. The more mechanical the image, the better — even when the underlying concept has no visual form at all.

### Simple sentences and conversational tone

No academic register. Short sentences. Plain words. Meta-comments like "this is the elegant part" or "c'est ça la magie" serve as attention cues. Dense, jargon-heavy paragraphs cause drift.

### Tables for consolidation

End-of-section tables (3 columns max) summarize the case analysis and serve as locking mechanisms. The "three cases" table for the dot product was immediately read as a recap that closes the concept.

---

## 6. Articulation-based correction loop

### "Si je comprends bien..."

After an explanation, I reformulate the concept in my own words — sometimes with slight errors or inversions. This is my verification loop: I state a rough model, the agent corrects the misalignment, the final form locks in. **Do not rewrite my draft silently.** Pinpoint exactly which word, direction, or piece was off, and why. The precise correction step is load-bearing for retention.

### Mental image before mechanism

If the mental image is clear (the cart, the rubber sheet, the shadow), I retain the mechanism long-term. If the mechanism is presented first without an image, retention is fragile. Order to follow: **physical image → decomposition → mechanism → formula** (formula last, or not at all in intuition mode).

### Under-the-hood reasoning is primary

I retain a concept better once I understand *why* it works, not just how to apply it. Derivations are welcome, but only after the intuition is in place. Premature abstraction loses me.

---

## 7. Signals to watch

| Signal | Meaning | Response |
|---|---|---|
| "j'ai compris", "c'est super utile" | Solid, ready to advance | Move on; save / consolidate |
| "si je comprends bien..." | Articulating a draft model | Listen for inversions or mismatches; correct precisely, not globally |
| Short neutral reply, no follow-up | Not fully locked | Offer a complementary angle (schema or analogy) before moving on |
| Explicit request for a schema | Words alone are insufficient for this concept | Always comply; flag the pattern for similar concepts in the future |
| "je marche comme ça" | Naming a personal learning mechanism | Note it here for future sessions |

---

## 8. Pedagogical don'ts

- Don't explain a geometric or algebraic concept formula-first — the formula is the last step, not the first.
- Don't bundle multiple concepts into one session block without my agreement — one concept, layered, beats three concepts, shallow.
- Don't rephrase my articulation draft as if you never heard it — name the delta, don't overwrite it.

> *Last updated by the agent: 2026-04-18 — initialized after multi-session linear algebra intuition work (sections 1.1 Norm, 1.2 Dot Product). Patterns observed over ~4 sessions.*

---

# Part III — Wiki rules

## 9. Page format conventions

All domain pages follow the same visual grammar (look at any existing file for the pattern). Preserve it when editing or creating new pages.

| Element | Convention |
|---------|------------|
| **Title** | `# Part N — Topic Title` |
| **Intro** | 1-3 sentences framing the topic, often with a small table |
| **Sections** | `## N.1 — Subtitle`, `## N.2 — Subtitle`, … |
| **Code blocks** | Fenced with triple backticks + language tag (```rust, ```cpp, ```bash) |
| **Tables** | Used heavily for comparisons, options, summaries |
| **ASCII art boxes** | For key insights — `┌─────┐` rounded-corner style, used sparingly for emphasis |
| **Cross-references** | Standard markdown links `[text](path/to/page.md)` — Obsidian renders both these and `[[wiki-links]]` but we use standard markdown for portability |
| **Summary section** | Most pages end with a `## N.X — Summary` table |
| **Language** | English for all wiki content (user is French but wants the wiki in English for consistency with source material) |

**Do NOT:**

- Add emojis unless the user asks
- Write multi-paragraph docstrings or verbose comments in code blocks
- Create pages with only prose — always include tables, code, or diagrams
- Use `<br>` or HTML — stick to pure markdown

---

## 10. Ingest workflow (from `raw/`)

When the user says *"ingest `raw/<file>`"*:

1. **Read** the raw file completely
2. **Ask questions before acting:**
   - Which **domain(s)** does this belong to? (rust, ros2, math, new domain?)
   - What's the **focus** — what should be emphasized?
   - **New page** or **update existing**? If update, which one(s)?
   - **Granularity** — one dense page or split into several?
3. **Propose** a plan (don't write files yet): list the pages you'd create/edit, with 1-line summaries
4. **Wait for validation**
5. **Then** write the pages following the format conventions
6. **Update indexes:** the relevant domain's `README.md`, and the root `README.md` if the domain changes or cross-references are added

```
┌──────────────────────────────────────────────────────────────────┐
│  Never auto-ingest without user validation. This is a            │
│  LEARNING wiki — the user wants to review before content lands.  │
└──────────────────────────────────────────────────────────────────┘
```

---

## 11. Updating existing pages

- Prefer `Edit` over `Write` (sends only the diff)
- Preserve the existing tone, structure, and level of detail
- If adding a section, place it in a logical spot and update the `Summary` table at the end
- If the change affects the domain index, update that domain's `README.md`
- If the change introduces cross-domain links, update the root `README.md`

---

## 12. Cross-references

When a concept spans domains, add a **"See also"** line at the end of the relevant section:

```markdown
> See also: [Linear Algebra — Matrix × Vector](../mathematics/01-linear-algebra/1.3-matrix-vector.md)
```

### Path conventions

| Relationship | Convention |
|---|---|
| Same folder | `[text](./file.md)` |
| Same domain, different chapter | `[text](../other-chapter/file.md)` |
| Different domain | `[text](../../other-domain/chapter/file.md)` |
| Wiki root file (CLAUDE, README) | `[text](../CLAUDE.md)` from `<domain>/`, or `[text](../../CLAUDE.md)` from a chapter folder |

### Known cross-domain connections

| From | To | Why |
|------|----|----|
| `ros2/moveit/20-inverse-kinematics.md` | `mathematics/01-linear-algebra/` | Jacobian, rotation matrices |
| `ros2/moveit/19-motion-planning.md` | `mathematics/` | Distance metrics, sampling |
| `embedded/` | `rust/` (no_std patterns), `low-level/` (planned), `electronics/` (planned) | Registers, GPIO, protocols, ESP HAL |
| `ml/`, `dl/` (future) | `mathematics/03-derivatives/` | Gradients, backprop |

### When you add a new cross-domain link

1. Add a `> See also: [other domain — concept](path)` line at the end of the relevant section.
2. Update the source chapter's `README.md` "Applied in" table.
3. Update the target chapter's `README.md` if appropriate.
4. Update the root `README.md` "Cross-domain connections" table for high-traffic links.
5. Update the table above if the link represents a recurring pattern.

---

## 13. Anki cards — markdown source → Anki

**Scope: math only.** Anki is used for declarative knowledge (formulas, definitions, identities, theorems). Rust / ROS2 / embedded use practice instead — no Anki for procedural knowledge.

**Authorship: self-made.** Valentin writes his own cards (generation effect). The agent does NOT propose, generate, or pre-fill Anki cards by default, even when atomic facts surface. If explicitly asked *"give me a card on X"*, comply.

**Language: English.** Front and back of every card are written in English — same convention as the rest of the wiki (cf. §9) and as `daily/` entries. Doubles as English practice.

### Format spec — one concept, two manually-reversed cards

````markdown
## 1.1.X — Anki

TARGET DECK: Mathematics::01-linear-algebra

START
Basic
Front: What is the formula of the Euclidean norm in $\mathbb{R}^n$?
Back: $\|v\| = \sqrt{\sum_{i=1}^n v_i^2}$ — Pythagoras cascaded across $n$ perpendicular axes.
END

START
Basic
Front: $\|v\| = \sqrt{\sum_{i=1}^n v_i^2}$ — what concept does this formula represent?
Back: Euclidean norm in $\mathbb{R}^n$.
END
````

### Card-writing conventions

- **Front**: one line ending with `?`. A simple question on the concept (definition / formula / property).
- **Back**: one to two lines max. LaTeX formula + short intuition.
- **Atomic**: one fact per card. A page with 5 atomic concepts → 10 cards (5 Q→A + 5 A→Q).
- **Two directions per concept**: write the reverse manually. Reformulating the question forces a second pass of generation effect, which is in fact better for retention than mechanically auto-generated reverses.
- **Avoid open-ended questions** ("explain the dot product") — articulation happens in `daily/`, not in Anki.
- **Source lens**: the card reflects how the currently-read source frames the concept (typically MML for math).

---

## 14. What NOT to do

- **Don't auto-ingest** — always validate with the user first (see §10).
- **Don't bypass validation** for non-trivial structural changes — propose before acting.
- **Don't create documentation files beyond wiki pages** unless explicitly requested (no TODO.md, NOTES.md, status reports).
- **Don't merge domains** — if a topic straddles two domains, write in the primary domain and cross-reference.
- **Don't reorganize or renumber historical pages** without discussing — numbering is stable (e.g., `13-tests.md` stays `13-tests.md` even if inserting later).
- **Don't bundle multiple concepts into one Pattern A file** — split, don't bundle. (Exception: Pattern B layered topics, see §15.)
- **Don't re-explain concepts already in the wiki** — link to the existing page instead.
- **Don't touch `.obsidian/`** — it's the Obsidian app config, managed by the app itself.

---

# Part IV — Adding a new domain

This part is the **playbook** for adding a new domain to the wiki (e.g. `low-level/`, `electronics/`, `ml/`, `dl/`). It captures the conventions distilled from the existing domains (`mathematics/`, `rust/`, `ros2/`, `embedded/`, `research/`).

When the user says *"I want to add a new domain X"*, **follow this part step by step**.

## 15. Two structural patterns

Before writing anything, pick which pattern fits the domain. The choice depends on the *natural granularity* of the source material.

### Pattern A — **Atomic concept** (math, rust, future ml/dl)

Each chapter folder, each file inside is **one concept**. A "concept" = something you could explain in 5-15 minutes standing at a whiteboard. Used when the source material is built from small, composable building blocks.

```
<domain>/
├── README.md           ← syllabus + reading order
├── RECAP.md            ← single-glance table of every concept
├── 01-<chapter-slug>/
│   ├── README.md       ← chapter TOC + Applied-in table
│   ├── 1.1-<slug>.md   ← one concept = one file
│   ├── 1.2-<slug>.md
│   └── …
├── 02-<chapter-slug>/
└── …
```

**Examples:**

- `mathematics/01-linear-algebra/` → 9 concept files (`1.1-norm.md`, `1.2-dot-product.md`, …)
- `rust/05-ownership/` → 10 concept files (`5.1-why-ownership.md` through `5.10-return-values.md`)

**Per-file structure** (atomic pattern):

```markdown
# X.Y — Concept Title

**What it does:**
1-2 sentences on purpose + a short mechanical analogy.

## Intuition — <subtitle>
Physical image → step-by-step decomposition → "what to remember" closure.
ASCII art frames preferred for transformations (one frame per step).

## Formula
Code-block math.

## Simple example
Minimal numeric case.

## Complex example
Realistic scenario from robotics / ML / physics.

## See also (optional)
Cross-domain links.
```

### Pattern B — **Layered topic** (ros2, embedded)

Each file = **one topic**, internally layered into ~6-8 facets (Analogy → Concepts → Code → Math → Flow). Used when the source material is built from larger units that are pedagogically inseparable (e.g. "Topics & Pub/Sub" needs all of: analogy + publisher code + subscriber code + QoS + serialization to be useful).

```
<domain>/
├── README.md           ← syllabus + reading order
├── RECAP.md            ← single-glance table (1 row per file)
├── <block-1>/          ← optional sub-folder grouping
│   ├── 00-<slug>.md
│   ├── 01-<slug>.md
│   └── …
└── <block-2>/
```

**Example:**

- `ros2/basics/02-topics-pub-sub.md` covers Analogy / Topics / Publisher (deep dive) / Subscriber (deep dive) / QoS / Serialization / Python comparison / Flow diagram — all internal `## 2.1 — …` to `## 2.8 — …` sections inside one file.

**Per-file structure** (layered pattern):

```markdown
# Part N — Topic Title

Intro: 1-3 sentences framing the topic, often with a small table.

## N.1 — The Analogy
Physical image / mechanical analogy. Often the best opener.

## N.2 — The Core Concept
The thing itself, plainly stated.

## N.3 — Code Deep-Dive
Fenced code blocks with language tag (```cpp / ```python / ```rust).

## N.4 — Comparison / Variant
e.g. C++ vs Python.

## N.5 — Math / Internals
Formulas, memory layouts, protocol details when applicable.

## N.6 — Flow / Sequence Diagram
ASCII art showing how things connect over time.

## N.X — Quick Reference (optional)
Table summarizing options, signatures, or "when to use what".

**Next:** [Part N+1 — …](next-file.md)
```

### Decision rule

| Source material… | Use Pattern |
|---|---|
| Textbook chapters with many small theorems / definitions | **A** (atomic) |
| Programming language with many small features | **A** (atomic) |
| Framework / system whose pieces only make sense together | **B** (layered) |
| Tutorial that teaches end-to-end builds | **B** (layered) |

---

## 16. READMEs and RECAP

### Per-chapter `README.md` (Pattern A only)

Every chapter folder has a `README.md` that is the chapter TOC. Format:

```markdown
# NN — Chapter Title

Each page covers **one concept**. Course content (formula, examples) and intuition merged.

| § | Page | In one line |
|---|------|-------------|
| X.1 | [Concept Name](./X.1-slug.md) | 1-line essence in plain English |
| X.2 | [Concept Name](./X.2-slug.md) | … |

## Applied in (optional)

| Concept | Used in |
|---------|---------|
| **Concept A** | [Other Domain — Page](../../other/page.md) — 1-line context |
```

The "Applied in" table makes cross-domain reuse visible. Add it whenever the chapter's concepts are referenced from another domain.

### Domain `README.md` (always required)

Every domain has a top-level `README.md` with:

1. **One-line scope statement** at the top.
2. **At-a-glance pointer** to the `RECAP.md`.
3. **Companion pointer** if any (e.g. `rust/RECAP.md` as a bookmark layer, future `*-intuition.md` files).
4. **Conventions section** — note the structural pattern used (A or B), naming rules.
5. **Syllabus** — chapter list with links to each chapter folder's README (Pattern A) or each topic file (Pattern B). Group into blocks if the domain has > 10 entries.
6. **Reading order & prerequisites** — for domains with non-linear dependencies (Pattern B especially), include an ASCII DAG and "skip rules" listing the minimal prereqs to jump to a specific topic. See `ros2/README.md` for the canonical example.

### `RECAP.md` (always required)

Single-glance table of every concept in the domain. One row per concept-file (Pattern A) or per topic-file (Pattern B). Three columns:

```markdown
| Concept | What / for what (1-2 sentences) | Intuition (1 phrase) |
```

Group rows by chapter (Pattern A) or by block (Pattern B). The whole RECAP should be readable in under a minute.

Link the RECAP from the domain `README.md` with `> **At a glance:** [RECAP.md](./RECAP.md)`.

---

## 17. Launch checklist

Use this when the user says *"I want to add a new domain X"*:

- [ ] **Validate scope with the user.** What's the source material? What's the goal? Is it linear or non-linear?
- [ ] **Pick the structural pattern** (A atomic or B layered). State your reasoning.
- [ ] **Propose folder + naming convention** before any file is written.
- [ ] **Wait for explicit validation** before mass file creation (§14: never auto-ingest).
- [ ] **Create the domain folder** with an empty `README.md` skeleton.
- [ ] **Write the first 1-2 chapters** as a pilot. Iterate the format with the user before scaling.
- [ ] **Build the chapter `README.md`** for each chapter as you go.
- [ ] **Build the domain `RECAP.md`** once the structure stabilizes.
- [ ] **Add the reading-order DAG** in the domain `README.md` if dependencies are non-linear.
- [ ] **Update root `README.md`** — add the domain to the Domains table + cross-domain connections.
- [ ] **Update §12** cross-domain table if applicable.
- [ ] **Consider an atlas companion** if the domain has flows, schemas, or notation worth capturing as a long-term artifact (e.g. `<domain>/<domain>-flows.md`, `<domain>/notation.md`). Optional — only when there's a clear visual/symbolic payload to preserve.

---

## 18. Maintenance scripts

Two recurring scripts proven useful during the math + rust + ros2 refactos:

### A. Split a multi-concept main file into per-concept files (Pattern A)

```bash
extract_section() {
  awk -v sec="^## $2 —" -v next_pat="^## " '
    $0 ~ sec { found=1; print; next }
    found && $0 ~ next_pat { exit }
    found { print }
  ' "$1" | sed '1s/^## /# /'
}

extract_section old-chapter.md "1.1" > new-chapter/1.1-slug.md
```

### B. Bulk-rewrite cross-references after a structural move

Use Python with `os.walk` + regex. The `ros2/` reorg used a Python one-shot script to:

1. Compute target folder for each chapter number.
2. Rewrite same-domain inter-file links to use the new sub-folder paths.
3. Bump `../mathematics/` → `../../mathematics/` for files moved one level deeper.

Pattern:

```python
def folder_for(n):
    n = int(n)
    if n == 0: return "setup"
    if 1 <= n <= 7: return "basics"
    # ...

# For each link, compute target folder and rewrite if cross-folder.
```

After any structural change: run `grep -rn "<old-pattern>" . --include="*.md"` to verify zero stale references.
