<div align="center">

# GraphBit - 고성능 에이전트 프레임워크 (한국어)

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
    <a href="https://discord.com/invite/huVJwkyu"><img src="https://img.shields.io/discord/1234567890?color=7289da&label=Discord&logo=discord&logoColor=white" alt="Discord"></a>
</p>
<p align="center">
    <a href="https://www.youtube.com/@graphbitAI"><img src="https://img.shields.io/badge/YouTube-FF0000?logo=youtube&logoColor=white" alt="YouTube"></a>
    <a href="https://x.com/graphbit_ai"><img src="https://img.shields.io/badge/X-000000?logo=x&logoColor=white" alt="X"></a>
    <a href="https://discord.com/invite/c4CsMq6F"><img src="https://img.shields.io/badge/Discord-7289da?logo=discord&logoColor=white" alt="Discord"></a>
    <a href="https://www.linkedin.com/showcase/graphbitai/"><img src="https://img.shields.io/badge/LinkedIn-0077B5?logo=linkedin&logoColor=white" alt="LinkedIn"></a>
</p>

**Rust 성능을 갖춘 타입 안전 AI 에이전트 워크플로우**

</div>

---

🚧 **번역 진행 중** - 이 문서는 영어에서 번역 중입니다.

📖 **[Read in English](README.md)** | **[영어로 읽기](README.md)**

---

**다른 언어로 읽기**: [🇨🇳 简体中文](README.zh-CN.md) | [🇨🇳 繁體中文](README.zh-TW.md) | [🇪🇸 Español](README.es.md) | [🇫🇷 Français](README.fr.md) | [🇩🇪 Deutsch](README.de.md) | [🇯🇵 日本語](README.ja.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇮🇹 Italiano](README.it.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## GraphBit 소개

GraphBit은 결정론적이고 동시적이며 낮은 오버헤드 실행이 필요한 개발자를 위한 오픈 소스 에이전트 AI 프레임워크입니다.

## 왜 GraphBit인가?

효율성이 확장성을 결정합니다. GraphBit은 오버헤드 없이 결정론적이고 동시적이며 초효율적인 AI 실행이 필요한 개발자를 위해 구축되었습니다.

Rust 코어와 최소한의 Python 레이어로 구축된 GraphBit은 다른 프레임워크에 비해 최대 68배 낮은 CPU 사용량과 140배 낮은 메모리 사용량을 제공하면서 동등하거나 더 높은 처리량을 유지합니다.

병렬로 실행되는 멀티 에이전트 워크플로우, 단계 간 메모리 지속성, 장애로부터의 자가 복구, 100% 작업 신뢰성을 보장합니다. GraphBit은 엔터프라이즈 AI 시스템부터 저리소스 엣지 배포까지 프로덕션 워크로드를 위해 구축되었습니다.

## 주요 기능

- **도구 선택** - LLM이 설명을 기반으로 도구를 지능적으로 선택
- **타입 안전성** - 모든 실행 레이어에서 강력한 타입 지정
- **신뢰성** - 서킷 브레이커, 재시도 정책, 오류 처리 및 장애 복구
- **멀티 LLM 지원** - OpenAI, Azure OpenAI, Anthropic, OpenRouter, DeepSeek, Replicate, Ollama, TogetherAI 등
- **리소스 관리** - 동시성 제어 및 메모리 최적화
- **관찰 가능성** - 내장 추적, 구조화된 로그 및 성능 메트릭

## 빠른 시작

### 설치

가상 환경 사용을 권장합니다.

```bash
pip install graphbit
```

### 환경 설정

`.env` 파일 생성:

```env
OPENAI_API_KEY=your_api_key_here
```

### 기본 예제

```python
from graphbit import Agent

# 에이전트 생성
agent = Agent(
    name="assistant",
    model="gpt-4",
    instructions="You are a helpful assistant."
)

# 에이전트 실행
result = agent.run("Hello, GraphBit!")
print(result)
```

## 문서

전체 문서는 [https://docs.graphbit.ai/](https://docs.graphbit.ai/)를 참조하세요.

## 기여

기여를 환영합니다! 개발 설정 및 가이드라인은 [Contributing](CONTRIBUTING.md) 파일을 참조하세요.

## 보안

보안 취약점을 발견한 경우 공개 이슈를 생성하는 대신 GitHub Security 또는 이메일을 통해 책임감 있게 보고해 주세요.

자세한 보고 절차 및 응답 일정은 [Security Policy](SECURITY.md)를 참조하세요.

## 라이선스

GraphBit은 3단계 모델로 라이선스가 부여됩니다: **모델 A(무료 사용)**는 개인, 학술 기관 및 소규모 팀(최대 10명의 직원/사용자)용, **모델 B(무료 평가판)**는 30일 평가용, **모델 C(엔터프라이즈)**는 상업/프로덕션 사용용입니다. 명시적인 엔터프라이즈 라이선스 없이는 모든 모델에서 재배포가 금지됩니다.

전체 이용 약관은 [Full License](LICENSE.md)를 참조하세요.

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.

---

**참고**: 이 번역은 커뮤니티에서 유지 관리합니다. 오류를 발견하거나 번역을 개선하고 싶다면 Pull Request를 제출해 주세요.

