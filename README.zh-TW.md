<div align="center">

# GraphBit - 高效能智慧體框架 (繁體中文)

<p align="center">
    <img src="assets/logo(circle).png" width="160px" alt="Logo" />
</p>

<p align="center">
    <a href="https://graphbit.ai/">Website</a> |
    <a href="https://docs.graphbit.ai/">Docs</a> |
    <a href="https://discord.com/invite/huVJwkyu">Discord</a>
    <br /><br />
</p>

[![Build Status](https://img.shields.io/github/actions/workflow/status/InfinitiBit/graphbit/update-docs.yml?branch=main)](https://github.com/InfinitiBit/graphbit/actions/workflows/update-docs.yml)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/InfinitiBit/graphbit/blob/main/CONTRIBUTING.md)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Python Version](https://img.shields.io/badge/python-3.10--3.13-blue.svg)](https://www.python.org)

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

## 快速開始

### 安裝

建議使用虛擬環境。

```bash
pip install graphbit
```

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

