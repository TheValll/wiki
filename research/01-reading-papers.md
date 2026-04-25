# Part 1 — How to Read a Paper

A practical method from **S. Keshav** (University of Waterloo) for reading research papers efficiently. The thesis: reading a paper is **not** a single linear read — it is a sequence of up to **three passes**, each with a distinct goal, and you stop as soon as your need is met.

| Pass | Time | What you get | When to stop |
|------|------|--------------|---------------|
| 1st | 5-10 min | Bird's-eye view: what kind of paper, what it claims | Paper is irrelevant, too far from your area, or built on bad assumptions |
| 2nd | ~1 hour | Content without details — enough to summarise to a colleague | Paper is interesting but outside your speciality |
| 3rd | 1-5 hours | Full understanding, ability to reconstruct the paper from memory | When you must review, build on, or critique the work |

The pay-off is **dose control**: you spend the right amount of time per paper, not the same amount on every paper.

---

## 1.1 — Why three passes (not one)

The wrong model: open the PDF on page 1, read every word top to bottom, give up around page 6, never finish.

The right model: each pass has a **defined goal** and a **defined stop condition**. You only pay for the depth you need.

```
Pass 1 ─▶ "Should I keep going?"           5-10 min
   │
   ├─▶ no  →  done. The paper has been triaged.
   │
   └─▶ yes
         │
         Pass 2 ─▶ "Do I now grasp the paper?"   ~1 hour
            │
            ├─▶ enough  →  done. I can summarise it.
            │
            └─▶ not enough, or I have to review it
                   │
                   Pass 3 ─▶ "Can I reconstruct it?"   1-5 hours
```

A typical researcher spends hundreds of hours a year on papers. The three-pass approach is a defence against drowning in details before having a bird's-eye view, and a way to estimate how long a stack of papers will take.

---

## 1.2 — First pass: the bird's-eye view

Five to ten minutes. Four moves:

| Step | What to do |
|------|------------|
| 1 | Carefully read the **title, abstract, introduction** |
| 2 | Read every **section and sub-section heading** — ignore the body |
| 3 | Read the **conclusions** |
| 4 | Glance over the **references**, ticking the ones you have already read |

At the end, you should be able to answer the **five Cs**:

```
┌──────────────────────────────────────────────────────────────────┐
│  Category    — measurement / analysis / prototype / position?    │
│  Context     — what other papers / theory does it build on?      │
│  Correctness — do the assumptions look valid?                    │
│  Contributions — what is the paper's main contribution?          │
│  Clarity     — is it well written?                               │
└──────────────────────────────────────────────────────────────────┘
```

If you cannot answer the five Cs after a careful first pass, the paper itself is probably the problem — many papers do not survive a first pass even by experienced readers.

### When to stop after pass 1

Stop here when:

- The paper is **outside your area** but might one day be relevant — knowing it exists is enough.
- The paper does **not interest you** (the contributions are not what you thought).
- The **assumptions are invalid** for your use case.
- You **lack the prerequisites** to go deeper — note it for later, after you have read background material.

> If you are writing a paper, remember: **most readers will only ever do pass 1**. Pick coherent headings, write a concise abstract — the page-1 contract has to deliver in 5 minutes or it is over. See also: [Part 2 — How to Write a Great Research Paper § 2.4](02-writing-papers.md#24--nail-the-introduction-one-page).

---

## 1.3 — Second pass: grasping the content

Up to one hour. Read the paper carefully, but **skip the details** — proofs, derivations, edge cases. The goal is comprehension of the **argument**, not validation of the math.

Two practical moves:

| Move | Why |
|------|-----|
| **Inspect the figures** | Are axes labelled? Are error bars shown? Are units sane? Sloppy figures are a tell — they separate rushed work from careful work. |
| **Mark unread references** | Anything cited that you have not read and that looks central — write it down. This is how literature surveys grow. |

After a successful second pass, you should be able to **summarise the main thrust of the paper, with supporting evidence, to a colleague** — without re-opening the PDF.

### What to do when pass 2 fails

You finish the hour and you still don't get it. Three honest options:

```
(a) set the paper aside  ─ if you can succeed in your career without it
(b) come back later      ─ after you have read background material
(c) push to pass 3       ─ if you must understand it (review, foundational paper, ...)
```

Causes for a failed second pass: unfamiliar terminology, an experimental technique you don't know, a poorly-written paper, or simply late-night fatigue. The disciplined response is to **name the cause** and act on it, not to grind another hour blindly.

---

## 1.4 — Third pass: virtual re-implementation

One to five hours. The reader becomes a **co-author in their head**.

The technique:

```
For every claim:
    1. Make the same assumptions as the authors.
    2. Re-derive / re-build the argument yourself.
    3. Compare your reconstruction with the actual paper.
    4. The differences expose: hidden assumptions, weak proofs,
       missing citations, presentation choices the authors made.
```

Two reflexes during this pass:

- **Challenge every assumption** in every statement. If you cannot justify it, neither can the authors — that is a finding.
- **Ask "how would I have presented this?"** The comparison is where you steal proof and presentation techniques for your own future papers.

Output of a successful third pass:

| You should be able to | And to identify |
|-----------------------|-----------------|
| Reconstruct the paper's structure from memory | Implicit assumptions |
| Critique strong and weak points | Missing citations to relevant work |
| Use the techniques in your own work | Issues with experimental or analytical methodology |

Beginners: ~4-5 hours. Experienced researchers in their own field: ~1 hour. The gap closes with practice and with prior knowledge of the area.

```
┌──────────────────────────────────────────────────────────────────┐
│  Pass 3 is reading as construction. You re-implement the paper   │
│  in your head — and the gap between your version and theirs is   │
│  where the real learning happens.                                 │
└──────────────────────────────────────────────────────────────────┘
```

---

## 1.5 — Doing a literature survey

Reading a single paper is one skill. Mapping a whole field — tens of papers in a domain you don't yet know — is the same skill applied iteratively.

### The three-step procedure

```
Step 1 ─ Seed papers
   │
   │   Search Google Scholar / CiteSeer with well-chosen keywords.
   │   Find 3-5 recent papers in the area.
   │   Pass 1 each. Read their related-work sections.
   │   ─▶ Lucky: you find a recent survey paper. Read it. Done.
   │
Step 2 ─ Key papers and people
   │
   │   In the bibliographies of the seed papers, look for:
   │     - shared citations  →  the canonical papers in the area
   │     - repeated authors  →  the active researchers in the area
   │   Download the key papers. Visit the authors' websites — see
   │   where they have published recently. Those venues are the
   │   top conferences in the field.
   │
Step 3 ─ Recent proceedings
   │
   │   Browse the recent proceedings of the top conferences.
   │   A pass-1 scan identifies high-quality recent work.
   │
   ▼
First version of your survey
   │
   │   Pass 2 each of the assembled papers.
   │   If they all cite a paper you do not have, fetch it. Iterate.
```

The procedure is **iterative**: each round of reading gives you better keywords, better authors, and better conferences for the next round. Stop when new rounds stop adding new key papers.

### Why this works

Reading a field cold is hopeless — you don't know what is important. The survey procedure exploits the **citation graph and the reputation graph** in tandem:

- Shared citations → the field's gravitational centres.
- Repeated authors → the people whose websites are worth visiting.
- Top conferences → where the best work appears now.

You convert a field you don't know into a small set of must-reads.

---

## 1.6 — Connecting to the PhD path

For the PhD trajectory (see [`../me.md`](../me.md)), this is not a one-off skill — it is a **weekly muscle**. Three concrete pay-offs:

| Pay-off | How |
|---------|-----|
| **Triage** | A robotics conference may publish 200 papers a year. Pass-1 each to triage 50 in 8 hours instead of reading 5 deeply and missing the rest. |
| **Survey** | Before writing a paper or a thesis chapter, the three-step procedure converts a field you don't know into a reading list. |
| **Reviewing** | Reviewing for a conference forces a pass-3 reading. Use the time well — and write the kind of review you would want to receive (see [Part 2 § 2.7](02-writing-papers.md#27--listen)). |

The reading skill is the **mirror image** of the writing skill: a paper that is hard to triage in pass 1 has a failed introduction; a paper that is hard to grasp in pass 2 has a failed narrative; a paper that resists pass 3 has a failed argument. Reading well teaches you to write well.

> See also: [Part 2 — How to Write a Great Research Paper](02-writing-papers.md) — the other side of the same skill.

---

## 1.7 — Summary

| Pass | Time | Output | Stop condition |
|------|------|--------|-----------------|
| **1st — bird's-eye** | 5-10 min | The five Cs (Category, Context, Correctness, Contributions, Clarity) | Paper irrelevant, too far, or built on bad assumptions |
| **2nd — content** | ~1 hour | Can summarise main thrust + supporting evidence to a colleague | Paper outside your speciality but understood |
| **3rd — re-implementation** | 1-5 hours | Can reconstruct the paper from memory; identify hidden assumptions | When reviewing, building on, or critiquing the work |

| Activity | Workflow |
|----------|----------|
| **Triaging a stack of papers** | Pass 1 on each. Maybe 1 in 5 promotes to pass 2. |
| **Reading something in your area** | Pass 1 → pass 2. Rarely pass 3 unless you cite or build on it. |
| **Reviewing a submission** | Pass 1 + 2 + 3 — that is what reviewing is. |
| **Surveying a new field** | Three-step procedure (seeds → key papers/authors → recent proceedings), iterating. |

### The underlying mental shift

Reading a paper is **dose-controlled**, not all-or-nothing. Each pass has a defined goal, a defined cost, and a defined stop condition. The reader controls the depth — the paper does not.

Source: S. Keshav, *"How to Read a Paper"*, ACM SIGCOMM Computer Communication Review, 2007. PDF: <https://www.cs.umd.edu/~mwh/courses/CMSC818W/keshav-paper-reader.pdf>.
