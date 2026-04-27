# Part 2 — Data Jobs

Three professional roles around data, with overlapping but distinct missions. Knowing which one does what is a frequent QCM target.

| Role | Mission in one line | Salary (Paris, target) |
|---|---|---|
| **Data Engineer** | Collect, store, distribute data at scale | 47 000 € |
| **Data Analyst** | Make the data speak | 45 000 € |
| **Data Scientist** | Build models on data to predict / automate | 49 000 € |

---

## 2.1 — Data Engineer

> A data professional whose goal is to ensure the **collection, storage, and distribution of data at very large scale** in the company.

### Daily actions

- **Collect** data from many sources and structures.
- **Process and distribute** data in batch or streaming.
- Build **Big Data pipelines** to feed datalakes, datawarehouses, and datamarts.
- **Schedule** Big Data pipelines.

### Tools (reference)

| Layer | Tools |
|---|---|
| **Storage** | BigQuery, Snowflake, Redshift, Clickhouse, Postgres |
| **Orchestration & Transformation** | Airflow, DBT, Prefect, Dagster, Databricks |
| **Ingestion (In/Out)** | Airbyte, Fivetran, Python, Hightouch, Stitch |
| **Activation** | Power BI, Tableau, Metabase, Looker, Whaly |
| **Governance** | Monte Carlo, Sifflet, Atlan, Castor, Stash |
| **Data Platforms** | Databricks, Azure Synapse Analytics, Snowflake |
| **Streaming** | Kafka, Spark Streaming, Flink |
| **Languages** | SQL, Python, PySpark |

---

## 2.2 — Data Analyst

> A data professional whose goal is to **make the data speak**.

### Daily actions

- **Extract and clean** data.
- **Standardize** data.
- Produce **reports**.
- Make **business recommendations**.

### Tools (reference)

Tableau, Apache Spark, SQL, Microsoft Power BI, Datapine, Airtable, ClicData, Thoughtspot, SAS, Qlik.

> Strong overlap with Data Engineer on SQL and BI tools — but the Analyst stops at insight, while the Engineer builds the pipelines that feed it.

---

## 2.3 — Data Scientist

> A data professional who relies on **data + math + algorithms** to **automate results, predict outcomes, and find correlations** between data points.

### Skill triangle

```
        Mathematics & Statistics
                  ╲
                   ╲
                    DATA SCIENTIST  ── Business knowledge
                   ╱
                  ╱
        Programming & Algorithms
```

The Data Scientist sits at the intersection of three domains.

### Daily actions

- **Extract and clean** data.
- **Standardize and normalize** data.
- Define **preprocessing pipelines**.
- Build **statistical and Machine Learning models**.

### Tools (reference)

| Category | Tools |
|---|---|
| **Big Data handling** | SQL, Hadoop, Spark, MySQL, neo4j, Excel |
| **Data mining & transformation** | Pandas, WEKA, Scrapy |
| **Model deployment** | TensorFlow.js, MLflow |
| **Visualization** | Tableau, ggplot2, D3 |
| **Machine Learning** | Algorithms.io, SAS, MATLAB, Python, DataRobot, bigml, R |

---

## 2.4 — Comparison table (QCM-friendly)

| Question | Data Engineer | Data Analyst | Data Scientist |
|---|---|---|---|
| Primary output | Pipelines, datasets | Reports, recommendations | Predictive models |
| Math heavy? | No | Light (stats) | **Yes** |
| Codes ML models? | No | No | **Yes** |
| Builds pipelines? | **Yes** | No | Yes (preprocessing) |
| Owns dataviz? | No | **Yes** | Sometimes |
| Salary anchor (Paris) | 47k€ | 45k€ | 49k€ |

> The line is fuzzy in practice — a Lead Data Scientist may build pipelines, a Senior Data Engineer may deploy ML models. But for QCM purposes, stick to the canonical missions above.

**Next:** [Part 3 — Hadoop Ecosystem](./03-hadoop.md)
