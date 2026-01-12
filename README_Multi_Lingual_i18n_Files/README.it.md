<div align="center">

# GraphBit - Framework Agentico ad Alte Prestazioni (Italiano)

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

**Workflow di Agenti IA Type-Safe con Prestazioni Rust**

</div>

---

🚧 **Traduzione in corso** - Questo documento è in fase di traduzione dall'inglese.

📖 **[Read in English](../README.md)** | **[Leggi in inglese](../README.md)**

---

**Leggi in altre lingue**: [🇨🇳 简体中文](README.zh-CN.md) | [🇨🇳 繁體中文](README.zh-TW.md) | [🇪🇸 Español](README.es.md) | [🇫🇷 Français](README.fr.md) | [🇩🇪 Deutsch](README.de.md) | [🇯🇵 日本語](README.ja.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## Informazioni su GraphBit

GraphBit è un framework IA agentico a sorgente disponibile per sviluppatori che necessitano di esecuzione deterministica, concorrente e a basso overhead.

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

## Benchmark

GraphBit è stato costruito per l'efficienza su larga scala, non affermazioni teoriche, ma risultati misurati.

La nostra suite di benchmark interna ha confrontato GraphBit con i principali framework di agenti basati su Python su carichi di lavoro identici.

| Metrica             | GraphBit        | Altri Framework  | Guadagno                 |
|:--------------------|:---------------:|:----------------:|:-------------------------|
| Utilizzo CPU        | 1.0× base       | 68.3× superiore  | ~68× CPU                 |
| Impronta Memoria    | 1.0× base       | 140× superiore   | ~140× Memoria            |
| Velocità Esecuzione | ≈ uguale / più veloce | —            | Throughput coerente      |
| Determinismo        | 100% successo   | Variabile        | Affidabilità garantita   |

GraphBit offre costantemente efficienza di livello produzione per chiamate LLM, invocazioni di strumenti e catene multi-agente.

### Demo Benchmark

<div align="center">
  <a href="https://www.youtube.com/watch?v=MaCl5oENeAY">
    <img src="https://img.youtube.com/vi/MaCl5oENeAY/maxresdefault.jpg" alt="GraphBit Benchmark Demo" style="max-width: 600px; height: auto;">
  </a>
  <p><em>Guarda la Demo Benchmark di GraphBit</em></p>
</div>

## Quando Usare GraphBit

Scegli GraphBit se hai bisogno di:

- Sistemi multi-agente di livello produzione che non crollano sotto carico
- Esecuzione type-safe e output riproducibili
- Orchestrazione in tempo reale per applicazioni IA ibride o in streaming
- Efficienza a livello Rust con ergonomia a livello Python

Se stai scalando oltre i prototipi o ti importa del determinismo runtime, GraphBit è per te.

## Avvio Rapido

### Installazione

Si consiglia di utilizzare un ambiente virtuale.

```bash
pip install graphbit
```

### Tutorial Video di Avvio Rapido

<div align="center">
  <a href="https://youtu.be/ti0wbHFKKFM?si=hnxi-1W823z5I_zs">
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 600px; height: auto;">
  </a>
  <p><em>Guarda il tutorial di Installazione di GraphBit tramite PyPI | Guida Completa all'Esempio e all'Esecuzione</em></p>
</div>


### Configurazione dell'Ambiente

Configurare le chiavi API che si desidera utilizzare nel progetto:
```bash
# OpenAI (opzionale – richiesto se si utilizzano modelli OpenAI)
export OPENAI_API_KEY=your_openai_api_key_here

# Anthropic (opzionale – richiesto se si utilizzano modelli Anthropic)
export ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

> **Nota sulla Sicurezza**: Non eseguire mai il commit delle chiavi API nel controllo di versione. Utilizzare sempre variabili d'ambiente o gestione sicura dei segreti.

### Utilizzo di Base
```python
import os

from graphbit import LlmConfig, Executor, Workflow, Node, tool

# Inizializzare e configurare
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Creare l'esecutore
executor = Executor(config)

# Creare strumenti con descrizioni chiare per la selezione del LLM
@tool(_description="Ottenere informazioni meteorologiche attuali per qualsiasi città")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}

@tool(_description="Eseguire calcoli matematici e restituire risultati")
def calculate(expression: str) -> str:
    return f"Result: {eval(expression)}"

# Costruire il flusso di lavoro
workflow = Workflow("Analysis Pipeline")

# Creare nodi agente
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

# Connettere ed eseguire
id1 = workflow.add_node(smart_agent)
id2 = workflow.add_node(processor)
workflow.connect(id1, id2)

result = executor.execute(workflow)
print(f"Workflow completed: {result.is_success()}")
print("\nSmart Agent Output: \n", result.get_node_output("Smart Agent"))
print("\nData Processor Output: \n", result.get_node_output("Data Processor"))
```

## Osservabilità e Tracciamento

GraphBit Tracer cattura e monitora le chiamate LLM e i flussi di lavoro AI con configurazione minima. Avvolge i client LLM GraphBit e gli esecutori di flusso di lavoro per tracciare prompt, risposte, utilizzo dei token, latenza ed errori senza modificare il codice.

<div align="center">
  <a href="https://www.youtube.com/watch?v=nzwrxSiRl2U">
    <img src="https://img.youtube.com/vi/nzwrxSiRl2U/maxresdefault.jpg" alt="GraphBit Observability & Tracing" style="max-width: 600px; height: auto;">
  </a>
  <p><em>Guarda il tutorial su Osservabilità e Tracciamento di GraphBit</em></p>
</div>

## Architettura di Alto Livello

<p align="center">
  <img src="../assets/architecture.svg" height="250" alt="GraphBit Architecture">
</p>

Design a tre livelli per affidabilità e prestazioni:
- **Nucleo Rust** - Motore di flusso di lavoro, agenti e fornitori LLM
- **Livello di Orchestrazione** - Gestione ed esecuzione dei progetti
- **API Python** - Binding PyO3 con supporto asincrono

## Integrazioni API Python

GraphBit fornisce un'API Python ricca per costruire e integrare flussi di lavoro agentici:

- **Client LLM** - Integrazioni LLM multi-provider (OpenAI, Anthropic, Azure e altro)
- **Flussi di Lavoro** - Definire e gestire grafi di flusso di lavoro multi-agente con gestione dello stato
- **Nodi** - Nodi agente, nodi strumento e componenti di flusso di lavoro personalizzati
- **Esecutori** - Motore di esecuzione del flusso di lavoro con gestione della configurazione
- **Sistema di Strumenti** - Decoratori di funzioni, registro e framework di esecuzione per strumenti agente
- **Risultati del Flusso di Lavoro** - Risultati di esecuzione con metadati, timing e accesso all'output
- **Embeddings** - Embeddings vettoriali per ricerca semantica e recupero
- **Contesto del Flusso di Lavoro** - Stato condiviso e variabili attraverso l'esecuzione del flusso di lavoro
- **Caricatori di Documenti** - Caricare e analizzare documenti da più formati (PDF, DOCX, TXT, JSON, CSV, XML, HTML)
- **Divisori di Testo** - Dividere documenti in blocchi (carattere, token, frase, ricorsivo)

Per l'elenco completo di classi, metodi ed esempi di utilizzo, consulta il [Riferimento API Python](docs/api-reference/python-api.md).

## Documentazione

Per la documentazione completa, visitare: [https://docs.graphbit.ai/](https://docs.graphbit.ai/)

## Ecosistema ed Estensioni

L'architettura modulare di GraphBit supporta integrazioni esterne:

| Categoria         | Esempi                                                                                        |
|:------------------|:----------------------------------------------------------------------------------------------|
| Provider LLM      | OpenAI, Anthropic, Azure OpenAI, DeepSeek, Together, Ollama, OpenRouter, Fireworks, Mistral AI, Replicate, Perplexity, HuggingFace, AI21, Bytedance, xAI, e altro |
| Archivi Vettoriali | Pinecone, Qdrant, Chroma, Milvus, Weaviate, FAISS, Elasticsearch, AstraDB, Redis, e altro   |
| Database          | PostgreSQL (PGVector), MongoDB, MariaDB, IBM DB2, Redis, e altro                              |
| Piattaforme Cloud | AWS (Boto3), Azure, Google Cloud Platform, e altro                                            |
| API di Ricerca    | Serper, Google Search, GitHub Search, GitLab Search, e altro                                  |
| Modelli di Embedding | OpenAI Embeddings, Voyage AI, e altro                                                      |

Le estensioni sono sviluppate e mantenute dalla comunità.

<p align="center">
  <img src="../assets/Ecosystem.png" alt="GraphBit Ecosystem - Stop Choosing, Start Orchestrating" style="max-width: 100%; height: auto;">
</p>


### Costruire il Tuo Primo Flusso di Lavoro dell'Agente con GraphBit

<div align="center">
  <a href="https://www.youtube.com/watch?v=gKvkMc2qZcA">
    <img src="https://img.youtube.com/vi/gKvkMc2qZcA/maxresdefault.jpg" alt="Making Agent Workflow by GraphBit" style="max-width: 600px; height: auto;">
  </a>
  <p><em>Guarda il tutorial di Creazione del Flusso di Lavoro dell'Agente con GraphBit</em></p>
</div>

## Contribuire a GraphBit

Accogliamo contributi. Per iniziare, consulta il file [Contributing](CONTRIBUTING.md) per la configurazione dello sviluppo e le linee guida.

GraphBit è costruito da una meravigliosa comunità di ricercatori e ingegneri.

<a href="https://github.com/Infinitibit/graphbit/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Infinitibit/graphbit" />
</a>

## Sicurezza

Se scopri una vulnerabilità di sicurezza, segnalala responsabilmente tramite GitHub Security o email invece di creare un problema pubblico.

Per procedure di segnalazione dettagliate e tempistiche di risposta, consultare la nostra [Security Policy](SECURITY.md).

## Licenza

GraphBit è concesso in licenza secondo un modello a tre livelli: **Modello A (Uso Gratuito)** per individui, istituzioni accademiche e piccoli team (fino a 10 dipendenti/utenti), **Modello B (Prova Gratuita)** per valutazione di 30 giorni, e **Modello C (Enterprise)** per uso commerciale/produzione. La ridistribuzione è vietata sotto tutti i modelli senza una Licenza Enterprise esplicita.

Per termini e condizioni completi, consultare la [Full License](LICENSE.md).

Copyright © 2023–2026 InfinitiBit GmbH. All rights reserved.

---

**Nota**: Questa traduzione è mantenuta dalla comunità. Se trovi errori o desideri migliorare la traduzione, invia una Pull Request.

