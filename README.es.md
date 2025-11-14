<div align="center">

# GraphBit - Framework Agéntico de Alto Rendimiento (Español)

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

## Inicio Rápido

### Instalación

Se recomienda usar un entorno virtual.

```bash
pip install graphbit
```

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

