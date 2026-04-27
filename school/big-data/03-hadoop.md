# Part 3 — Hadoop Ecosystem

Hadoop is the **first-generation Big Data framework**: distributed storage (HDFS) + distributed computing (MapReduce) + resource management (YARN), plus a rich ecosystem (Hive, Pig, HBase, etc.).

---

## 3.1 — What Hadoop is

A Big Data framework that performs **distributed computation** on **massive data volumes** across **many machines**.

### Properties

- **Ecosystem**: a set of components addressing ML, analytics, automation.
- **Horizontal + vertical scalability** of treatments.
- Relies on **parallel processing**.
- Offers **high availability** + **fault tolerance**.
- Connects to many data sources.
- **Native compatibility with Java**; supports R, Python, Scala via libraries.

### The ecosystem at a glance

```
     ┌───────┬──────────┬───────┬──────┬───────────┬──────┬────────┬──────┐
     │ Oozie │ HCatalog │  Pig  │ Hive │  Mahout   │ Drill │  AVRO  │ HBase│
     │workflow│ schema  │script │ SQL  │ ML        │analyt│ (JSON) │NoSQL │
     └───────┴──────────┴───────┴──────┴───────────┴──────┴────────┴──────┘
     ┌───────┬───────────────┬──────────────────────────────────────────────┐
     │ Sqoop │   Zookeeper   │    Apache Ambari                            │
     │ Flume │  (coordination)│  (mgmt + monitoring)                       │
     └───────┴───────────────┴──────────────────────────────────────────────┘
     ┌───────────────────────────────────────────────────────────────────────┐
     │                       MapReduce (Data Processing)                      │
     ├───────────────────────────────────────────────────────────────────────┤
     │                       YARN  (Cluster Resource Mgmt)                    │
     ├───────────────────────────────────────────────────────────────────────┤
     │                       HDFS  (Hadoop Distributed File System)            │
     └───────────────────────────────────────────────────────────────────────┘
```

The **3 cores** of Hadoop: **HDFS, MapReduce, YARN**.

---

## 3.2 — HDFS (Hadoop Distributed File System)

The storage layer. Allows Hadoop to **read input data** and **write output data** across many machines.

### Two daemons

| Daemon | Role | Cardinality |
|---|---|---|
| **NameNode** | Stores the **metadata** about block distribution. For a file `users` in directory `raw` (split into 20 blocks across 4 machines), the NameNode knows where each block lives. | **Unique** |
| **DataNode** | Stores the actual **data blocks**. Constantly communicates with the NameNode to receive data and report on storage status. | **One per machine** |

### Architecture

```
                                     Metadata: /home/foo/data, replicas=3, ...
                                     │
                              ┌──────┴───────┐
                              │   NameNode   │
                              └──────────────┘
                              ▲ Metadata ops
                              │
        Client ─Read───────►  │  ◄────Block ops─── DataNodes
                              │
   ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐
   │ DataNode 1 │  │ DataNode 2 │  │ DataNode 3 │  │ DataNode 4 │  │ DataNode 5 │
   │ ▆▆ ▆       │  │ ▆▆         │  │ ▆▆ ──repl──│► ▆▆          │  │  ▆▆▆       │
   └────────────┘  └────────────┘  └────────────┘  └────────────┘  └────────────┘
   ◄──────────── Rack 1 ──────────────►       ◄────────── Rack 2 ──────────────►
                                                   ▲
                                          Client ──Write
```

- Files are split into **blocks** (default 128 MB or 256 MB).
- Blocks are **replicated** (typically 3×) across DataNodes for fault tolerance.
- **Client read** → NameNode for metadata, DataNodes for actual data.
- **Client write** → DataNodes (with replication chained between them).

> QCM: the **NameNode is unique** (single point of metadata). The **DataNodes are many**. They are also called **daemons**.

---

## 3.3 — MapReduce

A **programming model** that runs on top of HDFS for **distributed computation** in the Hadoop environment.

### The motivating analogy: counting people in a hotel

| Approach | Time |
|---|---|
| **Classical**: 1 person counts every floor (10 min/floor × 100 floors) | **1000 min** |
| **MapReduce**: 100 people, 1 per floor, counts in parallel + share/aggregate | **~20 min** (10 min compute + collect/share overhead) |

This is the core insight: **distribute the work, then aggregate**.

### Real-world block layout

```
    ┌─────────┐         ┌─────────┐         ┌─────────┐         ┌─────────┐
    │ Server  │         │ Server  │         │ Server  │         │ Server  │
    │ blocks  │         │ blocks  │         │ blocks  │         │ blocks  │
    │ 256 MB  │   ...   │ 256 MB  │   ...   │ 256 MB  │   ...   │ 256 MB  │
    └─────────┘         └─────────┘         └─────────┘         └─────────┘

    Cluster of 20 000 servers with these blocks  →  5 To of data processed
```

### The 3 operations

| Operation | What it does |
|---|---|
| **MAP** | Transforms input data into multiple **key/value blocks**. For each block, applies the mapping function and emits a new key/value system. |
| **SHUFFLE** | Implicit step: redistributes key/value pairs across blocks, **grouping by key**. |
| **REDUCE** | After mapping, regroups by key. For each key, **combines** the values and writes the result to HDFS. |

### Canonical example: word count

```
INPUT          SPLITTING       MAPPING            SHUFFLING       REDUCING       FINAL
                                                                                    
Deer Bear      Deer Bear       Deer, 1            Bear, 1         Bear, 2          
River          River           Bear, 1            Bear, 1                          Bear, 2
                               River, 1                                            Car,  3
Car Car        Car Car         Car, 1             Car, 1          Car, 3           Deer, 2
River          River           Car, 1             Car, 1                           River,2
                               River, 1           Car, 1
Deer Car       Deer Car                                                            
Bear           Bear            Deer, 1            Deer, 1         Deer, 2          
                               Car, 1             Deer, 1
                               Bear, 1                                              
                                                  River, 1        River, 2
                                                  River, 1
```

> The **shuffle** is implicit but heavy: it moves data across the network to group keys. It's the most expensive part of MapReduce.

---

## 3.4 — YARN (Yet Another Resource Negotiator)

The **resource manager** of Hadoop. Provides a stable foundation for **operational management and resource sharing** of the system.

### Role

- **Allocates resources** for each treatment.
- **Monitors treatments** running on different machines.
- **Coordinates** application execution in a distributed way.

### 6 components

| Component | Role |
|---|---|
| **Resource Manager (RM)** | Receives job execution instructions; allocates resources; schedules execution |
| **Scheduler** | Sub-component of RM. Schedules job execution based on available resources + application demand |
| **ApplicationManager** | Sub-component of RM. Receives jobs; negotiates container creation in a node with the RM |
| **ApplicationMaster** | Lives **inside each node**, associated with a **single application**. Supervises distributed job execution. Produces application logs. |
| **NodeManager** | Monitors execution on each node (one per node). Reports to the RM (RAM, CPU, health status). |
| **Container** | Controlled by NodeManagers. Holds the physical resources (RAM, CPU, disk) needed to execute jobs on a node. |

> **Job** = a program/treatment to execute.
> **Node** = a networked machine in the cluster.
> **Container** = the resource envelope where work runs on a node.

### YARN process (8 steps)

```
                                      ┌─────────────────────┐
                                      │ Application Manager │
                                      └─────────────────────┘
                                          ▲   2,3,4   │
                                          │           ▼ 5
   ┌────────┐    1     ┌──────────────────────┐    8   ┌──────────────┐
   │ Client │────────► │  Resource Manager    │ ◄────  │ Node Manager │
   │        │ ◄──── 7  └──────────────────────┘         └──────────────┘
   │        │                                            ▲
   │        │ ─────────────── 6 ─────────────────────────┘
   └────────┘
```

| Step | Description |
|---|---|
| 1 | A job is submitted to Hadoop. YARN activates and sends the request to the Resource Manager. |
| 2 | The RM creates a container to start the **Application Manager**. |
| 3 | The Application Manager registers with the RM. |
| 4 | The Application Manager negotiates the resources needed (containers) for the job. |
| 5 | The Application Manager asks the **NodeManager** to create the containers in which the work will run. |
| 6 | The containers execute the job in a distributed way. |
| 7 | The client can request job-tracking metrics from the RM. |
| 8 | The Application Manager unregisters from the RM; memory is freed for other jobs. |

> QCM trap: distinguish **ApplicationManager** (in RM, manages multiple apps) from **ApplicationMaster** (one per running app, lives in a node). They sound alike but do different things.

---

## 3.5 — Hive (Hadoop with SQL)

### The challenge: Facebook scale

> 950M users, 500 To/day, 70k queries/day, 300M photos/day. Traditional RDBMS broke. Hadoop MapReduce works but is **hard to program**, while users **already know SQL**.

### What Hive does

A **data warehouse** for analytics on top of Hadoop, using **HiveQL** (SQL-like).

### Characteristics

- Organizes data into **tables in HDFS directories**. Tables are similar to a relational database.
- Data points have a unique number; each table corresponds to one HDFS directory.
- Tables can be **partitioned** and **bucketed**.
- **Schema flexibility and evolution**.
- Easy to plug **custom mapper / reducer** code.
- **JDBC/ODBC drivers** available.
- HIVE tables can be **defined directly on HDFS**.
- **Extensible**: types, formats, functions, scripts.
- Translates **HiveQL queries into MapReduce jobs** (or Spark, or other distributed executors).

> QCM: Hive **does not replace MapReduce** — it **generates MapReduce jobs from SQL**.

---

## 3.6 — Other Hadoop components

| Component | What it does |
|---|---|
| **Ambari** | Simplified management, creation, and audit of Hadoop clusters. |
| **Pig** | Platform that runs MapReduce programs using a simplified scripting language called **Pig Latin**. |
| **Oozie** | Sets up Big Data **workflows** — schedule and plan jobs. |
| **Mahout** | Trains **Machine Learning models in a distributed way** via MapReduce. |
| **HBase** | **NoSQL column-oriented** database built on HDFS. |
| **Sqoop** | Data collection from RDBMS into Hadoop. |
| **Flume** | Data collection (log streams). |
| **Zookeeper** | Coordination service. |
| **HCatalog** | Table & schema management. |
| **Drill** | Interactive analysis. |
| **AVRO** | Data serialization (JSON-based). |
| **Thrift** | Cross-language service framework. |

---

## 3.7 — Module summary

| Concept | Essence |
|---|---|
| **HDFS** | Distributed FS with **NameNode (unique, metadata)** + **DataNodes (one per machine, blocks)** |
| **MapReduce** | Map → Shuffle → Reduce, key/value programming model |
| **YARN** | Resource manager: RM, Scheduler, ApplicationManager, ApplicationMaster, NodeManager, Container |
| **Hive** | SQL on HDFS via HiveQL → translates to MapReduce |
| **Pig** | Scripting (Pig Latin) → MapReduce |
| **HBase** | NoSQL column DB on HDFS |
| **Oozie** | Workflow scheduler |
| **Mahout** | Distributed ML on MapReduce |
| **Ambari** | Cluster management |

> See also: HDFS = a **DFS** ([§1.5.4](./01-concepts.md#154--distributed-file-system-dfs)).

**Next:** [Part 4 — Spark Ecosystem](./04-spark.md)
