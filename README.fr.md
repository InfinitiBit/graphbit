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

[![Build Status](https://img.shields.io/github/actions/workflow/status/InfinitiBit/graphbit/update-docs.yml?branch=main)](https://github.com/InfinitiBit/graphbit/actions/workflows/update-docs.yml)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/InfinitiBit/graphbit/blob/main/CONTRIBUTING.md)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Python Version](https://img.shields.io/badge/python-3.10--3.13-blue.svg)](https://www.python.org)

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

## Démarrage Rapide

### Installation

Il est recommandé d'utiliser un environnement virtuel.

```bash
pip install graphbit
```

### Configuration de l'Environnement

Créer un fichier `.env` :

```env
OPENAI_API_KEY=your_api_key_here
```

### Exemple de Base

```python
from graphbit import Agent

# Créer un agent
agent = Agent(
    name="assistant",
    model="gpt-4",
    instructions="You are a helpful assistant."
)

# Exécuter l'agent
result = agent.run("Hello, GraphBit!")
print(result)
```

## Documentation

Pour la documentation complète, visitez : [https://docs.graphbit.ai/](https://docs.graphbit.ai/)

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

