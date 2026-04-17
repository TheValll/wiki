# Research Papers

Research papers to read, with notes. This is a **personal reading log** — not ingested into the wiki automatically. Papers accumulate here; ideas worth synthesizing get pulled into the relevant domain (`robotics/`, `ml/`, `mathematics/`…) over time.

## Goal

Build up a reading habit and a searchable archive of what has been read, for two purposes:
1. **PhD application prep** — EPFL EDRS candidacy targets Sept 2028. Jurys expect familiarity with the literature of the target labs and subfield.
2. **Mental model building** — papers expose you to how actual researchers frame problems, which is different from textbook material.

## Directory layout

```
raw/papers/
├── README.md          ← this file
├── template.md        ← template for a new paper note (copy when starting a read)
├── shortlist.md       ← ordered reading list (starter pack + ongoing additions)
├── 2026-04-17_keshav-how-to-read-a-paper.md
├── 2026-04-24_cadena-slam-survey.md
└── ...
```

## Naming convention

```
YYYY-MM-DD_first-author-short-title.md
```

Date = the day you **started** reading it.

Examples:
- `2026-04-17_keshav-how-to-read-a-paper.md`
- `2026-05-03_chi-diffusion-policy.md`

## Reading method — Keshav 3-pass

See [Keshav — How to Read a Paper](https://web.stanford.edu/class/ee384m/Handouts/HowtoReadPaper.pdf) (3 pages).

| Pass | Duration | Goal |
|------|----------|------|
| **1** | ~10 min | Abstract, intro, section titles, figures, conclusion. Decide: worth continuing? |
| **2** | ~1 h | Read carefully, grasp the approach, read figures thoroughly. Don't worry about math details. |
| **3** | ~4-5 h | Virtually re-implement. Question every choice. Only for core papers. |

Current target: **1 paper / week (Pass 1 + Pass 2)** + **1 paper / month in depth (Pass 3)**.

## Workflow

1. Pick next paper from `shortlist.md`
2. Copy `template.md` → `YYYY-MM-DD_firstauthor-title.md`
3. Fill it progressively as you read
4. When done, check it off in `shortlist.md`
5. If the paper contains material worth synthesizing into the wiki, mention it to Claude for ingestion

## Useful tools

| Tool | Use |
|------|-----|
| [arxiv-sanity-lite.com](https://arxiv-sanity-lite.com/) | Filtered arXiv feed by topic |
| [Connected Papers](https://www.connectedpapers.com/) | Given a seed paper, view its neighborhood graph |
| [Semantic Scholar](https://www.semanticscholar.org/) | Auto-generated TL;DRs |
| [Papers With Code](https://paperswithcode.com/) | ML papers + implementations |
| Google Scholar alerts | Auto-email when a followed author publishes |

## Venues to know

| Type | Name | Role |
|------|------|------|
| **Conference** | ICRA | The big one in robotics (~2000 papers/year) |
| **Conference** | IROS | Second major robotics venue |
| **Conference** | RSS | Smaller, very high quality |
| **Conference** | CoRL | Robot learning / ML × robotics |
| **Journal** | T-RO | IEEE Transactions on Robotics |
| **Journal** | RA-L | Short format, widely read |
| **Journal** | IJRR | Historic robotics journal |
| **Preprint** | arXiv `cs.RO` | Everything appears here before publication |

## Target labs (EPFL, for reference)

- **LSRO** (Bleuler) — medical / surgical robotics
- **TNE** (Micera) — neural interfaces, neuroprostheses
- **BioRob** (Ijspeert) — biorobotics, locomotion, exoskeletons
- **eSpace** (EPFL Space Center) — orbital debris, in-orbit servicing
- **LIS / MOBOTS** — mobile robotics, drones

Follow these authors on Google Scholar for auto-updates.
