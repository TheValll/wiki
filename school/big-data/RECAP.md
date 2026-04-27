# RECAP — Big Data Framework

Single-glance table covering all course concepts. Optimized for QCM revision.

---

## Module 1 — Concepts clés

| Concept | What / for what | Memorise |
|---|---|---|
| **5V** | Defining traits of Big Data | Volume, Variety, Velocity, **Veracity, Value** (added later) |
| **Structured data** | Schema-defined, predictable | RDBMS, **SQL**; PostgreSQL, Oracle |
| **Unstructured data** | Schema-free, raw | **NoSQL**; Cassandra, MongoDB, Neo4j |
| **Parquet** | File format | **Column-oriented**, language-agnostic, compressed, analytical |
| **Datawarehouse** | Storage for structured data | Reporting + analysis; contains **Datamarts** |
| **Datalake** | Multi-source, structured + unstructured | Format of choice = **Parquet** |
| **Lakehouse** | Datawarehouse + Datalake combined | **Médaillon: Bronze → Silver → Gold** (quality grows) |
| **DFS** | Distributed file system | HDFS, GFS, MapR; speed, scalability, fault tolerance |
| **Cloud** | Networked compute + storage | Flexibility, scalability, security, cost optimization, Green IT |
| **IaaS** | Infrastructure as a Service | Hardware (Amazon EC2) |
| **PaaS** | Platform as a Service | IaaS + runtime/OS (Heroku) |
| **SaaS** | Software as a Service | Turnkey app (Shopify) |
| **Public cloud** | Shared resources | Hotel analogy |
| **Private cloud** | Exclusive to one org | Internal or hosted |
| **VM** | Full guest OS | **Hypervisor** at the base |
| **Container** | Shared host OS | **Container Engine**; lighter, faster |

---

## Module 2 — Data jobs

| Role | Mission | Core actions | Salary (Paris) |
|---|---|---|---|
| **Data Engineer** | Collect/store/distribute at scale | Pipelines, batch + streaming, datalakes/DWH/datamarts | **47 k€** |
| **Data Analyst** | Make data speak | Extract, clean, standardize, report, recommend | **45 k€** |
| **Data Scientist** | Predict + automate | Preprocess, build ML/stat models | **49 k€** |

> Data Scientist triangle: **Math + Programming + Business knowledge**.

---

## Module 3 — Hadoop

| Concept | Essence |
|---|---|
| **Hadoop** | Big Data framework for distributed compute, native Java |
| **3 cores** | **HDFS, MapReduce, YARN** |
| **HDFS** | Distributed FS |
| **NameNode** | Stores **metadata**; **unique** |
| **DataNode** | Stores blocks; **one per machine**; talks to NameNode |
| **MapReduce** | Programming model: **Map → Shuffle → Reduce** |
| **Map** | Transform input → key/value blocks |
| **Reduce** | Group by key, combine, write to HDFS |
| **YARN** | Hadoop resource manager |
| **Resource Manager** | Receives jobs, allocates resources |
| **Scheduler** | Sub-component of RM; schedules jobs |
| **ApplicationManager** | Sub-component of RM; receives jobs, negotiates containers |
| **ApplicationMaster** | **One per app, lives in a node**; supervises distributed execution |
| **NodeManager** | One per node; reports RAM/CPU/health to RM |
| **Container** | Resource envelope (RAM/CPU/disk) where job runs |
| **Hive** | SQL on Hadoop via **HiveQL → MapReduce jobs** |
| **Pig** | Scripting via **Pig Latin** → MapReduce |
| **Oozie** | Big Data workflow scheduler |
| **Mahout** | Distributed ML on MapReduce |
| **HBase** | NoSQL **column-oriented** DB on HDFS |
| **Ambari** | Cluster management |
| **Sqoop / Flume** | Data ingestion (RDBMS / logs) |
| **Zookeeper** | Coordination |

> YARN flow (8 steps): submit → RM creates AM container → AM registers → AM negotiates → AM asks NodeManager → containers run → client polls metrics → AM unregisters.

---

## Module 4 — Spark

| Concept | Essence |
|---|---|
| **Spark** | In-memory distributed Big Data framework, written in **Scala** |
| **vs Hadoop MR** | In-memory, lazy, iterative, easy API, batch + streaming |
| **RDD** | **Resilient Distributed Dataset**; immutable, lazy, fault-tolerant |
| **Resilient** | Fault tolerance via **Data Lineage / DAG** |
| **Distributed** | Across multiple cluster nodes |
| **Dataset** | Records being processed |
| **Transformation** | Creates a new RDD; **lazy** (map, filter, sample, groupByKey) |
| **Action** | Triggers execution; returns non-RDD to **Driver** (collect, count, take, first) |
| **Narrow transformation** | No shuffle (map, filter); same Stage |
| **Wide transformation** | **Shuffle** (groupByKey, reduceByKey, join, sort); **new Stage** |
| **Shuffle** | Move data across machines, group by key |
| **DataFrame** | Structured, SQL-like, **untyped** |
| **Dataset** | Strongly-typed; **Java/Scala only** |
| **Spark Driver** | Central program; creates SparkContext |
| **SparkContext** | Driver ↔ Cluster Manager link; creates RDD variables |
| **Spark Executor** | Runs tasks; on Worker Nodes |
| **Cluster Manager** | Allocates resources; Standalone / YARN / Mesos / Kubernetes |
| **Standalone** | Native Spark, dev/test |
| **YARN** | Production with Hadoop |
| **Mesos** | Strong isolation between clusters |
| **Kubernetes** | Container orchestration |
| **Job** | Created by an **action** on an RDD |
| **Stage** | Job slice; **boundaries = wide transformations / shuffles** |
| **Task** | **Smallest unit**; one per partition; runs on one executor |
| **DAG** | **Directed Acyclic Graph**; built lazily before action; used for ordering, optimization, fault tolerance |
| **Spark SQL** | SQL on DataFrames; **Catalyst** optimizer |
| **createOrReplaceTempView** | Temporary view, dies with session |
| **saveAsTable** | Persistent table on disk |
| **cache()** | Memory-only persistence |
| **persist(level)** | MEMORY_ONLY / MEMORY_ONLY_SER / MEMORY_AND_DISK / MEMORY_AND_DISK_SER |
| **spark-submit** | Deploy a Spark app |
| **deploy-mode client** | Driver local, debug |
| **deploy-mode cluster** | Driver in cluster, **production** |
| **--master** | yarn / spark / mesos / kubernetes |
| **--executor-memory** | Memory per executor |
| **--num-executors** | Total executors |
| **--executor-cores** | Cores per executor |
| **executor memory rule** | Block size × **1.25** (25 % overhead) |

---

## Cross-module quick traps

- **Hive translates HiveQL to MapReduce** — it doesn't replace MR, it builds on it.
- **NameNode is unique**, **DataNodes are many** — beware reverse phrasing.
- **ApplicationManager** ≠ **ApplicationMaster** (RM sub-component vs per-app supervisor).
- **Transformations are lazy**, **actions trigger execution**.
- **Wide transformations create new Stages**, narrow do not.
- **Datasets exist only in Java/Scala**, not Python.
- **Production deploy mode = cluster**, not client.
- **Bronze → Silver → Gold**: quality grows left to right (not the reverse).
- **5V**: Volume, Variety, Velocity are the original 3; **Veracity + Value** were added later.
- **Containers share the host OS** (via Container Engine); **VMs each have their own Guest OS** (via Hypervisor).
