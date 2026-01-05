<div align="center">

# GraphBit - Framework Agéntico de Alto Rendimiento (Español)

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

**Flujos de Trabajo de Agentes IA con Seguridad de Tipos y Rendimiento de Rust**

</div>

---

🚧 **Traducción en progreso** - Este documento está siendo traducido del inglés.

📖 **[Read in English](../README.md)** | **[Leer en inglés](../README.md)**

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
    <img src="https://img.youtube.com/vi/MaCl5oENeAY/maxresdefault.jpg" alt="GraphBit Benchmark Demo" style="max-width: 600px; height: auto;">
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
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 600px; height: auto;">
  </a>
  <p><em>Vea el tutorial de Instalación de GraphBit vía PyPI | Guía Completa de Ejemplo y Ejecución</em></p>
</div>


### Configuración del Entorno

Configure las claves API que desea usar en su proyecto:
```bash
# OpenAI (opcional – requerido si usa modelos OpenAI)
export OPENAI_API_KEY=your_openai_api_key_here

# Anthropic (opcional – requerido si usa modelos Anthropic)
export ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

> **Nota de Seguridad**: Nunca confirme claves API en el control de versiones. Siempre use variables de entorno o gestión segura de secretos.

### Uso Básico
```python
import os

from graphbit import LlmConfig, Executor, Workflow, Node, tool

# Inicializar y configurar
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Crear ejecutor
executor = Executor(config)

# Crear herramientas con descripciones claras para la selección del LLM
@tool(_description="Obtener información meteorológica actual para cualquier ciudad")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}

@tool(_description="Realizar cálculos matemáticos y devolver resultados")
def calculate(expression: str) -> str:
    return f"Result: {eval(expression)}"

# Construir flujo de trabajo
workflow = Workflow("Analysis Pipeline")

# Crear nodos de agente
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

# Conectar y ejecutar
id1 = workflow.add_node(smart_agent)
id2 = workflow.add_node(processor)
workflow.connect(id1, id2)

result = executor.execute(workflow)
print(f"Workflow completed: {result.is_success()}")
print("\nSmart Agent Output: \n", result.get_node_output("Smart Agent"))
print("\nData Processor Output: \n", result.get_node_output("Data Processor"))
```

## Observabilidad y Rastreo

GraphBit Tracer captura y monitorea llamadas LLM y flujos de trabajo de IA con configuración mínima. Envuelve los clientes LLM de GraphBit y los ejecutores de flujo de trabajo para rastrear prompts, respuestas, uso de tokens, latencia y errores sin cambiar su código.

<div align="center">
  <a href="https://www.youtube.com/watch?v=nzwrxSiRl2U">
    <img src="https://img.youtube.com/vi/nzwrxSiRl2U/maxresdefault.jpg" alt="GraphBit Observability & Tracing" style="max-width: 600px; height: auto;">
  </a>
  <p><em>Vea el tutorial de Observabilidad y Rastreo de GraphBit</em></p>
</div>

## Arquitectura de Alto Nivel

<p align="center">
  <img src="../assets/architecture.svg" height="250" alt="GraphBit Architecture">
</p>

Diseño de tres niveles para confiabilidad y rendimiento:
- **Núcleo Rust** - Motor de flujo de trabajo, agentes y proveedores LLM
- **Capa de Orquestación** - Gestión y ejecución de proyectos
- **API Python** - Enlaces PyO3 con soporte asíncrono

## Integraciones de API Python

GraphBit proporciona una API Python rica para construir e integrar flujos de trabajo agénticos:

- **Clientes LLM** - Integraciones LLM multiproveedores (OpenAI, Anthropic, Azure y más)
- **Flujos de Trabajo** - Definir y gestionar gráficos de flujo de trabajo multiagente con gestión de estado
- **Nodos** - Nodos de agente, nodos de herramientas y componentes de flujo de trabajo personalizados
- **Ejecutores** - Motor de ejecución de flujo de trabajo con gestión de configuración
- **Sistema de Herramientas** - Decoradores de funciones, registro y marco de ejecución para herramientas de agente
- **Resultados de Flujo de Trabajo** - Resultados de ejecución con metadatos, temporización y acceso a salidas
- **Embeddings** - Embeddings vectoriales para búsqueda semántica y recuperación
- **Contexto de Flujo de Trabajo** - Estado compartido y variables a través de la ejecución del flujo de trabajo
- **Cargadores de Documentos** - Cargar y analizar documentos de múltiples formatos (PDF, DOCX, TXT, JSON, CSV, XML, HTML)
- **Divisores de Texto** - Dividir documentos en fragmentos (carácter, token, oración, recursivo)

Para la lista completa de clases, métodos y ejemplos de uso, consulte la [Referencia de API Python](docs/api-reference/python-api.md).

## Documentación

Para documentación completa, visite: [https://docs.graphbit.ai/](https://docs.graphbit.ai/)

## Ecosistema y Extensiones

La arquitectura modular de GraphBit soporta integraciones externas:

| Categoría         | Ejemplos                                                                                      |
|:------------------|:----------------------------------------------------------------------------------------------|
| Proveedores LLM   | OpenAI, Anthropic, Azure OpenAI, DeepSeek, Together, Ollama, OpenRouter, Fireworks, Mistral AI, Replicate, Perplexity, HuggingFace, AI21, Bytedance, xAI, y más |
| Almacenes Vectoriales | Pinecone, Qdrant, Chroma, Milvus, Weaviate, FAISS, Elasticsearch, AstraDB, Redis, y más   |
| Bases de Datos    | PostgreSQL (PGVector), MongoDB, MariaDB, IBM DB2, Redis, y más                                |
| Plataformas Cloud | AWS (Boto3), Azure, Google Cloud Platform, y más                                              |
| APIs de Búsqueda  | Serper, Google Search, GitHub Search, GitLab Search, y más                                    |
| Modelos de Embeddings | OpenAI Embeddings, Voyage AI, y más                                                       |

Las extensiones son desarrolladas y mantenidas por la comunidad.

<p align="center">
  <img src="../assets/Ecosystem.png" alt="GraphBit Ecosystem - Stop Choosing, Start Orchestrating" style="max-width: 100%; height: auto;">
</p>


### Construyendo Su Primer Flujo de Trabajo de Agente con GraphBit

<div align="center">
  <a href="https://www.youtube.com/watch?v=gKvkMc2qZcA">
    <img src="https://img.youtube.com/vi/gKvkMc2qZcA/maxresdefault.jpg" alt="Making Agent Workflow by GraphBit" style="max-width: 600px; height: auto;">
  </a>
  <p><em>Vea el tutorial de Creación de Flujo de Trabajo de Agente con GraphBit</em></p>
</div>

## Contribuir a GraphBit

Damos la bienvenida a contribuciones. Para comenzar, consulte el archivo [Contributing](CONTRIBUTING.md) para configuración de desarrollo y directrices.

GraphBit es construido por una maravillosa comunidad de investigadores e ingenieros.

<a href="https://github.com/Infinitibit/graphbit/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Infinitibit/graphbit" />
</a>

## Seguridad

Si descubre una vulnerabilidad de seguridad, repórtela responsablemente a través de GitHub Security o por correo electrónico en lugar de crear un issue público.

Para procedimientos detallados de reporte y plazos de respuesta, consulte nuestra [Security Policy](SECURITY.md).

## Licencia

GraphBit está licenciado bajo un modelo de tres niveles: **Modelo A (Uso Gratuito)** para individuos, instituciones académicas y equipos pequeños (hasta 10 empleados/usuarios), **Modelo B (Prueba Gratuita)** para evaluación de 30 días, y **Modelo C (Empresarial)** para uso comercial/producción. La redistribución está prohibida bajo todos los modelos sin una Licencia Empresarial explícita.

Para términos y condiciones completos, consulte la [Full License](LICENSE.md).

Copyright © 2023–2026 InfinitiBit GmbH. All rights reserved.

---

**Nota**: Esta traducción es mantenida por la comunidad. Si encuentra algún error o desea mejorar la traducción, envíe un Pull Request.

