<div align="center">

# GraphBit - Framework Agentico ad Alte Prestazioni (Italiano)

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

**Workflow di Agenti IA Type-Safe con Prestazioni Rust**

</div>

---

🚧 **Traduzione in corso** - Questo documento è in fase di traduzione dall'inglese.

📖 **[Read in English](README.md)** | **[Leggi in inglese](README.md)**

---

**Leggi in altre lingue**: [🇨🇳 简体中文](README.zh-CN.md) | [🇨🇳 繁體中文](README.zh-TW.md) | [🇪🇸 Español](README.es.md) | [🇫🇷 Français](README.fr.md) | [🇩🇪 Deutsch](README.de.md) | [🇯🇵 日本語](README.ja.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## Informazioni su GraphBit

GraphBit è un framework IA agentico open-source per sviluppatori che necessitano di esecuzione deterministica, concorrente e a basso overhead.

## Perché GraphBit?

L'efficienza decide chi può scalare. GraphBit è costruito per sviluppatori che necessitano di esecuzione IA deterministica, concorrente e ultra-efficiente senza overhead.

Costruito con un core Rust e un layer Python minimale, GraphBit offre fino a 68× meno utilizzo CPU e 140× meno impronta di memoria rispetto ad altri framework, mantenendo un throughput uguale o superiore.

Alimenta workflow multi-agente che vengono eseguiti in parallelo, persistono la memoria tra i passaggi, si auto-recuperano dai guasti e garantiscono il 100% di affidabilità delle attività. GraphBit è costruito per carichi di lavoro di produzione, dai sistemi IA aziendali ai deployment edge a risorse limitate.

## Caratteristiche Principali

- **Selezione degli Strumenti** - Gli LLM scelgono intelligentemente gli strumenti in base alle descrizioni
- **Sicurezza dei Tipi** - Tipizzazione forte attraverso ogni livello di esecuzione
- **Affidabilità** - Circuit breaker, politiche di retry, gestione degli errori e recupero dai guasti
- **Supporto Multi-LLM** - OpenAI, Azure OpenAI, Anthropic, OpenRouter, DeepSeek, Replicate, Ollama, TogetherAI e altro
- **Gestione delle Risorse** - Controlli di concorrenza e ottimizzazione della memoria
- **Osservabilità** - Tracciamento integrato, log strutturati e metriche delle prestazioni

## Avvio Rapido

### Installazione

Si consiglia di utilizzare un ambiente virtuale.

```bash
pip install graphbit
```

### Configurazione dell'Ambiente

Creare un file `.env`:

```env
OPENAI_API_KEY=your_api_key_here
```

### Esempio Base

```python
from graphbit import Agent

# Creare un agente
agent = Agent(
    name="assistant",
    model="gpt-4",
    instructions="You are a helpful assistant."
)

# Eseguire l'agente
result = agent.run("Hello, GraphBit!")
print(result)
```

## Documentazione

Per la documentazione completa, visitare: [https://docs.graphbit.ai/](https://docs.graphbit.ai/)

## Contribuire

Accogliamo i contributi! Consultare il file [Contributing](CONTRIBUTING.md) per la configurazione dello sviluppo e le linee guida.

## Sicurezza

Se scopri una vulnerabilità di sicurezza, segnalala responsabilmente tramite GitHub Security o email invece di creare un problema pubblico.

Per procedure di segnalazione dettagliate e tempistiche di risposta, consultare la nostra [Security Policy](SECURITY.md).

## Licenza

GraphBit è concesso in licenza secondo un modello a tre livelli: **Modello A (Uso Gratuito)** per individui, istituzioni accademiche e piccoli team (fino a 10 dipendenti/utenti), **Modello B (Prova Gratuita)** per valutazione di 30 giorni, e **Modello C (Enterprise)** per uso commerciale/produzione. La ridistribuzione è vietata sotto tutti i modelli senza una Licenza Enterprise esplicita.

Per termini e condizioni completi, consultare la [Full License](LICENSE.md).

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.

---

**Nota**: Questa traduzione è mantenuta dalla comunità. Se trovi errori o desideri migliorare la traduzione, invia una Pull Request.

