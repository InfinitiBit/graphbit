<div align="center">

# GraphBit - 高性能智能体框架 (简体中文)

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

## 基准测试

GraphBit 为大规模效率而构建，不是理论声明，而是实测结果。

我们的内部基准测试套件在相同工作负载下将 GraphBit 与领先的基于 Python 的智能体框架进行了比较。

| 指标                | GraphBit        | 其他框架         | 增益                     |
|:--------------------|:---------------:|:----------------:|:-------------------------|
| CPU 使用率          | 1.0× 基准       | 68.3× 更高       | ~68× CPU                 |
| 内存占用            | 1.0× 基准       | 140× 更高        | ~140× 内存               |
| 执行速度            | ≈ 相等 / 更快   | —                | 一致的吞吐量             |
| 确定性              | 100% 成功       | 可变             | 保证的可靠性             |

GraphBit 在 LLM 调用、工具调用和多智能体链中始终提供生产级效率。

### 基准测试演示

<div align="center">
  <a href="https://www.youtube.com/watch?v=MaCl5oENeAY">
    <img src="https://img.youtube.com/vi/MaCl5oENeAY/maxresdefault.jpg" alt="GraphBit Benchmark Demo" style="max-width: 100%; height: auto;">
  </a>
  <p><em>观看 GraphBit 基准测试演示</em></p>
</div>

## 何时使用 GraphBit

如果您需要以下功能，请选择 GraphBit：

- 不会在负载下崩溃的生产级多智能体系统
- 类型安全的执行和可重现的输出
- 用于混合或流式 AI 应用的实时编排
- Rust 级别的效率和 Python 级别的人体工程学

如果您正在扩展原型之外或关心运行时确定性，GraphBit 适合您。

## 快速开始

### 安装

建议使用虚拟环境。

```bash
pip install graphbit
```

### 快速入门视频教程

<div align="center">
  <a href="https://youtu.be/ti0wbHFKKFM?si=hnxi-1W823z5I_zs">
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 100%; height: auto;">
  </a>
  <p><em>观看通过 PyPI 安装 GraphBit | 完整示例和运行指南教程</em></p>
</div>


### 环境设置

设置您想在项目中使用的 API 密钥：
```bash
# OpenAI（可选 – 如果使用 OpenAI 模型则需要）
export OPENAI_API_KEY=your_openai_api_key_here

# Anthropic（可选 – 如果使用 Anthropic 模型则需要）
export ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

> **安全提示**：切勿将 API 密钥提交到版本控制。始终使用环境变量或安全的密钥管理。

### 基本用法
```python
import os

from graphbit import LlmConfig, Executor, Workflow, Node, tool

# 初始化和配置
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# 创建执行器
executor = Executor(config)

# 创建具有清晰描述的工具以供 LLM 选择
@tool(_description="获取任何城市的当前天气信息")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}

@tool(_description="执行数学计算并返回结果")
def calculate(expression: str) -> str:
    return f"Result: {eval(expression)}"

# 构建工作流
workflow = Workflow("Analysis Pipeline")

# 创建智能体节点
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

# 连接和执行
id1 = workflow.add_node(smart_agent)
id2 = workflow.add_node(processor)
workflow.connect(id1, id2)

result = executor.execute(workflow)
print(f"Workflow completed: {result.is_success()}")
print("\nSmart Agent Output: \n", result.get_node_output("Smart Agent"))
print("\nData Processor Output: \n", result.get_node_output("Data Processor"))
```

## 文档

完整文档请访问：[https://docs.graphbit.ai/](https://docs.graphbit.ai/)


### 使用 GraphBit 构建您的第一个智能体工作流

<div align="center">
  <a href="https://www.youtube.com/watch?v=gKvkMc2qZcA">
    <img src="https://img.youtube.com/vi/gKvkMc2qZcA/maxresdefault.jpg" alt="Making Agent Workflow by GraphBit" style="max-width: 100%; height: auto;">
  </a>
  <p><em>观看使用 GraphBit 创建智能体工作流教程</em></p>
</div>

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

