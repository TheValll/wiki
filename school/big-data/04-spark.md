# Part 4 — Spark Ecosystem

The **second-generation** Big Data framework. Fixes MapReduce's limits with **in-memory** processing and a richer programming model. Lingua franca of modern data engineering.

---

## 4.1 — Limits of MapReduce

Why we needed something better than Hadoop MapReduce:

- **Suboptimal file processing** — heavy disk read/write between stages, sequential Map/Reduce.
- **No real-time computation**.
- Cannot **iteratively** apply treatments on the same data (each iteration re-reads from disk).
- **Hard to use** (low-level API, painful debugging on failure).

### Alternatives to Hadoop MapReduce

```
                    ┌────────────────┐
                    │  APACHE SPARK  │
                    └────────────────┘
                  ↗  ↑   ↑   ↑   ↑   ↖
              Kafka HDFS Cassandra ObjStores
              Postgres Elastic DataWarehouses
              ... and many more!

     ┌───────────────┐
     │   FLINK       │
     └───────────────┘
```

**Spark** is the dominant alternative. **Flink** competes (especially for true streaming).

---

## 4.2 — Spark introduction

A Big Data framework written in **Scala**, performing **distributed in-memory computation** on many machines over very large data volumes.

### Key properties

- Inherits the advantages of Hadoop and **fixes most of its limits**.
- Characterised by **in-memory** data processing.
- Only **one read phase** at HDFS input, **one write phase** at the end of the operation.
- Manipulates **2 main object types**: **RDDs** and **DataFrames** (cached in memory).
- Connects to many data sources; natively compatible with **Java, R, Python, Scala**.
- Supports **batch, mini-batch, and streaming**.

### Spark ecosystem

```
┌──────────┬──────────┬──────────┬──────────┬──────────┐
│ Spark    │ Spark    │ MLlib    │ GraphX   │ SparkR   │
│ SQL      │Streaming │ (ML)     │ (graphs) │ (R)      │
└──────────┴──────────┴──────────┴──────────┴──────────┘
┌──────────────────────────────────────────────────────┐
│             Apache Spark Core API                     │
└──────────────────────────────────────────────────────┘
┌────┬─────┬────────┬────────┬────────┐
│ R  │ SQL │ Python │ Scala  │  Java  │
└────┴─────┴────────┴────────┴────────┘
```

> See also: Spark MLlib (machine learning) — *`ml/` domain to be rebuilt*.

---

## 4.3 — RDD (Resilient Distributed Dataset)

Immutable collections of objects performing computations across **multiple cluster nodes**. The **fundamental data structure** of Spark.

### Acronym breakdown

| Letter | Meaning |
|---|---|
| **R**esilient | Fault tolerance via **Data Lineage** with DAGs (see §4.8) |
| **D**istributed | Datasets live across multiple cluster nodes |
| **D**ataset | Records of the data we're working with |

### 8 RDD characteristics

| Feature | Description |
|---|---|
| **In-Memory Computation** | RDDs stored in RAM for faster processing |
| **Lazy Evaluation** | Operations execute only when necessary (loaded on demand) |
| **Fault Tolerance** | Recovers from failures via lineage |
| **Immutability** | Once written to memory, cannot be modified |
| **Partitioning** | Data distributed across multiple nodes for parallelism |
| **Persistence** | Data can persist in memory **or** on disk |
| **Coarse-Grained Operations** | Same operation applied to all elements of a partition |
| **Location-Stickiness** | Spark prefers running computations close to the data |

### Two operation types

| Type | Definition | Examples |
|---|---|---|
| **Transformations** | Functions that **create new RDDs** from an existing RDD or by loading source data. **Lazy** — not executed immediately. | `map()`, `filter()`, `sample()`, `groupByKey()`, `reduceByKey()` |
| **Actions** | Functions that **produce a result** (non-RDD) stored in the **Driver** memory. Trigger execution. Also export results to any storage. | `first()`, `take()`, `count()`, `collect()`, `saveAsTextFile()` |

> QCM: transformations are **lazy**; actions **trigger computation**.

---

## 4.4 — RDD vs DataFrame vs Dataset

Three Spark data abstractions. Choose based on data shape and need.

| Structure | Best for | Schema | Type-safe |
|---|---|---|---|
| **RDD** | Unstructured data, no schema, basic computations (map, filter), groupings | None | No |
| **DataFrame** | Structured data, SQL-like operations, no complex business logic | Yes (untyped) | No |
| **Dataset** | Strongly-typed structured data with compile-time benefits | Yes (typed) | **Yes** |

### Loading commands (Python / PySpark)

```python
# RDD
sc.parallelize([1, 2, 3])         # from a Python collection
sc.textFile("path/to/file.txt")   # from a text file

# DataFrame
spark.read.csv("path/to/file.csv")
```

### Datasets caveat

- Datasets are **only available in Java and Scala** (not in Python or R).
- They combine the benefits of both DataFrames and RDDs.
- **DataFrames are essentially untyped Datasets**.
- There are commands to navigate from one structure to another.

> QCM: in PySpark you only see RDDs and DataFrames — Datasets require Scala/Java.

---

## 4.5 — Spark architecture

```
┌────────────────────┐         ┌──────────────────────┐
│   Driver Program   │         │     Worker Node      │
│ ┌────────────────┐ │         │  ┌──────┐  ┌──────┐  │
│ │ SparkContext   │◄┼────────►│  │ Exec │  │Cache │  │
│ └────────────────┘ │         │  ├──────┤  └──────┘  │
└────────────────────┘         │  │ Task │  ┌──────┐  │
            ▲                  │  ├──────┤  │ Task │  │
            │                  │  └──────┘  └──────┘  │
            ▼                  └──────────────────────┘
   ┌──────────────────┐                  ▲
   │ Cluster Manager  │                  │
   └──────────────────┘                  ▼
            ▲                  ┌──────────────────────┐
            │                  │     Worker Node      │
            └─────────────────►│  ┌──────┐  ┌──────┐  │
                               │  │ Exec │  │Cache │  │
                               │  ├──────┤  └──────┘  │
                               │  │ Task │  ┌──────┐  │
                               │  ├──────┤  │ Task │  │
                               │  └──────┘  └──────┘  │
                               └──────────────────────┘
```

### Components

| Component | Role |
|---|---|
| **Spark Driver** | Central program managing a Spark application. Creates the **SparkContext**. |
| **SparkContext** | Connector linking Driver ↔ Cluster Manager. Coordinates task execution; creates the variables needed to manipulate data (notably RDDs). |
| **Spark Executor** | Program executing the tasks of a Spark job. Handles disk I/O. Runs on a **Worker Node**. |
| **Cluster Manager** | Allocates resources and manages the cluster. |

> QCM: the **Driver creates the SparkContext**; the **Cluster Manager** allocates resources; **Executors** run tasks on Worker Nodes.

---

## 4.6 — Cluster Manager types

| Type | When to use |
|---|---|
| **Standalone Cluster** | Native Spark cluster manager. Small to medium clusters. Ideal for **development/testing**. Contains basic cluster-management functions. |
| **YARN** | Hadoop's resource manager. **Great scalability** vs Standalone. **Complex configuration** to interface with Spark. |
| **Mesos** | Same advantages as YARN **except** strong Hadoop integration. Recommended when you want **strong isolation** between clusters (security). |
| **Kubernetes** | Container orchestrator. Ideal when applications are **deployed in containers**. |

> QCM: Standalone = dev/test; YARN = production with Hadoop; Kubernetes = container-based.

---

## 4.7 — Internal mechanics: Jobs, Stages, Tasks

### The hierarchy

```
Spark Application                  ← Spark Context (Spark Session Object)
       │
       ├── Job                     ← triggered by an RDD Action (collect, saveAsTextFile)
       │    │
       │    ├── Stage              ← split by Wide transformations (reduceByKey, sort)
       │    │    └── Task          ← one per partition (Narrow transformations)
       │    └── Stage
       │         └── Task
       └── Job
            └── Stage
                 └── Task
```

### Definitions

| Concept | Definition |
|---|---|
| **Job** | A set of transformations applied to data. **Created each time an action is called on an RDD**. A Spark program may have many actions → many jobs. A job subdivides into multiple Stages. |
| **Stage** | A grouping of operations (transformations + actions) based on their nature and dependencies. **Each transformation requiring a shuffle creates a new Stage.** A Stage is composed of multiple Tasks. |
| **Task** | The **smallest unit of work** in Spark. Executes on a single machine (executor). Practically, encapsulated transformation operations processed in executors **on a partition**. |

### Wide vs Narrow transformations

```
NARROW (no shuffle)              WIDE (shuffle required)

   ┌──┐    ┌──┐                  ┌──┐ ─────► ┌──┐
   │  │ ─► │  │                  │  │ ─┐     │  │
   └──┘    └──┘                  └──┘  ╲ ──► └──┘
                                       ╲
   ┌──┐    ┌──┐                  ┌──┐ ──╲──► ┌──┐
   │  │ ─► │  │                  │  │ ──╳───►│  │
   └──┘    └──┘                  └──┘ ──╱──► └──┘
                                       ╱
   ┌──┐    ┌──┐                  ┌──┐ ─╱      ┌──┐
   │  │ ─► │  │                  │  │ ───────►│  │
   └──┘    └──┘                  └──┘         └──┘
```

| Type | Network shuffle? | Examples | Stage impact |
|---|---|---|---|
| **Narrow** | No | `map()`, `filter()`, `union()` | Same Stage |
| **Wide** | **Yes** | `groupByKey()`, `reduceByKey()`, `sort()`, `join()` | **New Stage** |

### Shuffling

Operation that **moves data from one machine to another**, creating partitions that share the same key. Triggered by Wide transformations. **Network-heavy** — main optimization target.

### Code example: stage boundaries

```python
rdd  = sc.textFile("fichier.txt")          # Read           → Stage 1
rdd2 = rdd.map(lambda x: x.split())        # Narrow         → still Stage 1
rdd3 = rdd2.groupByKey()                   # Wide (shuffle) → new Stage 2
rdd3.collect()                             # Action → triggers everything
```

### Practical recap

```
                   Spark Application
                   ├── Job (per action)
                   │    ├── Stage (boundaries = wide transformations)
                   │    │    └── Task (one per partition)
                   │    │         ↑
                   │    │         executed in parallel by Executors
                   │    └── ...
                   └── ...
```

Executors run on **Worker Nodes**, orchestrated by a **Cluster Manager**.

---

## 4.8 — DAG (Directed Acyclic Graph)

A **fundamental concept** of Spark's execution engine. Represents and optimises the operation flow of a data-processing job.

### What it does

- Determines **dependencies** between operations.
- Executes steps in the **most efficient order**.
- **Sequences of transformations ending in an action**.

### Code → DAG construction

```python
rdd  = sc.textFile("fichier.txt")
rdd2 = rdd.filter(lambda x: "Spark" in x)
rdd3 = rdd2.map(lambda x: x.upper())
rdd3.collect()
```

When Spark runs the application, it **systematically builds an associated DAG in the background** before encountering an action. The action triggers the operations.

### Goals

- Know **in what order** to execute operations.
- **Optimize** execution.
- Manage **fault tolerance** (replay steps if a machine dies).

### Daily-life analogy

> *What's your routine for going to work?* → That's a DAG. Wake up → shower → coffee → commute → arrive. Each step depends on the previous; the order matters; you can't have an action without its prerequisites.

---

## 4.9 — Spark SQL

The Spark component that lets you **manipulate data in tabular form using SQL**.

### Advantages

- **Easy to use**: SQL is widely accessible.
- **Optimization**: Spark contains a component (Catalyst optimizer) that reduces resource usage and speeds query execution.
- **Interoperability**: manipulate data in many formats.
- **Integration**: compatible with **Java, R, Scala, Python**.

### Manipulating DataFrames

To manipulate DataFrames you instantiate a **SparkSession** (analogous to creating a SparkContext for RDDs). The major difference is the **type of data manipulated**.

```python
from pyspark.sql import SparkSession

spark = SparkSession.builder.appName("DataFrameExample").getOrCreate()
```

Or with Hive support:

```python
warehouse_location = abspath('spark-warehouse')

spark = (SparkSession.builder
    .appName("Python Spark SQL Hive integration example")
    .config("spark.sql.warehouse.dir", warehouse_location)
    .enableHiveSupport()
    .getOrCreate())
```

### Creating views and tables

When data is loaded in a DataFrame, you can create **views** (datamart approach) inside Spark. These views are **Hive tables** — Spark contains a minimalistic Hive component (with minimal dependencies) for SQL-style data manipulation (when Hive is enabled).

Two approaches:

| Approach | What it does |
|---|---|
| `createOrReplaceTempView` | Creates **temporary views** that disappear when the session closes. |
| `saveAsTable` | Saves tables to **disk**. |

```python
df.createOrReplaceTempView("employes")

df.write.format("parquet").mode("append").saveAsTable("my_table")
```

### Full example

```scala
import org.apache.spark.sql.SparkSession

// Create a Spark Session
val spark = SparkSession.builder
  .appName("Spark Job Stage Task Example")
  .getOrCreate()

val data = spark.read.option("header", "true").csv("path/to/your/file.csv")

val newData = data.withColumn("new_column", data("existing_column") * 2)

val result = newData.count()

println(result)

spark.stop()
```

---

## 4.10 — Data persistence

Sometimes it's relevant to **store intermediate results** to optimize Spark application performance.

### Two ways

| Method | Behavior |
|---|---|
| `cache()` | Stores data **in memory**. |
| `persist()` | Stores data on disk **and/or** in memory based on the level passed. |

### Storage levels for `persist()`

| Level | Description |
|---|---|
| **MEMORY_ONLY** | Memory only. |
| **MEMORY_ONLY_SER** | Memory only, **serialized** (smaller footprint). |
| **MEMORY_AND_DISK** | Memory **and** disk. |
| **MEMORY_AND_DISK_SER** | Memory **and** disk, serialized. |

> QCM: `cache()` ≡ `persist(MEMORY_ONLY)`. Use `persist()` for fine control.

---

## 4.11 — Spark Submit

How to deploy a Spark application.

### Two deployment modes

| Mode | Driver location | Use case |
|---|---|---|
| **Client** | Driver stays **local** | Useful for testing/debugging |
| **Cluster** | Driver runs **inside the YARN cluster** | More **robust**, ideal for **production** |

### Syntax

```bash
spark-submit \
  --master yarn \
  --deploy-mode cluster \
  --executor-memory 2G \
  --num-executors 2 \
  --executor-cores 2 \
  fichier.py
```

### Classic parameters

| Parameter | Meaning |
|---|---|
| `--master yarn / spark / mesos / kubernetes` | Deployment target |
| `--deploy-mode cluster / client` | Where the Driver runs |
| `--executor-memory` | Memory per executor |
| `--num-executors` | Total number of executors |
| `--executor-cores` | Cores per executor |

### Sizing rules of thumb

```
4 Go / 8 machines  ⇒ 500 Mb / machine

5 HDFS blocks per file for 500 Mb (HDFS default block 128 Mb)

1 Go ⇒ 1000 Mb

executor memory   ≈ block size × 1.25  (25 % overhead)
total memory cap  = cluster size
```

> QCM: in **production**, use `--deploy-mode cluster` (Driver inside the cluster, more robust).

---

## 4.12 — Module summary

| Concept | Essence |
|---|---|
| **Why Spark** | In-memory, fast, iterative, easy API; fixes MR limits |
| **RDD** | Resilient Distributed Dataset — immutable, lazy, fault-tolerant |
| **Transformations vs Actions** | Lazy vs eager; map/filter vs collect/count |
| **Narrow vs Wide** | No shuffle vs shuffle (creates new Stage) |
| **DataFrame** | Structured, SQL-like, untyped |
| **Dataset** | Typed structured (Java/Scala only) |
| **Architecture** | Driver + SparkContext ↔ Cluster Manager ↔ Executors on Worker Nodes |
| **Cluster Managers** | Standalone (dev), YARN (Hadoop), Mesos (isolation), Kubernetes (containers) |
| **Job / Stage / Task** | Action / Wide-bound / Per-partition |
| **DAG** | Sequence of transformations ending in an action; built lazily |
| **Spark SQL** | Tabular SQL on DataFrames; views (temp) + saveAsTable (persistent) |
| **cache vs persist** | Memory only vs memory + disk levels |
| **spark-submit** | client (debug) vs cluster (prod); --master, --num-executors, --executor-memory |

> See also:
> - YARN component → [`03-hadoop.md` §3.4](./03-hadoop.md#34--yarn-yet-another-resource-negotiator)
> - HDFS underneath Spark → [`03-hadoop.md` §3.2](./03-hadoop.md#32--hdfs-hadoop-distributed-file-system)
> - Spark MLlib *(deferred — `ml/` domain to be rebuilt)*
