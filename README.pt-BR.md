<div align="center">

# GraphBit - Framework Agêntico de Alto Desempenho (Português)

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

**Fluxos de Trabalho de Agentes IA com Segurança de Tipos e Desempenho Rust**

</div>

---

🚧 **Tradução em andamento** - Este documento está sendo traduzido do inglês.

📖 **[Read in English](README.md)** | **[Ler em inglês](README.md)**

---

**Ler em outros idiomas**: [🇨🇳 简体中文](README.zh-CN.md) | [🇨🇳 繁體中文](README.zh-TW.md) | [🇪🇸 Español](README.es.md) | [🇫🇷 Français](README.fr.md) | [🇩🇪 Deutsch](README.de.md) | [🇯🇵 日本語](README.ja.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇮🇹 Italiano](README.it.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## Sobre o GraphBit

GraphBit é um framework de IA agêntico de código aberto para desenvolvedores que precisam de execução determinística, concorrente e de baixa sobrecarga.

## Por que GraphBit?

A eficiência decide quem escala. GraphBit foi construído para desenvolvedores que precisam de execução de IA determinística, concorrente e ultra-eficiente sem sobrecarga.

Construído com um núcleo Rust e uma camada Python mínima, GraphBit oferece até 68× menos uso de CPU e 140× menos pegada de memória do que outros frameworks, mantendo throughput igual ou superior.

Ele alimenta fluxos de trabalho multi-agente que executam em paralelo, persistem memória entre etapas, se auto-recuperam de falhas e garantem 100% de confiabilidade de tarefas. GraphBit foi construído para cargas de trabalho de produção, desde sistemas de IA empresariais até implantações edge com recursos limitados.

## Recursos Principais

- **Seleção de Ferramentas** - LLMs escolhem ferramentas inteligentemente com base em descrições
- **Segurança de Tipos** - Tipagem forte em cada camada de execução
- **Confiabilidade** - Disjuntores, políticas de retry, tratamento de erros e recuperação de falhas
- **Suporte Multi-LLM** - OpenAI, Azure OpenAI, Anthropic, OpenRouter, DeepSeek, Replicate, Ollama, TogetherAI e mais
- **Gerenciamento de Recursos** - Controles de concorrência e otimização de memória
- **Observabilidade** - Rastreamento integrado, logs estruturados e métricas de desempenho

## Início Rápido

### Instalação

Recomenda-se usar um ambiente virtual.

```bash
pip install graphbit
```

### Configuração do Ambiente

Criar arquivo `.env`:

```env
OPENAI_API_KEY=your_api_key_here
```

### Exemplo Básico

```python
from graphbit import Agent

# Criar agente
agent = Agent(
    name="assistant",
    model="gpt-4",
    instructions="You are a helpful assistant."
)

# Executar agente
result = agent.run("Hello, GraphBit!")
print(result)
```

## Documentação

Para documentação completa, visite: [https://docs.graphbit.ai/](https://docs.graphbit.ai/)

## Contribuir

Damos as boas-vindas a contribuições! Consulte o arquivo [Contributing](CONTRIBUTING.md) para configuração de desenvolvimento e diretrizes.

## Segurança

Se você descobrir uma vulnerabilidade de segurança, relate-a responsavelmente através do GitHub Security ou por e-mail em vez de criar um problema público.

Para procedimentos detalhados de relatório e prazos de resposta, consulte nossa [Security Policy](SECURITY.md).

## Licença

GraphBit é licenciado sob um modelo de três níveis: **Modelo A (Uso Gratuito)** para indivíduos, instituições acadêmicas e pequenas equipes (até 10 funcionários/usuários), **Modelo B (Teste Gratuito)** para avaliação de 30 dias, e **Modelo C (Enterprise)** para uso comercial/produção. A redistribuição é proibida sob todos os modelos sem uma Licença Enterprise explícita.

Para termos e condições completos, consulte a [Full License](LICENSE.md).

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.

---

**Nota**: Esta tradução é mantida pela comunidade. Se você encontrar erros ou desejar melhorar a tradução, envie um Pull Request.

