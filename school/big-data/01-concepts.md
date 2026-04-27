# Part 1 — Big Data: Key Concepts

Foundations: why Big Data exists, how data is described (5V), how it's structured, stored, and where it lives (file format, storage system, cloud, container).

---

## 1.1 — Context: why Big Data

The shift to "Big Data" came from a convergence of factors:

- **IT progress** (compute, storage, networking).
- **Mass data production** by users.
- **Smartphones + IoT** generating continuous streams.
- **Social networks** adopted globally.
- **Structured + unstructured data** transferred at scale.
- **Internet accessibility** improved everywhere.

> *"La donnée est l'or noir du 21e siècle"* — recurring quote of the course.

### The 4 questions Big Data must answer

1. How to **collect** the data?
2. How to **store** the data?
3. How to **distribute** the data?
4. How to **exploit** the data to create value?

### Storage units

| Abbr | Prefix | Decimal size | Binary approx |
|---|---|---|---|
| K | kilo | 10³ | 2¹⁰ |
| M | mega | 10⁶ | 2²⁰ |
| G | giga | 10⁹ | 2³⁰ |
| T | tera | 10¹² | 2⁴⁰ |
| P | **peta** | 10¹⁵ | 2⁵⁰ |
| E | exa | 10¹⁸ | 2⁶⁰ |
| Z | zetta | 10²¹ | 2⁷⁰ |
| Y | yotta | 10²⁴ | 2⁸⁰ |

**For scale (2014):** Google processed 100 Po/day and stored 15 000 Po. Facebook processed 600 To/day. Use these figures as anchor points.

### What Big Data frameworks are used for

- Producing **reports at scale**.
- **Predicting** outcomes and building strategies on those predictions.
- **Automating** treatments → operational efficiency.

---

## 1.2 — The 5 V

The defining characteristics of Big Data. Originally 3V (Volume, Variety, Velocity) → now 5V.

| V | Definition | Example |
|---|---|---|
| **Volume** | Storage must ingest/keep Gigabytes → Petabytes of data | Google: 15 000 Po stored |
| **Variety** | Data is both **structured AND unstructured** | SQL tables + emails + videos + logs |
| **Velocity** | Data is collected in **batch, mini-batch, streaming, or event-trigger** modes | Real-time stock ticks vs nightly batches |
| **Veracity** | The **truthfulness / quality** of the data | Are sensors accurate? Are sources reliable? |
| **Value** | The **business value** extracted from the data | Recommendation engine ROI |

> Mnemonic: **Volume / Variety / Velocity** are the original 3V. **Veracity / Value** were added later.

---

## 1.3 — Data structures

Two families. The boundary determines which DB technology fits.

### Structured data

- **Formatted** in a well-defined schema.
- Notion of **unique identifier** + unique data model for same-nature records.
- **Predefined memory storage**.
- **Reduced flexibility** when updating the schema.
- Optimized for collection.
- Main characteristic: **relational databases, SQL**.

**Examples:** PostgreSQL, Oracle SQL, any RDBMS.

### Unstructured / semi-structured data

- **Raw data, no predefined schema** even if records share a nature.
- Memory size **not predefined**.
- High **flexibility** on structure.
- **Difficult to process** (no fixed schema to rely on).
- Main characteristic: **non-relational databases, NoSQL**.

**Examples:**
- Cassandra (wide-column)
- MongoDB (document)
- Neo4j (graph)

> QCM trap: emails, conversations, logs = unstructured. Excel-like tables = structured.

---

## 1.4 — Parquet format

A **file format** for storing data **column-wise** (vs row-wise CSV/JSON). Goal: efficient storage + querying.

### Characteristics

| Property | Why it matters |
|---|---|
| **Language-independent** | Works with Python, Java, Scala, R, etc. |
| **Columnar format** | Reads only the columns you query — cheaper I/O |
| **Analytical-friendly** | Optimized for OLAP (aggregations, group by) |
| **High compression efficiency** | Same-type values compress well column by column |

### Advantages

- **Ideal for any Big Data workload**.
- **Optimizes storage space**.
- **Boosts analytics performance**.

### Internal layout

A Parquet file is split into **Row Groups**, each containing all columns for a subset of rows. Within a row group, each column is stored contiguously (a **column chunk**). This layout is what enables column-pruning and per-column compression.

```
Row Group 1   [Product col][Customer col][Country col][Date col][Amount col]
Row Group 2   [Product col][Customer col][Country col][Date col][Amount col]
Row Group 3   [Product col][Customer col][Country col][Date col][Amount col]
```

> QCM: Parquet = **column-oriented**, **language-agnostic**, **compressed**, **analytics-optimized**.

---

## 1.5 — Storage systems

Three architectures, each adapted to a different mix of data type, query style, and cost.

### 1.5.1 — Data Warehouse (DWH)

- **Relational database** with **structured data**, hosted in a Data Center or Cloud.
- Stores data for **reporting + analysis**.
- Ideal for **company-wide datasets** (enterprise models).
- Data is generally **historicized**.
- **Datamarts** = portions of a DWH built for a specific business problem (Sales, Finance, Marketing).

```
SOURCES                  DATA WAREHOUSE              DATAMARTS
(custom apps,        ┌─ Staging layer              ┌─ Sales mart
 logs, files)  ───►  ├─ Reporting layer  ────────► ├─ Finance mart
                     └─ Metadata layer             └─ Marketing mart
```

### 1.5.2 — Data Lake

- Stores **structured + unstructured** data from **many sources**.
- Predilection format: **Parquet**.
- No enforced schema at ingestion.

### 1.5.3 — Lakehouse + medallion architecture

To combine the cheap-storage of a Datalake with the query-power of a Datawarehouse, the **Lakehouse** was born. Multiple zones in one physical layer.

```
                IMPROVE DATA QUALITY ───────────────────►

  Batch ─┐
         ├──► [Bronze]  ──►  [Silver]  ──►  [Gold]  ──► BI / ML
Stream ─┘    Raw           Filtered,        Business-
             Integration   Cleaned,         Level
                           Augmented        Aggregates
             "Landing      Define schema,   Continuously
              zone",       enforce,         updated, clean
              no schema    evolve            data delivered
```

| Layer | Role | Schema | Quality |
|---|---|---|---|
| **Bronze** | Raw ingestion (landing zone) | None | Low |
| **Silver** | Filtered, cleaned, augmented | Defined, evolves | Medium |
| **Gold** | Business-level aggregates | Strict | High |

> QCM: medallion = **Bronze → Silver → Gold**. Quality increases left to right.

### 1.5.4 — Distributed File System (DFS)

- File storage **distributed across servers and/or geographic locations**.
- Data accessed over the **network, securely**.
- Data lives in a private Data Center **or** in the Cloud.

**Key characteristics:**

| Property | Meaning |
|---|---|
| **Speed of access** | Faster when source is geographically closer |
| **Location independence** | No need to know *where* the data physically is |
| **Scalability** | DFS adapts to growing volumes |
| **Fault tolerance** | Redundancy → data is independent of any single cluster/server |
| **Security** | Access managers protect the data |

**Examples:** HDFS (Hadoop DFS), Google File System, MapR File System.

---

## 1.6 — Cloud

> **CNIL definition:** *"Cloud computing refers to the use of memory and compute capacity of computers and servers distributed worldwide and linked by a network."*

### Advantages

| Advantage | Why it matters |
|---|---|
| **Flexibility** | Access resources from anywhere; consumption adapts to needs |
| **Efficiency** | Focus on business problems, not infrastructure |
| **Scalability** | Resources scale up/down with demand |
| **Security** | Built-in security + customer can layer their own |
| **Cost optimization** | Bill based on usage |
| **Performance** | Access to powerful machines for heavy workloads |
| **Green IT** | Reduce carbon footprint by monitoring consumption |

### Disadvantages

| Disadvantage | Why |
|---|---|
| **Security** | Cloud = internet service → exposed to attacks |
| **Outage** | Network issues → resources inaccessible (avoid hard dependency) |
| **Data confidentiality** | Sensitive data may not be entrusted to third parties |
| **Unforeseen costs** | Lack of expertise → unexpected bills |

### Service types: IaaS / PaaS / SaaS

```
         What you manage       What the provider manages
         ─────────────────     ──────────────────────────
On-Site  EVERYTHING            Nothing
IaaS     Apps + Data + Runtime + OS + DB    Virtualization + Servers + Storage + Network
PaaS     Apps + Data                        Runtime + OS + DB + below
SaaS     —                                  Everything (you just use the app)
```

| Type | What's provided | Example | Layer focus |
|---|---|---|---|
| **IaaS** (Infrastructure) | Hardware infrastructure (servers/machines on the network) | **Amazon EC2** | No need to maintain servers/datacenters; provider handles hardware scalability + redundancy |
| **PaaS** (Platform) | IaaS + dedicated software environment for running applications | **Heroku** | No need to maintain logical infra (OS); focus on DevOps + app design + deployment |
| **SaaS** (Software) | Turnkey application, just personalize it | **Shopify** | No application maintenance; focus on marketing + commercial |

### Public vs private cloud

| Type | Definition | Analogy |
|---|---|---|
| **Private cloud** *(CloudFlare def)* | Cloud service offered exclusively to one organization. Two flavors: **internal** (on-prem) or **hosted** (provider). | Owning your own house |
| **Public cloud** | Resources shared by multiple clients | Hotel: 100 rooms, anyone can rent each, billed by quality + duration |

> QCM trap: IaaS ⊂ PaaS ⊂ SaaS in terms of what the provider manages (more managed = more SaaS).

---

## 1.7 — Virtualization

Two ways to abstract hardware to run multiple isolated workloads on one machine.

### Machine virtualization (VMs)

```
┌──────┬──────┬──────┐
│ App1 │ App2 │ App3 │
├──────┼──────┼──────┤
│Bins/ │Bins/ │Bins/ │
│ Lib  │ Lib  │ Lib  │
├──────┼──────┼──────┤
│Guest │Guest │Guest │
│  OS  │  OS  │  OS  │   ← each VM has a full OS
├──────┴──────┴──────┤
│    Hypervisor      │
├────────────────────┤
│  Infrastructure    │
└────────────────────┘
```

Each VM ships its **own guest OS** — heavier but stronger isolation.

### Containers

```
┌──────┬──────┬──────┐
│ App1 │ App2 │ App3 │
├──────┼──────┼──────┤
│Bins/ │Bins/ │Bins/ │
│ Lib  │ Lib  │ Lib  │
├──────┴──────┴──────┤
│  Container Engine  │   ← shared engine (e.g. Docker)
├────────────────────┤
│  Operating System  │   ← shared OS
├────────────────────┤
│  Infrastructure    │
└────────────────────┘
```

Containers **share the host OS** — lightweight, fast to start, less isolated.

| Aspect | VMs | Containers |
|---|---|---|
| OS | One per VM | Shared with host |
| Weight | Heavy (GB) | Light (MB) |
| Boot time | Minutes | Seconds |
| Isolation | Strong | Process-level |
| Examples | VMware, VirtualBox | Docker, Podman |

> QCM: containers run on a **shared OS via a Container Engine**; VMs run on a **Hypervisor with their own Guest OS**.

---

## 1.8 — Module summary

| Concept | One-line essence |
|---|---|
| 5V | Volume, Variety, Velocity, Veracity, Value |
| Structured data | Schema-defined, SQL, RDBMS |
| Unstructured data | Schema-free, NoSQL, Cassandra/MongoDB/Neo4j |
| Parquet | Columnar, language-agnostic, compressed, analytical |
| Datawarehouse | Relational, structured, reporting, datamarts inside |
| Datalake | Multi-source, structured + unstructured, Parquet |
| Lakehouse | DWH + Datalake combined; medallion = Bronze → Silver → Gold |
| DFS | Distributed file system; HDFS, GFS, MapR |
| Cloud | Memory + compute on networked servers; flex/scal/cost |
| IaaS / PaaS / SaaS | Infra / Platform / Software (provider manages more as you go right) |
| Public / Private cloud | Shared / dedicated |
| VM vs Container | Guest OS + Hypervisor / Shared OS + Container Engine |

**Next:** [Part 2 — Data Jobs](./02-data-jobs.md)
