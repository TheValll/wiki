# Part 1 — How to Write a Great Research Paper

Seven concrete, actionable guidelines from **Simon Peyton Jones** (Microsoft Research) — distilled from his talk at MSR. The thesis: writing is not the output of research, it is the **machinery** of research. And a paper is a **vehicle for conveying an idea** from your head into the reader's head.

| # | Guideline | One-line picture |
|---|-----------|------------------|
| 1 | Start writing early | Writing is the forcing function that refines the idea, not a postscript to it |
| 2 | Identify your key idea | One paper, one "ping" — make it explicit, refutable, quotable |
| 3 | Tell a story | Problem → why interesting → my idea → evidence → related work |
| 4 | Nail the introduction | Page 1 is your most precious page: example + contributions as forward-referenced claims |
| 5 | Put related work at the end | It's a sandbar between the reader and your idea — move it after the payload |
| 6 | Lead the reader by the hand | Intuition first, examples first, no recapitulation of your journey |
| 7 | Listen | Guinea pigs tell you where they got lost; reviewers gave you the gift of their time |

---

## 1.1 — Start writing early

The wrong model: idea → do research for 12 months → start writing.

The right model: idea → **start writing** → the writing reveals what you didn't understand → do the research → refine the paper.

```
Wrong:    [ idea ]──────────── 12 months ────────────▶[ paper ]
Right:    [ idea ]─▶[ paper draft ]─▶[ research ]─▶[ paper final ]
                         ↑                              ↑
              forces you to articulate,       the research is now
              reveals gaps you did not        pointed at real gaps,
              know were there                 not imagined ones
```

Two payoffs:

- **Self-clarification.** Things you thought you understood turn out to be fuzzy once you try to write them down in sentences. The act of writing **crystallizes** the idea — you discover the gaps early, when there is still time to fix them.
- **Dialogue.** A draft is something you can share. Stuff swirling around in your head is hard for anyone else to engage with. A written draft opens a real conversation with collaborators.

> Writing is not the printer at the end of the algorithm. It is the algorithm.

And: don't wait until you have a "good" idea. Write about any idea. Most ideas seem weedy at first — you discover they were substantive only after you sit down to write 12 pages about them. Small, unpromising ideas often grow into real ones once forced through written exposition.

---

## 1.2 — Identify your key idea

A paper is a **vehicle**: it carries an idea from your head into the reader's head. If the transfer succeeds, the reader becomes a carrier — they take the idea to other people, build on it, cite it. If the transfer fails, the paper is heat.

Three consequences:

| Rule | Why |
|------|-----|
| **One paper, one idea** | Do not squeeze two or three ideas into 10 pages — none of them get the room they need. Write two or three papers instead. |
| **State the idea explicitly** | By the end of the paper, the reader must be in no doubt about what the key idea is. A `## The main idea` section is not overkill. |
| **Ideas are durable** | Implementations rot. Tools get replaced. A clearly stated idea can still be read and built on in 50 years. Papers are the most durable output of research. |

### The "one ping only" exercise

After you read someone else's paper, close it and ask yourself: *what is the one key idea I took away from this paper?*

You will be surprised how often you cannot answer. As an author, make the answer impossible to miss — literally label it:

```markdown
## The main idea
…
```

Leave the reader no doubt about the moment when the setup ends and the new contribution begins.

---

## 1.3 — Tell a story

The structure of a paper mirrors what you would do at a whiteboard explaining your work to a friend:

```
1. Here is the problem I am trying to solve.
2. Here is why it is interesting.
3. Here is why it is unsolved (state of the art cannot handle it).
4. Here is my idea.
5. Here is evidence that my idea works.
6. Here is how my idea compares to the alternatives.
```

This narrative scales from a blog post to a thesis. The medium changes the length, not the order.

### Conference-paper layout

```
┌──────────────────────────────────────────┐
│ Title, abstract                          │   ← readership: 100%
├──────────────────────────────────────────┤
│ Introduction: problem + contributions    │   ← readership: ~30% (after abstract)
├──────────────────────────────────────────┤
│ Problem in detail                        │   ← readership: ~10% (after page 1)
├──────────────────────────────────────────┤
│ The idea                                 │
├──────────────────────────────────────────┤
│ Evidence (theorems, measurements, cases) │
├──────────────────────────────────────────┤
│ Related work                             │
├──────────────────────────────────────────┤
│ Conclusion                               │
└──────────────────────────────────────────┘
```

Readership drops off sharply after page 1. Build your strategy around that fact.

---

## 1.4 — Nail the introduction (one page)

Page 1 is the **most precious real estate** in the paper. You have ~10× the readers here than on page 4. Spend that page on exactly two things: **the problem** and **your contributions**.

### Introduce the problem with an example

Not with abstract generalities. Four lines of prose, then an actual example — a program fragment, a figure, a concrete case. The reader should think *"ah, I get it"* before they finish the first paragraph.

### Molehills, not mountains

Do not set up the problem as the conquest of Mount Everest — no paper conquers Everest, and the reader knows it. Describe a **specific molehill** you are going to actually conquer.

```
Wrong:  "Computer programs have bugs. Bugs are very bad. [citations]"
        → demotivating, worthy but vague, reader thinks "yeah yeah"

Right:  "Consider this program. It has a particular class of bug.
         We show how to detect bugs of this class, with the
         following guarantees: …"
        → specific, believable, the reader wants to know how
```

### State the contributions as refutable claims

This is the single most under-used pattern. **Literally label the list** and write each entry as a claim someone could disprove:

```markdown
## Contributions

In this paper we:

- Prove that the type system is sound (§4).
- Show that type checking is decidable (§5).
- Demonstrate an implementation that compiles `benchmark_X` 3× faster
  than the state-of-the-art (§6, §7).
```

Three details that matter:

| Detail | Why |
|--------|-----|
| **Forward references** (`§4`, `§5`, …) | Each claim points to where it is substantiated. Reviewers can jump straight to the evidence. |
| **Refutable claims** | *"We study the properties of our system"* is irrefutable and useless. *"We prove property X"* is a claim. |
| **Garbage-collection check** | Any section not referenced from the contributions list is probably dead code — remove it, or promote it into a new bullet. |

### Kill the "rest of the paper" paragraph

> *"The rest of this paper is structured as follows. In Section 2 we describe … In Section 3 we …"*

Nobody reads it. You are wasting your most precious bytes. If you want forward references, embed them in the contributions list where each one actually means something.

```
┌──────────────────────────────────────────────────────────────────┐
│  Contributions are the specification of the paper. The body is   │
│  the implementation. If a reader believes your specification,    │
│  they want to read the implementation.                            │
└──────────────────────────────────────────────────────────────────┘
```

---

## 1.5 — Put related work at the end

Related work at the beginning looks like good scholarship. It is terrible. It is a sandbar between your reader and your idea:

- You must compress it, which makes it dense and hard to read.
- The reader has none of the scaffolding yet to understand the comparisons.
- It delays your idea by pages, and readership drops off every page.
- Worst of all: the reader feels **stupid and tired** before they even meet your idea.

Put it at the end. By then:

- The reader has the vocabulary, examples, and framework to read the comparisons fluently.
- The comparisons can be specific (*"unlike Smith [2014], we do X"*).

Does that mean you ignore prior work on page 1? No. As you lead the reader toward your idea, you can drop a bracketed citation when you cross someone's shoulders. But the **full dialogue** with the literature lives at the end.

### Credit is not zero-sum

Money is zero-sum: if I give you £10, I have £10 less. **Credit is not**. Praising someone's work does not diminish your paper — it brings credit to the whole field and makes the related-work section genuinely useful.

Avoid the trap of implicitly denigrating prior work (*"Brown and White tried X but it was hopeless"*). Instead, acknowledge honestly what is good about prior work, and then what the remaining gap is — the gap you are filling.

### State your own weaknesses

Computer science is multi-dimensional (speed, space, simplicity, environment, …). It is rare to beat prior work on every axis. If you don't mention where you are weak, a reviewer will — and they will write *"the authors appear unaware that their system is hopeless on axis Z"*. That is a bad thing for a reviewer to be able to say.

```
┌──────────────────────────────────────────────────────────────────┐
│  Say upfront: "Our system is not the right choice for situation  │
│  Z — use [other work] for that." This is disarming, honest,     │
│  and makes your claim on the axes where you DO win more           │
│  credible.                                                        │
└──────────────────────────────────────────────────────────────────┘
```

---

## 1.6 — Lead the reader by the hand

The body of the paper is evidence and exposition. Two traps to avoid.

### Don't recapitulate the journey

Your research journey was a maze full of dead ends and rotating knives. Do not drag the reader through every wrong turn. Lead them **straight to the goodness**. After a page and a half of technical material, the reader should never read *"…which turned out to be a really bad idea; we have a much simpler approach"*. That is what the paper is **for** — to spare them the dead ends.

The one exception: if an obvious-looking avenue happens to not work, and the reader would ask *"why did you not just do X?"*, acknowledge it in a sentence. Don't make them wonder.

### Intuition first, formalism second

Start every chunk with an **example**, not a general statement. Examples then generalisation, not the reverse. Even if the reader skips the formalism, they have taken something away.

- **Examples** give intuitive grounding.
- **Generalisation** promotes the intuition to a rule.
- **Formalism** makes the rule precise.

This is the same order that works for teaching — and the same order that works for talks. A paper is a teaching medium.

### Simplicity is a feature

If your final idea looks simple, the reader may suspect it is trivial. Counter this by showing (briefly) the complicated things others tried — or the complicated path you nearly took — then the simpler recast. Do not wander them through the complication yourself; gesture at it and reference someone else's paper.

> The best ideas are the simple ones. But you must frame the simplicity so the reader sees it as **hard-won**, not trivial.

---

## 1.7 — Listen

Two audiences give you feedback before publication: **guinea pigs** (friends you ask to read drafts) and **reviewers** (after submission).

### Guinea pigs

A guinea pig is a friend who reads your draft and tells you what they thought. Three rules:

| Rule | Why |
|------|-----|
| **Use them one at a time** | Nobody can read your paper for the first time twice. Save guinea pigs for successive drafts, not one big wave on day one. |
| **Tell them what you want** | Left on their own, they will send back spelling and grammar fixes. Those are useless at the draft stage. Look them in the eye and say: *"I want you to tell me where you got lost."* |
| **Then talk to them** | Written feedback is useful, but a conversation after a read-through is better — you will naturally articulate what you meant better than you did on paper, and you only need to write down what you end up saying out loud. |

Non-expert guinea pigs are still useful for the first few sections — that is the territory where you cannot afford any loss. Experts are more useful for the technical meat, but rarer.

### Reviewers: critique is a gift of time

When reviewers criticize your paper unfairly, the first instinct is to bleed. Resist it. Remember: they gave you **an hour of their life** you will never have back. It is a free gift. Thank them — literally, in the acknowledgments.

When you finish bleeding from a harsh review, flip the question:

> *"Reviewer is an idiot" → "How could I rewrite the paper so that **not even this idiot** could make that mistake?"*

Most reviewers are conscientious and have genuinely misunderstood something. That misunderstanding is diagnostic — it points at a place where the paper is unclear. You cannot argue the reviewer out of misreading it; you can only rewrite the passage.

---

## 1.8 — Summary

| Lever | The actionable move |
|-------|---------------------|
| **Start writing early** | Write the paper draft before finishing the research; let the writing reveal the gaps |
| **One key idea** | State it explicitly, put a `## The main idea` section if needed |
| **Tell a story** | Problem → why → my idea → evidence → related work |
| **Introduction = 1 page** | Example-driven problem + explicit refutable contributions with forward references |
| **Molehill, not mountain** | Frame the conquerable problem, not the research field |
| **Related work at the END** | Sandbar between reader and idea — move after the payload; credit generously |
| **No recapitulation** | Lead by the hand — intuition and examples first, no dead ends |
| **State your weaknesses** | Disarm the reviewer; acknowledge where the approach does not apply |
| **Guinea pigs, one at a time** | Draft early so you have rounds to spend; ask where they got lost |
| **Thank reviewers** | They gave you an hour of their life; treat critique as a rewrite prompt |

### The underlying mental shift

Writing is the **machinery of research**, not its output. A paper is a **vehicle for one idea**, and the vehicle has to be honest, reader-first, and tuned for page 1. Feedback — including painful feedback — is a **gift of time** from someone who will never have that hour back.

> See also: [How to Read a Paper](README.md) (planned — Keshav's three-pass method) for the other side of the same skill.

Source: Simon Peyton Jones, *"How to Write a Great Research Paper"*, Microsoft Research, 2016. Video: <https://www.youtube.com/watch?v=VK51E3gHENc>.
