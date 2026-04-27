# Big Data Framework — M1 Efrei (2025-2026)

> Course by **Steve Elanga** (Senior Data Engineer). 5 modules covering Big Data fundamentals, Hadoop, and Spark.

> **At a glance:** [RECAP.md](./RECAP.md)

---

## Course objectives

- Understand the **genesis** of Data jobs and differentiate them.
- Master the **key concepts** of Big Data (5V, structures, Parquet, storage architectures).
- Understand the **medallion architecture** of a Lakehouse.
- Understand how **Hadoop** works (HDFS, MapReduce, YARN).
- Master the **fundamentals of Spark** (RDD, DataFrames, DAG).
- Combine Hadoop + Spark to solve a real business problem.

---

## Evaluation

| Component | Weight |
|---|---|
| Participation bonus | — |
| Spark project (Lakehouse, due Sunday after last class) | 60% |
| QCM (30 questions) | 40% |

---

## Syllabus

| § | Module | Pages |
|---|---|---|
| 1 | [**Concepts clés**](./01-concepts.md) — 5V, structures, Parquet, storage, DFS, Cloud, virtualization | one file |
| 2 | [**Métiers de la Data**](./02-data-jobs.md) — Data Engineer, Analyst, Scientist | one file |
| 3 | [**Écosystème Hadoop**](./03-hadoop.md) — HDFS, MapReduce, YARN, Hive | one file |
| 4 | [**Écosystème Spark**](./04-spark.md) — RDD, DataFrames, architecture, DAG, Spark SQL, persist, submit | one file |

---

## Reading order

```
01-concepts.md   (foundations)
      ↓
02-data-jobs.md  (context / jobs)
      ↓
03-hadoop.md     (first generation framework)
      ↓
04-spark.md      (second generation, builds on Hadoop concepts)
```

Spark assumes you know HDFS, MapReduce, and YARN — read `03-hadoop.md` first.

---

## Cross-domain links

| Concept | Linked to |
|---|---|
| Spark MLlib, distributed ML | [`../../ml/README.md`](../../ml/README.md) |
| MapReduce shuffle, hash partitioning | (algorithmic — no current wiki page) |

---

## Atlases / companions

None for this course (compact enough for 4 module files).
