<div align="center">

# GraphBit - 高效能智慧體框架 (繁體中文)

<p align="center">
    <img src="../assets/GraphBit_Final_GB_Github_GIF.gif" style="max-width: 600px; height: auto;" alt="Logo" />
</p>
<p align="center">
    <img alt="GraphBit - Developer-first, enterprise-grade LLM framework. | Product Hunt" loading="lazy" width="250" height="54" decoding="async" data-nimg="1" class="w-auto h-[54px] max-w-[250px]" style="color:transparent" src="https://api.producthunt.com/widgets/embed-image/v1/featured.svg?post_id=1004951&amp;theme=light&amp;t=1757340621693"> <img alt="GraphBit - Developer-first, enterprise-grade LLM framework. | Product Hunt" loading="lazy" width="250" height="54" decoding="async" data-nimg="1" class="w-auto h-[54px] max-w-[250px]" style="color:transparent" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=1004951&amp;theme=light&amp;period=daily&amp;t=1757933101511">
</p>

<p align="center">
    <a href="https://graphbit.ai/">Website</a> |
    <a href="https://docs.graphbit.ai/">Docs</a> |
    <a href="https://discord.com/invite/FMhgB3paMD">Discord</a>
    <br /><br />
</p>

</p>

<p align="center">
    <a href="https://trendshift.io/repositories/14884" target="_blank"><img src="https://trendshift.io/api/badge/repositories/14884" alt="InfinitiBit%2Fgraphbit | Trendshift" style="width: 250px; height: 55px;" width="250" height="55"/></a>
    <br>
    <a href="https://pepy.tech/projects/graphbit"><img src="https://static.pepy.tech/personalized-badge/graphbit?period=total&units=INTERNATIONAL_SYSTEM&left_color=GREY&right_color=GREEN&left_text=Downloads" alt="PyPI Downloads"/></a>
</p>

<p align="center">
    <a href="https://pypi.org/project/graphbit/"><img src="https://img.shields.io/pypi/v/graphbit?color=blue&label=PyPI" alt="PyPI"></a>
    <a href="https://pypi.org/project/graphbit/"><img src="https://img.shields.io/pypi/dm/graphbit?color=blue&label=Downloads" alt="PyPI Downloads"></a>
    <a href="https://github.com/InfinitiBit/graphbit/actions/workflows/update-docs.yml"><img src="https://img.shields.io/github/actions/workflow/status/InfinitiBit/graphbit/update-docs.yml?branch=main&label=Build" alt="Build Status"></a>
    <a href="https://github.com/InfinitiBit/graphbit/blob/main/CONTRIBUTING.md"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome"></a>
    <br>
    <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.70+-orange.svg?logo=rust" alt="Rust Version"></a>
    <a href="https://www.python.org"><img src="https://img.shields.io/badge/python-3.9--3.13-blue.svg?logo=python&logoColor=white" alt="Python Version"></a>
    <a href="https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md"><img src="https://img.shields.io/badge/license-Custom-lightgrey.svg" alt="License"></a>

</p>
<p align="center">
    <a href="https://www.youtube.com/@graphbitAI"><img src="https://img.shields.io/badge/YouTube-FF0000?logo=youtube&logoColor=white" alt="YouTube"></a>
    <a href="https://x.com/graphbit_ai"><img src="https://img.shields.io/badge/X-000000?logo=x&logoColor=white" alt="X"></a>
    <a href="https://discord.com/invite/FMhgB3paMD"><img src="https://img.shields.io/badge/Discord-7289da?logo=discord&logoColor=white" alt="Discord"></a>
    <a href="https://www.linkedin.com/showcase/graphbitai/"><img src="https://img.shields.io/badge/LinkedIn-0077B5?logo=linkedin&logoColor=white" alt="LinkedIn"></a>
</p>

**具有 Rust 效能的型別安全 AI 智慧體工作流程**

</div>

---

🚧 **翻譯進行中** - 本文件正在從英文翻譯中。

📖 **[Read in English](../README.md)** | **[閱讀英文版](../README.md)**

---

**其他語言版本**: [🇨🇳 简体中文](README.zh-CN.md) | [🇪🇸 Español](README.es.md) | [🇫🇷 Français](README.fr.md) | [🇩🇪 Deutsch](README.de.md) | [🇯🇵 日本語](README.ja.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇮🇹 Italiano](README.it.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## 關於 GraphBit

GraphBit 是一個原始碼可用的智慧體 AI 框架，專為需要確定性、並行和低開銷執行的開發者設計。

## 為什麼選擇 GraphBit？

效率決定誰能擴展規模。GraphBit 專為需要確定性、並行和超高效 AI 執行而無需額外開銷的開發者而建構。

GraphBit 採用 Rust 核心和最小化的 Python 層，與其他框架相比，CPU 使用率降低高達 68 倍，記憶體佔用降低 140 倍，同時保持相同或更高的吞吐量。

它支援並行執行的多智慧體工作流程，跨步驟持久化記憶體，從故障中自我恢復，並確保 100% 的任務可靠性。GraphBit 專為生產工作負載而建構，從企業 AI 系統到低資源邊緣部署。

## 主要特性

- **工具選擇** - LLM 根據描述智慧選擇工具
- **型別安全** - 每個執行層都有強型別
- **可靠性** - 斷路器、重試策略、錯誤處理和故障恢復
- **多 LLM 支援** - OpenAI、Azure OpenAI、Anthropic、OpenRouter、DeepSeek、Replicate、Ollama、TogetherAI 等
- **資源管理** - 並行控制和記憶體最佳化
- **可觀測性** - 內建追蹤、結構化日誌和效能指標

## 基準測試

GraphBit 為大規模效率而構建，不是理論聲明，而是實測結果。

我們的內部基準測試套件在相同工作負載下將 GraphBit 與領先的基於 Python 的智慧體框架進行了比較。

| 指標                | GraphBit        | 其他框架         | 增益                     |
|:--------------------|:---------------:|:----------------:|:-------------------------|
| CPU 使用率          | 1.0× 基準       | 68.3× 更高       | ~68× CPU                 |
| 記憶體佔用          | 1.0× 基準       | 140× 更高        | ~140× 記憶體             |
| 執行速度            | ≈ 相等 / 更快   | —                | 一致的吞吐量             |
| 確定性              | 100% 成功       | 可變             | 保證的可靠性             |

GraphBit 在 LLM 呼叫、工具呼叫和多智慧體鏈中始終提供生產級效率。

### 基準測試演示

<div align="center">
  <a href="https://www.youtube.com/watch?v=MaCl5oENeAY">
    <img src="https://img.youtube.com/vi/MaCl5oENeAY/maxresdefault.jpg" alt="GraphBit Benchmark Demo" style="max-width: 600px; height: auto;">
  </a>
  <p><em>觀看 GraphBit 基準測試演示</em></p>
</div>

## 何時使用 GraphBit

如果您需要以下功能，請選擇 GraphBit：

- 不會在負載下崩潰的生產級多智慧體系統
- 類型安全的執行和可重現的輸出
- 用於混合或串流 AI 應用的即時編排
- Rust 級別的效率和 Python 級別的人體工程學

如果您正在擴展原型之外或關心執行時確定性，GraphBit 適合您。

## 快速開始

### 安裝

建議使用虛擬環境。

```bash
pip install graphbit
```

### 快速入門影片教學

<div align="center">
  <a href="https://youtu.be/ti0wbHFKKFM?si=hnxi-1W823z5I_zs">
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 600px; height: auto;">
  </a>
  <p><em>觀看透過 PyPI 安裝 GraphBit | 完整範例和執行指南教學</em></p>
</div>


### 環境設定

設定您想在專案中使用的 API 金鑰：
```bash
# OpenAI（選用 – 如果使用 OpenAI 模型則需要）
export OPENAI_API_KEY=your_openai_api_key_here

# Anthropic（選用 – 如果使用 Anthropic 模型則需要）
export ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

> **安全提示**：切勿將 API 金鑰提交到版本控制。始終使用環境變數或安全的金鑰管理。

### 基本用法
```python
import os

from graphbit import LlmConfig, Executor, Workflow, Node, tool

# 初始化和配置
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# 建立執行器
executor = Executor(config)

# 建立具有清晰描述的工具以供 LLM 選擇
@tool(_description="取得任何城市的當前天氣資訊")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}

@tool(_description="執行數學計算並返回結果")
def calculate(expression: str) -> str:
    return f"Result: {eval(expression)}"

# 建立工作流程
workflow = Workflow("Analysis Pipeline")

# 建立智慧體節點
smart_agent = Node.agent(
    name="Smart Agent",
    prompt="What's the weather in Paris and calculate 15 + 27?",
    system_prompt="You are an assistant skilled in weather lookup and math calculations. Use tools to answer queries accurately.",
    tools=[get_weather, calculate]
)

processor = Node.agent(
    name="Data Processor",
    prompt="Process the results obtained from Smart Agent.",
    system_prompt="""You process and organize results from other agents.

    - Summarize and clarify key points
    - Structure your output for easy reading
    - Focus on actionable insights
    """
)

# 連接和執行
id1 = workflow.add_node(smart_agent)
id2 = workflow.add_node(processor)
workflow.connect(id1, id2)

result = executor.execute(workflow)
print(f"Workflow completed: {result.is_success()}")
print("\nSmart Agent Output: \n", result.get_node_output("Smart Agent"))
print("\nData Processor Output: \n", result.get_node_output("Data Processor"))
```

## 可觀測性與追蹤

GraphBit Tracer 以最小配置捕獲和監控 LLM 呼叫和 AI 工作流程。它包裝 GraphBit LLM 客戶端和工作流程執行器，以追蹤提示、回應、令牌使用、延遲和錯誤，而無需更改您的程式碼。

<div align="center">
  <a href="https://www.youtube.com/watch?v=nzwrxSiRl2U">
    <img src="https://img.youtube.com/vi/nzwrxSiRl2U/maxresdefault.jpg" alt="GraphBit Observability & Tracing" style="max-width: 600px; height: auto;">
  </a>
  <p><em>觀看 GraphBit 可觀測性與追蹤教學</em></p>
</div>

## 高層架構

<p align="center">
  <img src="../assets/architecture.svg" height="250" alt="GraphBit Architecture">
</p>

三層設計確保可靠性和效能：
- **Rust 核心** - 工作流程引擎、智慧體和 LLM 提供商
- **編排層** - 專案管理和執行
- **Python API** - PyO3 綁定，支援非同步

## Python API 整合

GraphBit 提供豐富的 Python API 用於建立和整合智慧體工作流程：

- **LLM 客戶端** - 多提供商 LLM 整合（OpenAI、Anthropic、Azure 等）
- **工作流程** - 定義和管理具有狀態管理的多智慧體工作流程圖
- **節點** - 智慧體節點、工具節點和自訂工作流程元件
- **執行器** - 具有配置管理的工作流程執行引擎
- **工具系統** - 智慧體工具的函式裝飾器、註冊表和執行框架
- **工作流程結果** - 帶有中繼資料、時間和輸出存取的執行結果
- **嵌入** - 用於語義搜尋和檢索的向量嵌入
- **工作流程上下文** - 工作流程執行過程中的共享狀態和變數
- **文件載入器** - 從多種格式載入和解析文件（PDF、DOCX、TXT、JSON、CSV、XML、HTML）
- **文字分割器** - 將文件分割成塊（字元、令牌、句子、遞迴）

有關類別、方法和使用範例的完整清單，請參閱 [Python API 參考](docs/api-reference/python-api.md)。

## 文件

完整文件請造訪：[https://docs.graphbit.ai/](https://docs.graphbit.ai/)

## 生態系統與擴展

GraphBit 的模組化架構支援外部整合：

| 類別              | 範例                                                                                          |
|:------------------|:----------------------------------------------------------------------------------------------|
| LLM 提供商        | OpenAI, Anthropic, Azure OpenAI, DeepSeek, Together, Ollama, OpenRouter, Fireworks, Mistral AI, Replicate, Perplexity, HuggingFace, AI21, Bytedance, xAI, 等 |
| 向量儲存          | Pinecone, Qdrant, Chroma, Milvus, Weaviate, FAISS, Elasticsearch, AstraDB, Redis, 等         |
| 資料庫            | PostgreSQL (PGVector), MongoDB, MariaDB, IBM DB2, Redis, 等                                   |
| 雲端平台          | AWS (Boto3), Azure, Google Cloud Platform, 等                                                 |
| 搜尋 API          | Serper, Google Search, GitHub Search, GitLab Search, 等                                       |
| 嵌入模型          | OpenAI Embeddings, Voyage AI, 等                                                              |

擴展由社群開發和維護。

<p align="center">
  <img src="../assets/Ecosystem.png" alt="GraphBit Ecosystem - Stop Choosing, Start Orchestrating" style="max-width: 100%; height: auto;">
</p>


### 使用 GraphBit 建立您的第一個智慧體工作流程

<div align="center">
  <a href="https://www.youtube.com/watch?v=gKvkMc2qZcA">
    <img src="https://img.youtube.com/vi/gKvkMc2qZcA/maxresdefault.jpg" alt="Making Agent Workflow by GraphBit" style="max-width: 600px; height: auto;">
  </a>
  <p><em>觀看使用 GraphBit 建立智慧體工作流程教學</em></p>
</div>

## 為 GraphBit 做出貢獻

我們歡迎貢獻。要開始，請參閱 [Contributing](CONTRIBUTING.md) 檔案以了解開發設定和指南。

GraphBit 由一個優秀的研究人員和工程師社群建立。

<a href="https://github.com/Infinitibit/graphbit/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Infinitibit/graphbit" />
</a>

## 安全性

如果您發現安全漏洞，請透過 GitHub Security 或電子郵件負責任地報告，而不是建立公開問題。

詳細報告程序和回應時間表，請參閱我們的 [Security Policy](SECURITY.md)。

## 授權

GraphBit 採用三層授權模式：**模式 A（免費使用）** 適用於個人、學術機構和小型團隊（最多 10 名員工/使用者），**模式 B（免費試用）** 適用於 30 天評估，**模式 C（企業版）** 適用於商業/生產使用。未經明確的企業授權，所有模式下均禁止重新分發。

完整條款和條件，請參閱 [Full License](LICENSE.md)。

Copyright © 2023–2026 InfinitiBit GmbH. All rights reserved.

---

**注意**: 此翻譯由社群維護。如果您發現任何錯誤或想要改進翻譯，請提交 Pull Request。

