<div align="center">

# GraphBit - Hochleistungs-Agenten-Framework (Deutsch)

<p align="center">
    <img src="assets/GraphBit_Final_GB_Github_GIF.gif" style="max-width: 100%; height: auto;" alt="Logo" />
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

**Typsichere KI-Agenten-Workflows mit Rust-Performance**

</div>

---

🚧 **Übersetzung in Arbeit** - Dieses Dokument wird gerade aus dem Englischen übersetzt.

📖 **[Read in English](README.md)** | **[Auf Englisch lesen](README.md)**

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

## Schnellstart

### Installation

Es wird empfohlen, eine virtuelle Umgebung zu verwenden.

```bash
pip install graphbit
```

### Umgebungseinrichtung

`.env`-Datei erstellen:

```env
OPENAI_API_KEY=your_api_key_here
```

### Grundlegendes Beispiel

```python
from graphbit import Agent

# Agent erstellen
agent = Agent(
    name="assistant",
    model="gpt-4",
    instructions="You are a helpful assistant."
)

# Agent ausführen
result = agent.run("Hello, GraphBit!")
print(result)
```

## Dokumentation

Für vollständige Dokumentation besuchen Sie: [https://docs.graphbit.ai/](https://docs.graphbit.ai/)

## Beitragen

Wir begrüßen Beiträge! Siehe [Contributing](CONTRIBUTING.md)-Datei für Entwicklungseinrichtung und Richtlinien.

## Sicherheit

Wenn Sie eine Sicherheitslücke entdecken, melden Sie diese bitte verantwortungsvoll über GitHub Security oder per E-Mail, anstatt ein öffentliches Issue zu erstellen.

Für detaillierte Meldeverfahren und Reaktionszeiten siehe unsere [Security Policy](SECURITY.md).

## Lizenz

GraphBit ist unter einem dreistufigen Modell lizenziert: **Modell A (Kostenlose Nutzung)** für Einzelpersonen, akademische Einrichtungen und kleine Teams (bis zu 10 Mitarbeiter/Benutzer), **Modell B (Kostenlose Testversion)** für 30-tägige Evaluierung, und **Modell C (Enterprise)** für kommerzielle/Produktionsnutzung. Weiterverbreitung ist unter allen Modellen ohne explizite Enterprise-Lizenz verboten.

Für vollständige Geschäftsbedingungen siehe [Full License](LICENSE.md).

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.

---

**Hinweis**: Diese Übersetzung wird von der Community gepflegt. Wenn Sie Fehler finden oder die Übersetzung verbessern möchten, reichen Sie bitte einen Pull Request ein.

