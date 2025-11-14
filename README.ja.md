<div align="center">

# GraphBit - 高性能エージェントフレームワーク (日本語)

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

## クイックスタート

### インストール

仮想環境の使用を推奨します。

```bash
pip install graphbit
```

### 環境設定

`.env`ファイルを作成：

```env
OPENAI_API_KEY=your_api_key_here
```

### 基本的な例

```python
from graphbit import Agent

# エージェントを作成
agent = Agent(
    name="assistant",
    model="gpt-4",
    instructions="You are a helpful assistant."
)

# エージェントを実行
result = agent.run("Hello, GraphBit!")
print(result)
```

## ドキュメント

完全なドキュメントについては、[https://docs.graphbit.ai/](https://docs.graphbit.ai/)をご覧ください。

## 貢献

貢献を歓迎します！開発セットアップとガイドラインについては、[Contributing](CONTRIBUTING.md)ファイルをご覧ください。

## セキュリティ

セキュリティ脆弱性を発見した場合は、公開イシューを作成するのではなく、GitHub Securityまたはメールで責任を持って報告してください。

詳細な報告手順と対応タイムラインについては、[Security Policy](SECURITY.md)をご覧ください。

## ライセンス

GraphBitは3層モデルでライセンスされています：**モデルA（無料使用）**は個人、学術機関、小規模チーム（最大10名の従業員/ユーザー）向け、**モデルB（無料トライアル）**は30日間の評価向け、**モデルC（エンタープライズ）**は商用/本番使用向けです。明示的なエンタープライズライセンスなしでは、すべてのモデルで再配布が禁止されています。

完全な利用規約については、[Full License](LICENSE.md)をご覧ください。

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.

---

**注意**: この翻訳はコミュニティによって維持されています。エラーを見つけた場合や翻訳を改善したい場合は、プルリクエストを送信してください。

