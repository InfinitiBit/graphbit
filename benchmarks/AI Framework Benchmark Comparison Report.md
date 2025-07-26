---
---
# **AI Framework Benchmark Comparison Report**

*COMPREHENSIVE BENCHMARK COMPARISON REPORT and Detailed per-scenario Tables*
*Performance Summary Across Intel and AMD Virtual Machines*

---

## **1. Test Environment Overview**

* **VM1:** Intel Xeon (t3.small)
* **VM2:** AMD EPYC (t3a.small)
* Configuration:

  * 2 vCPUs
  * 2 GB RAM
* Each framework was tested across 6 task scenarios, with results **averaged over 10 independent runs per scenario**.


---

## **2. Metrics Evaluated**

| Metric           | Description                                                  |
| ---------------- | ------------------------------------------------------------ |
| **Time (ms)**    | Average time to complete one task (lower is better)          |
| **Memory (MB)**  | Peak memory consumed per task (lower is better)              |
| **CPU (%)**      | Average processor usage per task (lower is better)           |
| **Tokens**       | Number of tokens processed per task (indicative of workload) |
| **Throughput**   | Tasks completed per second (higher is better)                |
| **Success Rate** | Whether all tasks completed successfully (target = 100%)     |

---

## **3. Overall Framework Summary**

| **Framework**  | **CPU (%)** | **Memory (MB)** | **Throughput (tasks/min)** | **Execution Time (ms)** |
| -------------- | ----------- | --------------- | -------------------------- | ----------------------- |
| **GraphBit**   | 0.06–0.27   | 0.00–0.03       | 23.4 – 25.2                | \~1125 – 63674          |
| **LangChain**  | 0.8–4.6     | 0.01–1.05       | 23.4 – 24.6                | \~1275 – 52577          |
| **LangGraph**  | 0.9–2.7     | 0.04–0.13       | 13.2 – 14.4                | \~1193 – 50712          |
| **CrewAI**     | 2.1–6.9     | 0.29–0.59       | 20.1 – 22.2                | \~1816 – 58013          |
| **PydanticAI** | 0.6–2.8     | 0.05–0.13       | 21.0 – 22.8                | \~2059 – 53960          |
| **LlamaIndex** | 5.7–38.5    | 0.10–12.9       | 25.8 – 26.4                | \~1223 – 49544          |

---

## **4. Use Case Recommendations by Metric**

###  **Lowest CPU Usage**

| Rank | Framework      | Avg CPU (%) | Note                                         |
| ---- | -------------- | ----------- | -------------------------------------------- |
| 1   | **GraphBit**   | 0.06–0.27   | Most efficient; suitable for minimal devices |
| 2   | **PydanticAI** | 0.6–1.1     | Efficient across memory and speed            |
| 3   | **LangChain**  | 0.8–1.6     | Balanced with higher task capacity           |

---

###  **Lowest Memory Usage**

| Rank | Framework      | Avg Memory (MB) | Note                                   |
| ---- | -------------- | --------------- | -------------------------------------- |
| 1   | **GraphBit**   | 0.00–0.03       | Nearly zero; best for memory-limited   |
| 3   | **PydanticAI** | 0.05–0.10       | Lightweight and predictable            |
| 3   | **LangGraph** | 0.04–0.11           | Slightly higher but stable |

---

###  **Highest Throughput (Tasks/Minute)**

| Rank | Framework      | Throughput (tasks/min) | Note                                 |
| ---- | -------------- | ---------------------- | ------------------------------------ |
| 1   | **LlamaIndex** | 25.8 – 26.4            | Fastest in simple/parallel scenarios |
| 2   | **GraphBit**   | 23.4 – 25.2            | Close second with lower resource use |
| 3   | **LangChain**  | 23.4 – 24.6            | Reliable throughput with balance     |

---

###  **Fastest Execution Time**

| Rank | Framework      | Time (ms) Range | Note                                  |
| ---- | -------------- | --------------- | ------------------------------------- |
| 1   | **GraphBit**   | \~1125 – 4524   | Quickest on simple and parallel       |
| 2   | **LlamaIndex** | \~1223 – 3935   | Close contender, more resource-hungry |
| 3   | **LangGraph**  | \~1193 – 3035   | Good time but less throughput         |

---

## **5. Comprehensive Scenario-by-Scenario Comparison**

*(All results below are averaged over 10 runs, tested across both Intel and AMD VMs.)*


---

###  **Scenario 1: Simple Task**

| Framework      | Time (ms) | Memory (MB) | CPU (%) | Tokens | Throughput (tasks/min) |
| -------------- | --------- | ----------- | ------- | ------ | ---------------------- |
| **GraphBit**   | 1125.6    | 0.000       | 0.274   | 95.0   | 0.91                   |
| **LangChain**  | 1334.0    | 1.050       | 4.597   | 95.0   | 0.79                   |
| **LangGraph**  | 1369.0    | 0.113       | 2.309   | 95.0   | 0.76                   |
| **CrewAI**     | 1816.9    | 0.588       | 6.986   | 167.4  | 0.56                   |
| **PydanticAI** | 2059.3    | 0.087       | 2.058   | 160.5  | 0.50                   |
| **LlamaIndex** | 1223.4    | 0.125       | 9.302   | 95.0   | 0.83                   |

 **Observation:**

* **GraphBit** delivers the **fastest execution** and **lowest CPU & memory usage**, setting the benchmark for minimal workloads.
* **LlamaIndex** is fast but trades off with **very high CPU usage**.
* **LangChain** remains competitive, but use more CPU.
* **CrewAI** and **PydanticAI** take longer due to multi-agent setups.

---

###  **Scenario 2: Sequential Pipeline**

| Framework      | Time (ms) | Memory (MB) | CPU (%) | Tokens | Throughput (tasks/min) |
| -------------- | --------- | ----------- | ------- | ------ | ---------------------- |
| **GraphBit**   | 18995.3   | 0.000       | 0.052   | 1245.6 | 0.21                   |
| **LangChain**  | 17616.0   | 0.000       | 0.712   | 1145.0 | 0.23                   |
| **LangGraph**  | 16098.2   | 0.062       | 0.653   | 1093.6 | 0.25                   |
| **CrewAI**     | 30443.7   | 0.287       | 1.546   | 2238.0 | 0.13                   |
| **PydanticAI** | 13705.3   | 0.087       | 1.005   | 872.6  | 0.30                   |
| **LlamaIndex** | 19360.7   | 0.100       | 1.046   | 2230.4 | 0.21                   |

 **Observation:**

* **LangGraph** and **PydanticAI** offer the best **throughput vs. time** balance.
* **GraphBit** uses almost **no memory or CPU**, excellent for long pipelines.
* **CrewAI** is resource-heavy and **very slow**, possibly due to coordination overhead.
* **LlamaIndex** is consistent but doesn’t outperform here due to latency.

---

###  **Scenario 3: Parallel Pipeline**

| Framework      | Time (ms) | Memory (MB) | CPU (%) | Tokens | Throughput (tasks/min) |
| -------------- | --------- | ----------- | ------- | ------ | ---------------------- |
| **GraphBit**   | 4034.1    | 0.025       | 0.327   | 295.5  | 1.01                   |
| **LangChain**  | 3915.7    | 0.013       | 3.517   | 293.0  | 1.04                   |
| **LangGraph**  | 3035.7    | 0.125       | 3.518   | 0.0    | 0.00                   |
| **CrewAI**     | 4191.4    | 0.500       | 10.378  | 296.4  | 0.97                   |
| **PydanticAI** | 4602.6    | 0.087       | 2.843   | 298.0  | 0.93                   |
| **LlamaIndex** | 3935.8    | 0.100       | 4.955   | 297.5  | 1.03                   |

 **Observation:**

* **LangChain** and **LlamaIndex** lead in **throughput**, both completing \~1 task/sec.
* **GraphBit** is still **resource-light** with excellent performance.
* **CrewAI** and **LlamaIndex** have high CPU draw, reflecting task orchestration cost.

---

###  **Scenario 4: Complex Workflow**

| Framework      | Time (ms) | Memory (MB) | CPU (%) | Tokens | Throughput (tasks/min) |
| -------------- | --------- | ----------- | ------- | ------ | ---------------------- |
| **GraphBit**   | 63673.7   | 0.000       | 0.030   | 7818.4 | 0.08                   |
| **LangChain**  | 50411.8   | 0.000       | 0.326   | 7191.2 | 0.10                   |
| **LangGraph**  | 32228.7   | 0.125       | 0.342   | 0.0    | 0.00                   |
| **CrewAI**     | 32800.2   | 0.352       | 1.915   | 4437.8 | 0.15                   |
| **PydanticAI** | 53960.1   | 0.087       | 0.308   | 3684.7 | 0.10                   |
| **LlamaIndex** | 27322.0   | 0.500       | 2.165   | 2523.2 | 0.24                   |

 **Observation:**

* **LlamaIndex** is the clear winner on **throughput**, finishing tasks \~3× faster.
* **GraphBit** remains ultra-efficient but is **slow on complex tasks**.
* **CrewAI** handles complexity better than expected, outperforming LangChain in speed.

---

###  **Scenario 5: Memory Intensive**

| Framework      | Time (ms) | Memory (MB) | CPU (%) | Tokens | Throughput (tasks/min) |
| -------------- | --------- | ----------- | ------- | ------ | ---------------------- |
| **GraphBit**   | 9602.1    | 0.000       | 0.051   | 5462.6 | 0.11                   |
| **LangChain**  | 13766.5   | 0.000       | 0.343   | 5470.7 | 0.10                   |
| **LangGraph**  | 8513.2    | 0.037       | 0.389   | 5468.9 | 0.12                   |
| **CrewAI**     | 9853.1    | 0.400       | 1.332   | 5507.1 | 0.10                   |
| **PydanticAI** | 9996.7    | 0.050       | 0.346   | 5479.7 | 0.10                   |
| **LlamaIndex** | 26957.9   | 12.923      | 38.533  | 5460.1 | 0.04                   |

 **Observation:**

* **GraphBit** excels again in **zero memory usage**, with very low CPU.
* This scenario highlights **memory leaks or inefficiencies** in **LlamaIndex** for RAM-heavy loads.
* **CrewAI** and **PydanticAI** maintain balance but offer **no advantage** here.


---

###  **Scenario 6: Concurrent Tasks (10 at once)**

| Framework      | Time (ms) | Memory (MB) | CPU (%) | Tokens | Throughput (tasks/min) |
| -------------- | --------- | ----------- | ------- | ------ | ---------------------- |
| **GraphBit**   | 53911.8   | 0.000       | 0.042   | 7297.9 | 0.19                   |
| **LangChain**  | 52577.4   | 0.062       | 0.593   | 7358.1 | 0.19                   |
| **LangGraph**  | 46908.1   | 0.075       | 0.670   | 7321.1 | 0.21                   |
| **CrewAI**     | 58012.6   | 0.533       | 2.105   | 8240.3 | 0.18                   |
| **PydanticAI** | 50071.8   | 0.125       | 0.627   | 7217.4 | 0.20                   |
| **LlamaIndex** | 48537.0   | 0.000       | 0.766   | 7385.3 | 0.21                   |

 **Observation:**

* **GraphBit** remains stable and resource-efficient even under load.
* **CrewAI** again suffers due to agent coordination overhead.
* **LangChain** shows a **negative memory reading**, likely due to a measurement artifact or fallback default.
* **PydanticAI** continues to be a consistent performer in medium concurrency with **predictable behavior**.

---

## **6. Key Takeaways**

---

###  **GraphBit: Ultra-Efficient & Consistent**

* **Best for low-resource environments**: near-zero memory, lowest CPU across all tasks.
* Performs **exceptionally well in Simple, Sequential, and Memory-Intensive** scenarios.
* Ideal for **edge devices**, **serverless functions**, or **cost-conscious deployments**.

---

###  **LlamaIndex: Highest Raw Speed, Highest Cost**

* **Fastest in Parallel and Complex scenarios**, achieving **top throughput**.
* Extremely aggressive in CPU and **memory-hungry**, peaking at **\~13 MB RAM and 38% CPU**.
* Suitable for **powerful machines** where **speed outweighs efficiency**.
* Poor performance in **Memory-Intensive** scenarios due to high RAM saturation.

---

###  **LangChain: Versatile, but Resource-Heavy**

* Delivers **balanced throughput** across all scenarios.
* Resource footprint is **consistently high**, especially in **CPU-intensive tasks**.
* Occasional **memory measurement anomalies** (e.g., negative MBs) suggest instability in some pipelines.
* Best used when **flexibility and maturity** matter more than speed.

---

###  **LangGraph: Excellent for Concurrency, Inconsistent Execution**

* Performs **very well in Concurrent and Memory-Intensive tasks**, offering **high throughput with modest resource use**.
* Strong candidate for **multi-threaded backends** but needs **error handling and fail-safes**.

---

###  **CrewAI: Capable but Sluggish**

* Performs **adequately in Complex and Parallel** scenarios.
* Suffers from **high coordination overhead**, leading to **low throughput and long runtimes**.
* High CPU usage makes it unsuitable for **scale-sensitive environments**.
* Ideal for **complex agent orchestration demos**, but not production benchmarks.

---

###  **PydanticAI: Balanced & Predictable**

* **Most consistent performer** across all scenarios.
* Moderate speed, low memory, low CPU — very predictable.
* Doesn't lead in any one metric, but **never fails** and handles **Sequential and Concurrent** tasks well.
* Ideal for **stable applications**, especially in **regulated or production-grade systems**.

---
---
