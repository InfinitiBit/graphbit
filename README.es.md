<div align="center">

# GraphBit - Framework Agéntico de Alto Rendimiento (Español)

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

**Flujos de Trabajo de Agentes IA con Seguridad de Tipos y Rendimiento de Rust**

</div>

---

🚧 **Traducción en progreso** - Este documento está siendo traducido del inglés.

📖 **[Read in English](README.md)** | **[Leer en inglés](README.md)**

---

**Leer en otros idiomas**: [🇨🇳 简体中文](README.zh-CN.md) | [🇨🇳 繁體中文](README.zh-TW.md) | [🇫🇷 Français](README.fr.md) | [🇩🇪 Deutsch](README.de.md) | [🇯🇵 日本語](README.ja.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇮🇹 Italiano](README.it.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## Acerca de GraphBit

GraphBit es un framework de IA agéntico de código abierto para desarrolladores que necesitan ejecución determinista, concurrente y de baja sobrecarga.

## ¿Por qué GraphBit?

La eficiencia decide quién escala. GraphBit está construido para desarrolladores que necesitan ejecución de IA determinista, concurrente y ultra-eficiente sin sobrecarga.

Construido con un núcleo Rust y una capa Python mínima, GraphBit ofrece hasta 68× menor uso de CPU y 140× menor huella de memoria que otros frameworks, manteniendo igual o mayor rendimiento.

Impulsa flujos de trabajo multi-agente que se ejecutan en paralelo, persisten memoria entre pasos, se auto-recuperan de fallos y garantizan 100% de fiabilidad en las tareas. GraphBit está construido para cargas de trabajo de producción, desde sistemas de IA empresariales hasta despliegues en edge con recursos limitados.

## Características Principales

- **Selección de Herramientas** - Los LLM eligen herramientas inteligentemente basándose en descripciones
- **Seguridad de Tipos** - Tipado fuerte en cada capa de ejecución
- **Fiabilidad** - Disyuntores, políticas de reintento, manejo de errores y recuperación de fallos
- **Soporte Multi-LLM** - OpenAI, Azure OpenAI, Anthropic, OpenRouter, DeepSeek, Replicate, Ollama, TogetherAI y más
- **Gestión de Recursos** - Controles de concurrencia y optimización de memoria
- **Observabilidad** - Trazado integrado, logs estructurados y métricas de rendimiento

## Benchmark

GraphBit fue construido para eficiencia a escala, no afirmaciones teóricas, sino resultados medidos.

Nuestro conjunto de pruebas interno comparó GraphBit con los principales frameworks de agentes basados en Python en cargas de trabajo idénticas.

| Métrica             | GraphBit        | Otros Frameworks | Ganancia                 |
|:--------------------|:---------------:|:----------------:|:-------------------------|
| Uso de CPU          | 1.0× base       | 68.3× mayor      | ~68× CPU                 |
| Huella de Memoria   | 1.0× base       | 140× mayor       | ~140× Memoria            |
| Velocidad de Ejecución | ≈ igual / más rápido | —         | Rendimiento consistente  |
| Determinismo        | 100% éxito      | Variable         | Fiabilidad garantizada   |

GraphBit ofrece consistentemente eficiencia de grado de producción en llamadas LLM, invocaciones de herramientas y cadenas multi-agente.

### Demo de Benchmark

<div align="center">
  <a href="https://www.youtube.com/watch?v=MaCl5oENeAY">
    <img src="https://img.youtube.com/vi/MaCl5oENeAY/maxresdefault.jpg" alt="GraphBit Benchmark Demo" style="max-width: 100%; height: auto;">
  </a>
  <p><em>Ver la Demo de Benchmark de GraphBit</em></p>
</div>

## Cuándo Usar GraphBit

Elija GraphBit si necesita:

- Sistemas multi-agente de grado de producción que no colapsen bajo carga
- Ejecución con seguridad de tipos y salidas reproducibles
- Orquestación en tiempo real para aplicaciones de IA híbridas o de streaming
- Eficiencia a nivel de Rust con ergonomía a nivel de Python

Si está escalando más allá de prototipos o le importa el determinismo en tiempo de ejecución, GraphBit es para usted.

## Inicio Rápido

### Instalación

Se recomienda usar un entorno virtual.

```bash
pip install graphbit
```

### Tutorial en Video de Inicio Rápido

<div align="center">
  <a href="https://youtu.be/ti0wbHFKKFM?si=hnxi-1W823z5I_zs">
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 100%; height: auto;">
  </a>
  <p><em>Vea el tutorial de Instalación de GraphBit vía PyPI | Guía Completa de Ejemplo y Ejecución</em></p>
</div>


### Configuración del Entorno

Crear archivo `.env`:

```env
OPENAI_API_KEY=your_api_key_here
```

### Ejemplo Básico

```python
from graphbit import Agent

# Crear agente
agent = Agent(
    name="assistant",
    model="gpt-4",
    instructions="You are a helpful assistant."
)

# Ejecutar agente
result = agent.run("Hello, GraphBit!")
print(result)
```

## Documentación

Para documentación completa, visite: [https://docs.graphbit.ai/](https://docs.graphbit.ai/)


### Construyendo Su Primer Flujo de Trabajo de Agente con GraphBit

<div align="center">
  <a href="https://www.youtube.com/watch?v=gKvkMc2qZcA">
    <img src="https://img.youtube.com/vi/gKvkMc2qZcA/maxresdefault.jpg" alt="Making Agent Workflow by GraphBit" style="max-width: 100%; height: auto;">
  </a>
  <p><em>Vea el tutorial de Creación de Flujo de Trabajo de Agente con GraphBit</em></p>
</div>

## Contribuir

¡Damos la bienvenida a contribuciones! Consulte el archivo [Contributing](CONTRIBUTING.md) para configuración de desarrollo y directrices.

## Seguridad

Si descubre una vulnerabilidad de seguridad, repórtela responsablemente a través de GitHub Security o por correo electrónico en lugar de crear un issue público.

Para procedimientos detallados de reporte y plazos de respuesta, consulte nuestra [Security Policy](SECURITY.md).

## Licencia

GraphBit está licenciado bajo un modelo de tres niveles: **Modelo A (Uso Gratuito)** para individuos, instituciones académicas y equipos pequeños (hasta 10 empleados/usuarios), **Modelo B (Prueba Gratuita)** para evaluación de 30 días, y **Modelo C (Empresarial)** para uso comercial/producción. La redistribución está prohibida bajo todos los modelos sin una Licencia Empresarial explícita.

Para términos y condiciones completos, consulte la [Full License](LICENSE.md).

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.

---

**Nota**: Esta traducción es mantenida por la comunidad. Si encuentra algún error o desea mejorar la traducción, envíe un Pull Request.

