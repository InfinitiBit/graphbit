<div align="center">

# GraphBit - 高性能智能体框架 (简体中文)

<p align="center">
    <img src="assets/GraphBit_Final_GB_Github_GIF.gif" width="160px" alt="Logo" />
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

**具有 Rust 性能的类型安全 AI 智能体工作流**

</div>

---

🚧 **翻译进行中** - 本文档正在从英文翻译中。

📖 **[Read in English](README.md)** | **[阅读英文版](README.md)**

---

**其他语言版本**: [🇨🇳 繁體中文](README.zh-TW.md) | [🇪🇸 Español](README.es.md) | [🇫🇷 Français](README.fr.md) | [🇩🇪 Deutsch](README.de.md) | [🇯🇵 日本語](README.ja.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇮🇹 Italiano](README.it.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## 关于 GraphBit

GraphBit 是一个开源的智能体 AI 框架，专为需要确定性、并发和低开销执行的开发者设计。

## 为什么选择 GraphBit？

效率决定谁能扩展规模。GraphBit 专为需要确定性、并发和超高效 AI 执行而无需额外开销的开发者而构建。

GraphBit 采用 Rust 核心和最小化的 Python 层，与其他框架相比，CPU 使用率降低高达 68 倍，内存占用降低 140 倍，同时保持相同或更高的吞吐量。

它支持并行运行的多智能体工作流，跨步骤持久化内存，从故障中自我恢复，并确保 100% 的任务可靠性。GraphBit 专为生产工作负载而构建，从企业 AI 系统到低资源边缘部署。

## 主要特性

- **工具选择** - LLM 根据描述智能选择工具
- **类型安全** - 每个执行层都有强类型
- **可靠性** - 断路器、重试策略、错误处理和故障恢复
- **多 LLM 支持** - OpenAI、Azure OpenAI、Anthropic、OpenRouter、DeepSeek、Replicate、Ollama、TogetherAI 等
- **资源管理** - 并发控制和内存优化
- **可观测性** - 内置跟踪、结构化日志和性能指标

## 快速开始

### 安装

建议使用虚拟环境。

```bash
pip install graphbit
```

### 环境设置

创建 `.env` 文件：

```env
OPENAI_API_KEY=your_api_key_here
```

### 基本示例

```python
from graphbit import Agent

# 创建智能体
agent = Agent(
    name="assistant",
    model="gpt-4",
    instructions="You are a helpful assistant."
)

# 运行智能体
result = agent.run("Hello, GraphBit!")
print(result)
```

## 文档

完整文档请访问：[https://docs.graphbit.ai/](https://docs.graphbit.ai/)

## 贡献

我们欢迎贡献！请查看 [Contributing](CONTRIBUTING.md) 文件了解开发设置和指南。

## 安全

如果您发现安全漏洞，请通过 GitHub Security 或电子邮件负责任地报告，而不是创建公开问题。

详细报告程序和响应时间表，请参阅我们的 [Security Policy](SECURITY.md)。

## 许可证

GraphBit 采用三层许可模式：**模式 A（免费使用）** 适用于个人、学术机构和小型团队（最多 10 名员工/用户），**模式 B（免费试用）** 适用于 30 天评估，**模式 C（企业版）** 适用于商业/生产使用。未经明确的企业许可证，所有模式下均禁止重新分发。

完整条款和条件，请参阅 [Full License](LICENSE.md)。

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.

---

**注意**: 此翻译由社区维护。如果您发现任何错误或想要改进翻译，请提交 Pull Request。

