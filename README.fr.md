<div align="center">

# GraphBit - Framework Agentique Haute Performance (Français)

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

**Workflows d'Agents IA Type-Safe avec Performance Rust**

</div>

---

🚧 **Traduction en cours** - Ce document est en cours de traduction depuis l'anglais.

📖 **[Read in English](README.md)** | **[Lire en anglais](README.md)**

---

**Lire dans d'autres langues**: [🇨🇳 简体中文](README.zh-CN.md) | [🇨🇳 繁體中文](README.zh-TW.md) | [🇪🇸 Español](README.es.md) | [🇩🇪 Deutsch](README.de.md) | [🇯🇵 日本語](README.ja.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇮🇹 Italiano](README.it.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## À propos de GraphBit

GraphBit est un framework IA agentique open-source pour les développeurs qui ont besoin d'une exécution déterministe, concurrente et à faible surcharge.

## Pourquoi GraphBit ?

L'efficacité décide qui peut évoluer. GraphBit est conçu pour les développeurs qui ont besoin d'une exécution IA déterministe, concurrente et ultra-efficace sans surcharge.

Construit avec un noyau Rust et une couche Python minimale, GraphBit offre jusqu'à 68× moins d'utilisation CPU et 140× moins d'empreinte mémoire que d'autres frameworks, tout en maintenant un débit égal ou supérieur.

Il alimente des workflows multi-agents qui s'exécutent en parallèle, persistent la mémoire entre les étapes, se récupèrent automatiquement des pannes et garantissent 100% de fiabilité des tâches. GraphBit est conçu pour les charges de travail de production, des systèmes IA d'entreprise aux déploiements edge à ressources limitées.

## Fonctionnalités Principales

- **Sélection d'Outils** - Les LLM choisissent intelligemment les outils en fonction des descriptions
- **Sécurité des Types** - Typage fort à travers chaque couche d'exécution
- **Fiabilité** - Disjoncteurs, politiques de réessai, gestion des erreurs et récupération des pannes
- **Support Multi-LLM** - OpenAI, Azure OpenAI, Anthropic, OpenRouter, DeepSeek, Replicate, Ollama, TogetherAI et plus
- **Gestion des Ressources** - Contrôles de concurrence et optimisation de la mémoire
- **Observabilité** - Traçage intégré, logs structurés et métriques de performance

## Benchmark

GraphBit a été conçu pour l'efficacité à grande échelle, non pas des affirmations théoriques, mais des résultats mesurés.

Notre suite de benchmarks interne a comparé GraphBit aux principaux frameworks d'agents basés sur Python sur des charges de travail identiques.

| Métrique            | GraphBit        | Autres Frameworks | Gain                     |
|:--------------------|:---------------:|:----------------:|:-------------------------|
| Utilisation CPU     | 1.0× base       | 68.3× supérieur  | ~68× CPU                 |
| Empreinte Mémoire   | 1.0× base       | 140× supérieur   | ~140× Mémoire            |
| Vitesse d'Exécution | ≈ égal / plus rapide | —            | Débit cohérent           |
| Déterminisme        | 100% succès     | Variable         | Fiabilité garantie       |

GraphBit offre systématiquement une efficacité de niveau production pour les appels LLM, les invocations d'outils et les chaînes multi-agents.

### Démo Benchmark

<div align="center">
  <a href="https://www.youtube.com/watch?v=MaCl5oENeAY">
    <img src="https://img.youtube.com/vi/MaCl5oENeAY/maxresdefault.jpg" alt="GraphBit Benchmark Demo" style="max-width: 100%; height: auto;">
  </a>
  <p><em>Regarder la Démo Benchmark de GraphBit</em></p>
</div>

## Quand Utiliser GraphBit

Choisissez GraphBit si vous avez besoin de :

- Systèmes multi-agents de niveau production qui ne s'effondrent pas sous la charge
- Exécution type-safe et sorties reproductibles
- Orchestration en temps réel pour applications IA hybrides ou en streaming
- Efficacité niveau Rust avec ergonomie niveau Python

Si vous dépassez les prototypes ou si le déterminisme d'exécution vous importe, GraphBit est fait pour vous.

## Démarrage Rapide

### Installation

Il est recommandé d'utiliser un environnement virtuel.

```bash
pip install graphbit
```

### Tutoriel Vidéo de Démarrage Rapide

<div align="center">
  <a href="https://youtu.be/ti0wbHFKKFM?si=hnxi-1W823z5I_zs">
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 100%; height: auto;">
  </a>
  <p><em>Regardez le tutoriel d'Installation de GraphBit via PyPI | Guide Complet d'Exemple et d'Exécution</em></p>
</div>


### Configuration de l'Environnement

Configurez les clés API que vous souhaitez utiliser dans votre projet :
```bash
# OpenAI (optionnel – requis si vous utilisez des modèles OpenAI)
export OPENAI_API_KEY=your_openai_api_key_here

# Anthropic (optionnel – requis si vous utilisez des modèles Anthropic)
export ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

> **Note de Sécurité** : Ne validez jamais les clés API dans le contrôle de version. Utilisez toujours des variables d'environnement ou une gestion sécurisée des secrets.

### Utilisation de Base
```python
import os

from graphbit import LlmConfig, Executor, Workflow, Node, tool

# Initialiser et configurer
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Créer l'exécuteur
executor = Executor(config)

# Créer des outils avec des descriptions claires pour la sélection du LLM
@tool(_description="Obtenir les informations météorologiques actuelles pour n'importe quelle ville")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}

@tool(_description="Effectuer des calculs mathématiques et renvoyer les résultats")
def calculate(expression: str) -> str:
    return f"Result: {eval(expression)}"

# Construire le flux de travail
workflow = Workflow("Analysis Pipeline")

# Créer des nœuds d'agent
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

# Connecter et exécuter
id1 = workflow.add_node(smart_agent)
id2 = workflow.add_node(processor)
workflow.connect(id1, id2)

result = executor.execute(workflow)
print(f"Workflow completed: {result.is_success()}")
print("\nSmart Agent Output: \n", result.get_node_output("Smart Agent"))
print("\nData Processor Output: \n", result.get_node_output("Data Processor"))
```

## Observabilité et Traçage

GraphBit Tracer capture et surveille les appels LLM et les flux de travail IA avec une configuration minimale. Il enveloppe les clients LLM GraphBit et les exécuteurs de flux de travail pour tracer les prompts, les réponses, l'utilisation des tokens, la latence et les erreurs sans modifier votre code.

<div align="center">
  <a href="https://www.youtube.com/watch?v=nzwrxSiRl2U">
    <img src="https://img.youtube.com/vi/nzwrxSiRl2U/maxresdefault.jpg" alt="GraphBit Observability & Tracing" style="max-width: 100%; height: auto;">
  </a>
  <p><em>Regardez le tutoriel sur l'Observabilité et le Traçage de GraphBit</em></p>
</div>

## Architecture de Haut Niveau

<p align="center">
  <img src="assets/architecture.svg" height="250" alt="GraphBit Architecture">
</p>

Conception à trois niveaux pour la fiabilité et les performances :
- **Noyau Rust** - Moteur de flux de travail, agents et fournisseurs LLM
- **Couche d'Orchestration** - Gestion et exécution de projets
- **API Python** - Liaisons PyO3 avec support asynchrone

## Intégrations API Python

GraphBit fournit une API Python riche pour construire et intégrer des flux de travail agentiques :

- **Clients LLM** - Intégrations LLM multi-fournisseurs (OpenAI, Anthropic, Azure et plus)
- **Flux de Travail** - Définir et gérer des graphes de flux de travail multi-agents avec gestion d'état
- **Nœuds** - Nœuds d'agent, nœuds d'outils et composants de flux de travail personnalisés
- **Exécuteurs** - Moteur d'exécution de flux de travail avec gestion de configuration
- **Système d'Outils** - Décorateurs de fonctions, registre et framework d'exécution pour les outils d'agent
- **Résultats de Flux de Travail** - Résultats d'exécution avec métadonnées, timing et accès aux sorties
- **Embeddings** - Embeddings vectoriels pour la recherche sémantique et la récupération
- **Contexte de Flux de Travail** - État partagé et variables à travers l'exécution du flux de travail
- **Chargeurs de Documents** - Charger et analyser des documents de plusieurs formats (PDF, DOCX, TXT, JSON, CSV, XML, HTML)
- **Diviseurs de Texte** - Diviser les documents en morceaux (caractère, token, phrase, récursif)

Pour la liste complète des classes, méthodes et exemples d'utilisation, consultez la [Référence API Python](docs/api-reference/python-api.md).

## Documentation

Pour la documentation complète, visitez : [https://docs.graphbit.ai/](https://docs.graphbit.ai/)


### Construire Votre Premier Flux de Travail d'Agent avec GraphBit

<div align="center">
  <a href="https://www.youtube.com/watch?v=gKvkMc2qZcA">
    <img src="https://img.youtube.com/vi/gKvkMc2qZcA/maxresdefault.jpg" alt="Making Agent Workflow by GraphBit" style="max-width: 100%; height: auto;">
  </a>
  <p><em>Regardez le tutoriel de Création de Flux de Travail d'Agent avec GraphBit</em></p>
</div>

## Contribuer

Nous accueillons les contributions ! Consultez le fichier [Contributing](CONTRIBUTING.md) pour la configuration de développement et les directives.

## Sécurité

Si vous découvrez une vulnérabilité de sécurité, veuillez la signaler de manière responsable via GitHub Security ou par e-mail plutôt que de créer un problème public.

Pour les procédures de signalement détaillées et les délais de réponse, consultez notre [Security Policy](SECURITY.md).

## Licence

GraphBit est sous licence selon un modèle à trois niveaux : **Modèle A (Utilisation Gratuite)** pour les particuliers, les institutions académiques et les petites équipes (jusqu'à 10 employés/utilisateurs), **Modèle B (Essai Gratuit)** pour une évaluation de 30 jours, et **Modèle C (Entreprise)** pour une utilisation commerciale/production. La redistribution est interdite sous tous les modèles sans une Licence Entreprise explicite.

Pour les termes et conditions complets, consultez la [Full License](LICENSE.md).

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.

---

**Note** : Cette traduction est maintenue par la communauté. Si vous trouvez des erreurs ou souhaitez améliorer la traduction, veuillez soumettre une Pull Request.

