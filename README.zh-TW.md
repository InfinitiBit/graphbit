<div align="center">

# GraphBit - 高效能智慧體框架 (繁體中文)

<p align="center">
    <img src="assets/GraphBit_Final_GB_Github_GIF.gif" style="max-width: 100%; height: auto;" alt="Logo" />
</p>
<p align="center">
    <img alt="GraphBit - Developer-first, enterprise-grade LLM framework. | Product Hunt" loading="lazy" width="250" height="54" decoding="async" data-nimg="1" class="w-auto h-[54px] max-w-[250px]" style="color:transparent" src="https://api.producthunt.com/widgets/embed-image/v1/featured.svg?post_id=1004951&amp;theme=light&amp;t=1757340621693"> <img alt="GraphBit - Developer-first, enterprise-grade LLM framework. | Product Hunt" loading="lazy" width="250" height="54" decoding="async" data-nimg="1" class="w-auto h-[54px] max-w-[250px]" style="color:transparent" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=1004951&amp;theme=light&amp;period=daily&amp;t=1757933101511">
</p>

<p align="center">
    <a href="https://graphbit.ai/">Website</a> |
    <a href="https://docs.graphbit.ai/">Docs</a> |
    <a href="https://discord.com/invite/huVJwkyu">Discord</a>
    <br /><br />
</p>

<p align="center">
    <a href="https://pypi.org/project/graphbit/"><img src="https://img.shields.io/pypi/v/graphbit?color=blue&label=PyPI" alt="PyPI"></a>
    <a href="https://pypi.org/project/graphbit/"><img src="https://img.shields.io/pypi/dm/graphbit?color=blue&label=Downloads" alt="PyPI Downloads"></a>
    <a href="https://github.com/InfinitiBit/graphbit/actions/workflows/update-docs.yml"><img src="https://img.shields.io/github/actions/workflow/status/InfinitiBit/graphbit/update-docs.yml?branch=main&label=Build" alt="Build Status"></a>
    <a href="https://github.com/InfinitiBit/graphbit/blob/main/CONTRIBUTING.md"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome"></a>
    <br>
    <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.70+-orange.svg?logo=rust" alt="Rust Version"></a>
    <a href="https://www.python.org"><img src="https://img.shields.io/badge/python-3.10--3.13-blue.svg?logo=python&logoColor=white" alt="Python Version"></a>
    <a href="https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md"><img src="https://img.shields.io/badge/license-Custom-lightgrey.svg" alt="License"></a>

</p>
<p align="center">
    <a href="https://www.youtube.com/@graphbitAI"><img src="https://img.shields.io/badge/YouTube-FF0000?logo=youtube&logoColor=white" alt="YouTube"></a>
    <a href="https://x.com/graphbit_ai"><img src="https://img.shields.io/badge/X-000000?logo=x&logoColor=white" alt="X"></a>
    <a href="https://discord.com/invite/huVJwkyu"><img src="https://img.shields.io/badge/Discord-7289da?logo=discord&logoColor=white" alt="Discord"></a>
    <a href="https://www.linkedin.com/showcase/graphbitai/"><img src="https://img.shields.io/badge/LinkedIn-0077B5?logo=linkedin&logoColor=white" alt="LinkedIn"></a>
</p>

**具有 Rust 效能的型別安全 AI 智慧體工作流程**

</div>

---

🚧 **翻譯進行中** - 本文件正在從英文翻譯中。

📖 **[Read in English](README.md)** | **[閱讀英文版](README.md)**

---

**其他語言版本**: [🇨🇳 简体中文](README.zh-CN.md) | [🇪🇸 Español](README.es.md) | [🇫🇷 Français](README.fr.md) | [🇩🇪 Deutsch](README.de.md) | [🇯🇵 日本語](README.ja.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇮🇹 Italiano](README.it.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## 關於 GraphBit

GraphBit 是一個開源的智慧體 AI 框架，專為需要確定性、並行和低開銷執行的開發者設計。

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
    <img src="https://img.youtube.com/vi/MaCl5oENeAY/maxresdefault.jpg" alt="GraphBit Benchmark Demo" style="max-width: 100%; height: auto;">
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
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 100%; height: auto;">
  </a>
  <p><em>觀看透過 PyPI 安裝 GraphBit | 完整範例和執行指南教學</em></p>
</div>


### 環境設定

建立 `.env` 檔案：

```env
OPENAI_API_KEY=your_api_key_here
```

### 基本範例

```python
from graphbit import Agent

# 建立智慧體
agent = Agent(
    name="assistant",
    model="gpt-4",
    instructions="You are a helpful assistant."
)

# 執行智慧體
result = agent.run("Hello, GraphBit!")
print(result)
```

## 文件

完整文件請造訪：[https://docs.graphbit.ai/](https://docs.graphbit.ai/)


### 使用 GraphBit 建立您的第一個智慧體工作流程

<div align="center">
  <a href="https://www.youtube.com/watch?v=gKvkMc2qZcA">
    <img src="https://img.youtube.com/vi/gKvkMc2qZcA/maxresdefault.jpg" alt="Making Agent Workflow by GraphBit" style="max-width: 100%; height: auto;">
  </a>
  <p><em>觀看使用 GraphBit 建立智慧體工作流程教學</em></p>
</div>

## 貢獻

我們歡迎貢獻！請查看 [Contributing](CONTRIBUTING.md) 檔案了解開發設定和指南。

## 安全性

如果您發現安全漏洞，請透過 GitHub Security 或電子郵件負責任地報告，而不是建立公開問題。

詳細報告程序和回應時間表，請參閱我們的 [Security Policy](SECURITY.md)。

## 授權

GraphBit 採用三層授權模式：**模式 A（免費使用）** 適用於個人、學術機構和小型團隊（最多 10 名員工/使用者），**模式 B（免費試用）** 適用於 30 天評估，**模式 C（企業版）** 適用於商業/生產使用。未經明確的企業授權，所有模式下均禁止重新分發。

完整條款和條件，請參閱 [Full License](LICENSE.md)。

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.

---

**注意**: 此翻譯由社群維護。如果您發現任何錯誤或想要改進翻譯，請提交 Pull Request。

