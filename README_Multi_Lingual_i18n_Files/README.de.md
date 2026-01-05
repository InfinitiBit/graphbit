<div align="center">

# GraphBit - Hochleistungs-Agenten-Framework (Deutsch)

<p align="center">
    <img src="../assets/GraphBit_Final_GB_Github_GIF.gif" style="max-width: 600px; height: auto;" alt="Logo" />
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
    <a href="https://www.python.org"><img src="https://img.shields.io/badge/python-3.10--3.13-blue.svg?logo=python&logoColor=white" alt="Python Version"></a>
    <a href="https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md"><img src="https://img.shields.io/badge/license-Custom-lightgrey.svg" alt="License"></a>

</p>
<p align="center">
    <a href="https://www.youtube.com/@graphbitAI"><img src="https://img.shields.io/badge/YouTube-FF0000?logo=youtube&logoColor=white" alt="YouTube"></a>
    <a href="https://x.com/graphbit_ai"><img src="https://img.shields.io/badge/X-000000?logo=x&logoColor=white" alt="X"></a>
    <a href="https://discord.com/invite/huVJwkyu"><img src="https://img.shields.io/badge/Discord-7289da?logo=discord&logoColor=white" alt="Discord"></a>
    <a href="https://www.linkedin.com/showcase/graphbitai/"><img src="https://img.shields.io/badge/LinkedIn-0077B5?logo=linkedin&logoColor=white" alt="LinkedIn"></a>
</p>

**Typsichere KI-Agenten-Workflows mit Rust-Performance**

</div>

---

🚧 **Übersetzung in Arbeit** - Dieses Dokument wird gerade aus dem Englischen übersetzt.

📖 **[Read in English](../README.md)** | **[Auf Englisch lesen](../README.md)**

---

**In anderen Sprachen lesen**: [🇨🇳 简体中文](README.zh-CN.md) | [🇨🇳 繁體中文](README.zh-TW.md) | [🇪🇸 Español](README.es.md) | [🇫🇷 Français](README.fr.md) | [🇯🇵 日本語](README.ja.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇮🇹 Italiano](README.it.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## Über GraphBit

GraphBit ist ein Open-Source-KI-Agenten-Framework für Entwickler, die deterministische, nebenläufige und ressourcenschonende Ausführung benötigen.

## Warum GraphBit?

Effizienz entscheidet, wer skaliert. GraphBit wurde für Entwickler entwickelt, die deterministische, nebenläufige und hocheffiziente KI-Ausführung ohne Overhead benötigen.

Mit einem Rust-Kern und einer minimalen Python-Schicht bietet GraphBit bis zu 68× geringere CPU-Nutzung und 140× geringeren Speicherbedarf als andere Frameworks bei gleichem oder höherem Durchsatz.

Es ermöglicht Multi-Agenten-Workflows, die parallel laufen, Speicher über Schritte hinweg persistieren, sich selbst von Fehlern erholen und 100% Aufgabenzuverlässigkeit garantieren. GraphBit ist für Produktionsworkloads konzipiert, von Unternehmens-KI-Systemen bis hin zu ressourcenbeschränkten Edge-Deployments.

## Hauptmerkmale

- **Werkzeugauswahl** - LLMs wählen intelligent Werkzeuge basierend auf Beschreibungen
- **Typsicherheit** - Starke Typisierung durch jede Ausführungsebene
- **Zuverlässigkeit** - Circuit Breaker, Retry-Richtlinien, Fehlerbehandlung und Wiederherstellung
- **Multi-LLM-Unterstützung** - OpenAI, Azure OpenAI, Anthropic, OpenRouter, DeepSeek, Replicate, Ollama, TogetherAI und mehr
- **Ressourcenverwaltung** - Nebenläufigkeitskontrollen und Speicheroptimierung
- **Beobachtbarkeit** - Integriertes Tracing, strukturierte Logs und Performance-Metriken

## Benchmark

GraphBit wurde für Effizienz im großen Maßstab entwickelt, nicht für theoretische Behauptungen, sondern für gemessene Ergebnisse.

Unsere interne Benchmark-Suite verglich GraphBit mit führenden Python-basierten Agenten-Frameworks bei identischen Workloads.

| Metrik              | GraphBit        | Andere Frameworks | Gewinn                   |
|:--------------------|:---------------:|:----------------:|:-------------------------|
| CPU-Nutzung         | 1.0× Basis      | 68.3× höher      | ~68× CPU                 |
| Speicher-Footprint  | 1.0× Basis      | 140× höher       | ~140× Speicher           |
| Ausführungsgeschwindigkeit | ≈ gleich / schneller | —      | Konsistenter Durchsatz   |
| Determinismus       | 100% Erfolg     | Variabel         | Garantierte Zuverlässigkeit |

GraphBit liefert durchgängig produktionsreife Effizienz bei LLM-Aufrufen, Tool-Aufrufen und Multi-Agenten-Ketten.

### Benchmark Demo

<div align="center">
  <a href="https://www.youtube.com/watch?v=MaCl5oENeAY">
    <img src="https://img.youtube.com/vi/MaCl5oENeAY/maxresdefault.jpg" alt="GraphBit Benchmark Demo" style="max-width: 600px; height: auto;">
  </a>
  <p><em>Sehen Sie sich die GraphBit Benchmark Demo an</em></p>
</div>

## Wann GraphBit Verwenden

Wählen Sie GraphBit, wenn Sie Folgendes benötigen:

- Produktionsreife Multi-Agenten-Systeme, die unter Last nicht zusammenbrechen
- Typsichere Ausführung und reproduzierbare Ausgaben
- Echtzeit-Orchestrierung für hybride oder Streaming-KI-Anwendungen
- Effizienz auf Rust-Niveau mit Ergonomie auf Python-Niveau

Wenn Sie über Prototypen hinaus skalieren oder Ihnen Laufzeit-Determinismus wichtig ist, ist GraphBit für Sie.

## Schnellstart

### Installation

Es wird empfohlen, eine virtuelle Umgebung zu verwenden.

```bash
pip install graphbit
```

### Schnellstart-Video-Tutorial

<div align="center">
  <a href="https://youtu.be/ti0wbHFKKFM?si=hnxi-1W823z5I_zs">
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 600px; height: auto;">
  </a>
  <p><em>Sehen Sie sich das Tutorial zur Installation von GraphBit über PyPI | Vollständiges Beispiel- und Ausführungshandbuch an</em></p>
</div>


### Umgebungseinrichtung

Richten Sie die API-Schlüssel ein, die Sie in Ihrem Projekt verwenden möchten:
```bash
# OpenAI (optional – erforderlich bei Verwendung von OpenAI-Modellen)
export OPENAI_API_KEY=your_openai_api_key_here

# Anthropic (optional – erforderlich bei Verwendung von Anthropic-Modellen)
export ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

> **Sicherheitshinweis**: Committen Sie niemals API-Schlüssel in die Versionskontrolle. Verwenden Sie immer Umgebungsvariablen oder sichere Geheimnisverwaltung.

### Grundlegende Verwendung
```python
import os

from graphbit import LlmConfig, Executor, Workflow, Node, tool

# Initialisieren und konfigurieren
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Executor erstellen
executor = Executor(config)

# Tools mit klaren Beschreibungen für die LLM-Auswahl erstellen
@tool(_description="Aktuelle Wetterinformationen für jede Stadt abrufen")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}

@tool(_description="Mathematische Berechnungen durchführen und Ergebnisse zurückgeben")
def calculate(expression: str) -> str:
    return f"Result: {eval(expression)}"

# Workflow erstellen
workflow = Workflow("Analysis Pipeline")

# Agenten-Knoten erstellen
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

# Verbinden und ausführen
id1 = workflow.add_node(smart_agent)
id2 = workflow.add_node(processor)
workflow.connect(id1, id2)

result = executor.execute(workflow)
print(f"Workflow completed: {result.is_success()}")
print("\nSmart Agent Output: \n", result.get_node_output("Smart Agent"))
print("\nData Processor Output: \n", result.get_node_output("Data Processor"))
```

## Beobachtbarkeit und Tracing

GraphBit Tracer erfasst und überwacht LLM-Aufrufe und KI-Workflows mit minimaler Konfiguration. Es umschließt GraphBit LLM-Clients und Workflow-Executors, um Prompts, Antworten, Token-Nutzung, Latenz und Fehler zu verfolgen, ohne Ihren Code zu ändern.

<div align="center">
  <a href="https://www.youtube.com/watch?v=nzwrxSiRl2U">
    <img src="https://img.youtube.com/vi/nzwrxSiRl2U/maxresdefault.jpg" alt="GraphBit Observability & Tracing" style="max-width: 600px; height: auto;">
  </a>
  <p><em>Sehen Sie sich das Tutorial zu GraphBit Beobachtbarkeit und Tracing an</em></p>
</div>

## High-Level-Architektur

<p align="center">
  <img src="../assets/architecture.svg" height="250" alt="GraphBit Architecture">
</p>

Dreistufiges Design für Zuverlässigkeit und Leistung:
- **Rust-Kern** - Workflow-Engine, Agenten und LLM-Anbieter
- **Orchestrierungsschicht** - Projektverwaltung und Ausführung
- **Python-API** - PyO3-Bindungen mit asynchroner Unterstützung

## Python-API-Integrationen

GraphBit bietet eine umfangreiche Python-API zum Erstellen und Integrieren agentischer Workflows:

- **LLM-Clients** - Multi-Provider-LLM-Integrationen (OpenAI, Anthropic, Azure und mehr)
- **Workflows** - Definieren und verwalten Sie Multi-Agenten-Workflow-Graphen mit Zustandsverwaltung
- **Knoten** - Agentenknoten, Werkzeugknoten und benutzerdefinierte Workflow-Komponenten
- **Executors** - Workflow-Ausführungs-Engine mit Konfigurationsverwaltung
- **Werkzeugsystem** - Funktionsdekoratoren, Registry und Ausführungs-Framework für Agentenwerkzeuge
- **Workflow-Ergebnisse** - Ausführungsergebnisse mit Metadaten, Timing und Ausgabezugriff
- **Embeddings** - Vektor-Embeddings für semantische Suche und Abruf
- **Workflow-Kontext** - Gemeinsamer Zustand und Variablen über die Workflow-Ausführung hinweg
- **Dokumenten-Loader** - Laden und Parsen von Dokumenten aus mehreren Formaten (PDF, DOCX, TXT, JSON, CSV, XML, HTML)
- **Text-Splitter** - Dokumente in Chunks aufteilen (Zeichen, Token, Satz, rekursiv)

Für die vollständige Liste der Klassen, Methoden und Verwendungsbeispiele siehe die [Python-API-Referenz](docs/api-reference/python-api.md).

## Dokumentation

Für vollständige Dokumentation besuchen Sie: [https://docs.graphbit.ai/](https://docs.graphbit.ai/)

## Ökosystem und Erweiterungen

Die modulare Architektur von GraphBit unterstützt externe Integrationen:

| Kategorie         | Beispiele                                                                                     |
|:------------------|:----------------------------------------------------------------------------------------------|
| LLM-Anbieter      | OpenAI, Anthropic, Azure OpenAI, DeepSeek, Together, Ollama, OpenRouter, Fireworks, Mistral AI, Replicate, Perplexity, HuggingFace, AI21, Bytedance, xAI, und mehr |
| Vektorspeicher    | Pinecone, Qdrant, Chroma, Milvus, Weaviate, FAISS, Elasticsearch, AstraDB, Redis, und mehr   |
| Datenbanken       | PostgreSQL (PGVector), MongoDB, MariaDB, IBM DB2, Redis, und mehr                             |
| Cloud-Plattformen | AWS (Boto3), Azure, Google Cloud Platform, und mehr                                           |
| Such-APIs         | Serper, Google Search, GitHub Search, GitLab Search, und mehr                                 |
| Embedding-Modelle | OpenAI Embeddings, Voyage AI, und mehr                                                        |

Erweiterungen werden von der Community entwickelt und gepflegt.

<p align="center">
  <img src="../assets/Ecosystem.png" alt="GraphBit Ecosystem - Stop Choosing, Start Orchestrating" style="max-width: 100%; height: auto;">
</p>


### Erstellen Ihres Ersten Agenten-Workflows mit GraphBit

<div align="center">
  <a href="https://www.youtube.com/watch?v=gKvkMc2qZcA">
    <img src="https://img.youtube.com/vi/gKvkMc2qZcA/maxresdefault.jpg" alt="Making Agent Workflow by GraphBit" style="max-width: 600px; height: auto;">
  </a>
  <p><em>Sehen Sie sich das Tutorial zur Erstellung eines Agenten-Workflows mit GraphBit an</em></p>
</div>

## Zu GraphBit Beitragen

Wir begrüßen Beiträge. Um zu beginnen, siehe bitte die [Contributing](CONTRIBUTING.md)-Datei für Entwicklungseinrichtung und Richtlinien.

GraphBit wird von einer wunderbaren Community von Forschern und Ingenieuren aufgebaut.

<a href="https://github.com/Infinitibit/graphbit/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Infinitibit/graphbit" />
</a>

## Sicherheit

Wenn Sie eine Sicherheitslücke entdecken, melden Sie diese bitte verantwortungsvoll über GitHub Security oder per E-Mail, anstatt ein öffentliches Issue zu erstellen.

Für detaillierte Meldeverfahren und Reaktionszeiten siehe unsere [Security Policy](SECURITY.md).

## Lizenz

GraphBit ist unter einem dreistufigen Modell lizenziert: **Modell A (Kostenlose Nutzung)** für Einzelpersonen, akademische Einrichtungen und kleine Teams (bis zu 10 Mitarbeiter/Benutzer), **Modell B (Kostenlose Testversion)** für 30-tägige Evaluierung, und **Modell C (Enterprise)** für kommerzielle/Produktionsnutzung. Weiterverbreitung ist unter allen Modellen ohne explizite Enterprise-Lizenz verboten.

Für vollständige Geschäftsbedingungen siehe [Full License](LICENSE.md).

Copyright © 2023–2026 InfinitiBit GmbH. All rights reserved.

---

**Hinweis**: Diese Übersetzung wird von der Community gepflegt. Wenn Sie Fehler finden oder die Übersetzung verbessern möchten, reichen Sie bitte einen Pull Request ein.

