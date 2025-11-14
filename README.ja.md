<div align="center">

# GraphBit - 高性能エージェントフレームワーク (日本語)

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

**Rustのパフォーマンスを持つ型安全なAIエージェントワークフロー**

</div>

---

🚧 **翻訳作業中** - このドキュメントは英語から翻訳中です。

📖 **[Read in English](README.md)** | **[英語で読む](README.md)**

---

**他の言語で読む**: [🇨🇳 简体中文](README.zh-CN.md) | [🇨🇳 繁體中文](README.zh-TW.md) | [🇪🇸 Español](README.es.md) | [🇫🇷 Français](README.fr.md) | [🇩🇪 Deutsch](README.de.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇮🇹 Italiano](README.it.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## GraphBitについて

GraphBitは、決定論的、並行的、低オーバーヘッドの実行を必要とする開発者向けのオープンソースエージェントAIフレームワークです。

## なぜGraphBitなのか？

効率性がスケールを決定します。GraphBitは、オーバーヘッドなしで決定論的、並行的、超効率的なAI実行を必要とする開発者のために構築されています。

Rustコアと最小限のPythonレイヤーで構築されたGraphBitは、他のフレームワークと比較して最大68倍低いCPU使用率と140倍低いメモリフットプリントを実現し、同等以上のスループットを維持します。

並列実行されるマルチエージェントワークフロー、ステップ間でのメモリ永続化、障害からの自己回復、100%のタスク信頼性を保証します。GraphBitは、エンタープライズAIシステムから低リソースエッジデプロイメントまで、本番ワークロード向けに構築されています。

## 主な機能

- **ツール選択** - LLMが説明に基づいてツールをインテリジェントに選択
- **型安全性** - すべての実行レイヤーで強力な型付け
- **信頼性** - サーキットブレーカー、リトライポリシー、エラー処理、障害回復
- **マルチLLMサポート** - OpenAI、Azure OpenAI、Anthropic、OpenRouter、DeepSeek、Replicate、Ollama、TogetherAIなど
- **リソース管理** - 並行制御とメモリ最適化
- **可観測性** - 組み込みトレーシング、構造化ログ、パフォーマンスメトリクス

## ベンチマーク

GraphBit は大規模な効率性のために構築されており、理論的な主張ではなく、測定された結果です。

私たちの内部ベンチマークスイートは、同一のワークロードで GraphBit を主要な Python ベースのエージェントフレームワークと比較しました。

| メトリック          | GraphBit        | 他のフレームワーク | 利得                     |
|:--------------------|:---------------:|:----------------:|:-------------------------|
| CPU 使用率          | 1.0× ベースライン | 68.3× 高い      | ~68× CPU                 |
| メモリフットプリント | 1.0× ベースライン | 140× 高い       | ~140× メモリ             |
| 実行速度            | ≈ 同等 / より速い | —              | 一貫したスループット     |
| 決定性              | 100% 成功       | 可変             | 保証された信頼性         |

GraphBit は、LLM 呼び出し、ツール呼び出し、マルチエージェントチェーン全体で一貫して本番グレードの効率を提供します。

### ベンチマークデモ

<div align="center">
  <a href="https://www.youtube.com/watch?v=MaCl5oENeAY">
    <img src="https://img.youtube.com/vi/MaCl5oENeAY/maxresdefault.jpg" alt="GraphBit Benchmark Demo" style="max-width: 100%; height: auto;">
  </a>
  <p><em>GraphBit ベンチマークデモを見る</em></p>
</div>

## GraphBit を使用するタイミング

次のような場合は GraphBit を選択してください：

- 負荷下で崩壊しない本番グレードのマルチエージェントシステム
- 型安全な実行と再現可能な出力
- ハイブリッドまたはストリーミング AI アプリケーションのリアルタイムオーケストレーション
- Rust レベルの効率性と Python レベルの人間工学

プロトタイプを超えてスケーリングする場合、またはランタイムの決定性を重視する場合、GraphBit はあなたに適しています。

## クイックスタート

### インストール

仮想環境の使用を推奨します。

```bash
pip install graphbit
```

### クイックスタートビデオチュートリアル

<div align="center">
  <a href="https://youtu.be/ti0wbHFKKFM?si=hnxi-1W823z5I_zs">
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 100%; height: auto;">
  </a>
  <p><em>PyPI 経由で GraphBit をインストール | 完全な例と実行ガイドのチュートリアルを見る</em></p>
</div>


### 環境設定

プロジェクトで使用したい API キーを設定します：
```bash
# OpenAI（オプション – OpenAI モデルを使用する場合は必須）
export OPENAI_API_KEY=your_openai_api_key_here

# Anthropic（オプション – Anthropic モデルを使用する場合は必須）
export ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

> **セキュリティ注意事項**：API キーをバージョン管理にコミットしないでください。常に環境変数または安全なシークレット管理を使用してください。

### 基本的な使用方法
```python
import os

from graphbit import LlmConfig, Executor, Workflow, Node, tool

# 初期化と設定
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# エグゼキューターを作成
executor = Executor(config)

# LLM 選択のための明確な説明を持つツールを作成
@tool(_description="任意の都市の現在の天気情報を取得")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}

@tool(_description="数学計算を実行して結果を返す")
def calculate(expression: str) -> str:
    return f"Result: {eval(expression)}"

# ワークフローを構築
workflow = Workflow("Analysis Pipeline")

# エージェントノードを作成
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

# 接続して実行
id1 = workflow.add_node(smart_agent)
id2 = workflow.add_node(processor)
workflow.connect(id1, id2)

result = executor.execute(workflow)
print(f"Workflow completed: {result.is_success()}")
print("\nSmart Agent Output: \n", result.get_node_output("Smart Agent"))
print("\nData Processor Output: \n", result.get_node_output("Data Processor"))
```

## 可観測性とトレーシング

GraphBit Tracer は、最小限の設定で LLM 呼び出しと AI ワークフローをキャプチャおよび監視します。GraphBit LLM クライアントとワークフローエグゼキューターをラップして、コードを変更することなくプロンプト、レスポンス、トークン使用量、レイテンシ、エラーを追跡します。

<div align="center">
  <a href="https://www.youtube.com/watch?v=nzwrxSiRl2U">
    <img src="https://img.youtube.com/vi/nzwrxSiRl2U/maxresdefault.jpg" alt="GraphBit Observability & Tracing" style="max-width: 100%; height: auto;">
  </a>
  <p><em>GraphBit 可観測性とトレーシングのチュートリアルを見る</em></p>
</div>

## 高レベルアーキテクチャ

<p align="center">
  <img src="assets/architecture.svg" height="250" alt="GraphBit Architecture">
</p>

信頼性とパフォーマンスのための3層設計：
- **Rust コア** - ワークフローエンジン、エージェント、LLM プロバイダー
- **オーケストレーション層** - プロジェクト管理と実行
- **Python API** - 非同期サポート付き PyO3 バインディング

## Python API 統合

GraphBit は、エージェントワークフローを構築および統合するための豊富な Python API を提供します：

- **LLM クライアント** - マルチプロバイダー LLM 統合（OpenAI、Anthropic、Azure など）
- **ワークフロー** - 状態管理を備えたマルチエージェントワークフローグラフの定義と管理
- **ノード** - エージェントノード、ツールノード、カスタムワークフローコンポーネント
- **エグゼキューター** - 設定管理を備えたワークフロー実行エンジン
- **ツールシステム** - エージェントツール用の関数デコレーター、レジストリ、実行フレームワーク
- **ワークフロー結果** - メタデータ、タイミング、出力アクセスを含む実行結果
- **埋め込み** - セマンティック検索と取得のためのベクトル埋め込み
- **ワークフローコンテキスト** - ワークフロー実行全体での共有状態と変数
- **ドキュメントローダー** - 複数の形式（PDF、DOCX、TXT、JSON、CSV、XML、HTML）からドキュメントを読み込んで解析
- **テキストスプリッター** - ドキュメントをチャンクに分割（文字、トークン、文、再帰的）

クラス、メソッド、使用例の完全なリストについては、[Python API リファレンス](docs/api-reference/python-api.md)を参照してください。

## ドキュメント

完全なドキュメントについては、[https://docs.graphbit.ai/](https://docs.graphbit.ai/)をご覧ください。

## エコシステムと拡張機能

GraphBit のモジュラーアーキテクチャは外部統合をサポートします：

| カテゴリ          | 例                                                                                            |
|:------------------|:----------------------------------------------------------------------------------------------|
| LLM プロバイダー  | OpenAI, Anthropic, Azure OpenAI, DeepSeek, Together, Ollama, OpenRouter, Fireworks, Mistral AI, Replicate, Perplexity, HuggingFace, AI21, Bytedance, xAI, など |
| ベクトルストア    | Pinecone, Qdrant, Chroma, Milvus, Weaviate, FAISS, Elasticsearch, AstraDB, Redis, など       |
| データベース      | PostgreSQL (PGVector), MongoDB, MariaDB, IBM DB2, Redis, など                                 |
| クラウドプラットフォーム | AWS (Boto3), Azure, Google Cloud Platform, など                                        |
| 検索 API          | Serper, Google Search, GitHub Search, GitLab Search, など                                     |
| 埋め込みモデル    | OpenAI Embeddings, Voyage AI, など                                                            |

拡張機能はコミュニティによって開発および維持されています。

<p align="center">
  <img src="assets/Ecosystem.png" alt="GraphBit Ecosystem - Stop Choosing, Start Orchestrating" style="max-width: 100%; height: auto;">
</p>


### GraphBit で最初のエージェントワークフローを構築する

<div align="center">
  <a href="https://www.youtube.com/watch?v=gKvkMc2qZcA">
    <img src="https://img.youtube.com/vi/gKvkMc2qZcA/maxresdefault.jpg" alt="Making Agent Workflow by GraphBit" style="max-width: 100%; height: auto;">
  </a>
  <p><em>GraphBit でエージェントワークフローを作成するチュートリアルを見る</em></p>
</div>

## GraphBit への貢献

貢献を歓迎します。開始するには、開発セットアップとガイドラインについて [Contributing](CONTRIBUTING.md) ファイルを参照してください。

GraphBit は素晴らしい研究者とエンジニアのコミュニティによって構築されています。

<a href="https://github.com/Infinitibit/graphbit/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Infinitibit/graphbit" />
</a>

## セキュリティ

セキュリティ脆弱性を発見した場合は、公開イシューを作成するのではなく、GitHub Securityまたはメールで責任を持って報告してください。

詳細な報告手順と対応タイムラインについては、[Security Policy](SECURITY.md)をご覧ください。

## ライセンス

GraphBitは3層モデルでライセンスされています：**モデルA（無料使用）**は個人、学術機関、小規模チーム（最大10名の従業員/ユーザー）向け、**モデルB（無料トライアル）**は30日間の評価向け、**モデルC（エンタープライズ）**は商用/本番使用向けです。明示的なエンタープライズライセンスなしでは、すべてのモデルで再配布が禁止されています。

完全な利用規約については、[Full License](LICENSE.md)をご覧ください。

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.

---

**注意**: この翻訳はコミュニティによって維持されています。エラーを見つけた場合や翻訳を改善したい場合は、プルリクエストを送信してください。

