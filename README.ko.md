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

</p>
<p align="center">
    <a href="https://www.youtube.com/@graphbitAI"><img src="https://img.shields.io/badge/YouTube-FF0000?logo=youtube&logoColor=white" alt="YouTube"></a>
    <a href="https://x.com/graphbit_ai"><img src="https://img.shields.io/badge/X-000000?logo=x&logoColor=white" alt="X"></a>
    <a href="https://discord.com/invite/huVJwkyu"><img src="https://img.shields.io/badge/Discord-7289da?logo=discord&logoColor=white" alt="Discord"></a>
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

## 벤치마크

GraphBit은 이론적 주장이 아닌 측정된 결과를 위해 대규모 효율성을 위해 구축되었습니다.

우리의 내부 벤치마크 제품군은 동일한 워크로드에서 GraphBit을 주요 Python 기반 에이전트 프레임워크와 비교했습니다.

| 메트릭              | GraphBit        | 다른 프레임워크  | 이득                     |
|:--------------------|:---------------:|:----------------:|:-------------------------|
| CPU 사용량          | 1.0× 기준       | 68.3× 더 높음    | ~68× CPU                 |
| 메모리 풋프린트     | 1.0× 기준       | 140× 더 높음     | ~140× 메모리             |
| 실행 속도           | ≈ 동등 / 더 빠름 | —               | 일관된 처리량            |
| 결정성              | 100% 성공       | 가변적           | 보장된 신뢰성            |

GraphBit은 LLM 호출, 도구 호출 및 다중 에이전트 체인 전반에 걸쳐 일관되게 프로덕션 급 효율성을 제공합니다.

### 벤치마크 데모

<div align="center">
  <a href="https://www.youtube.com/watch?v=MaCl5oENeAY">
    <img src="https://img.youtube.com/vi/MaCl5oENeAY/maxresdefault.jpg" alt="GraphBit Benchmark Demo" style="max-width: 100%; height: auto;">
  </a>
  <p><em>GraphBit 벤치마크 데모 보기</em></p>
</div>

## GraphBit을 사용해야 하는 경우

다음이 필요한 경우 GraphBit을 선택하세요:

- 부하 하에서 무너지지 않는 프로덕션 급 다중 에이전트 시스템
- 타입 안전 실행 및 재현 가능한 출력
- 하이브리드 또는 스트리밍 AI 애플리케이션을 위한 실시간 오케스트레이션
- Rust 수준의 효율성과 Python 수준의 인체공학

프로토타입을 넘어 확장하거나 런타임 결정성을 중요하게 생각한다면 GraphBit이 적합합니다.

## 빠른 시작

### 설치

가상 환경 사용을 권장합니다.

```bash
pip install graphbit
```

### 빠른 시작 비디오 튜토리얼

<div align="center">
  <a href="https://youtu.be/ti0wbHFKKFM?si=hnxi-1W823z5I_zs">
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 100%; height: auto;">
  </a>
  <p><em>PyPI를 통한 GraphBit 설치 | 전체 예제 및 실행 가이드 튜토리얼 보기</em></p>
</div>


### 환경 설정

프로젝트에서 사용할 API 키를 설정합니다:
```bash
# OpenAI (선택 사항 – OpenAI 모델을 사용하는 경우 필수)
export OPENAI_API_KEY=your_openai_api_key_here

# Anthropic (선택 사항 – Anthropic 모델을 사용하는 경우 필수)
export ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

> **보안 참고사항**: API 키를 버전 관리에 커밋하지 마세요. 항상 환경 변수 또는 안전한 비밀 관리를 사용하세요.

### 기본 사용법
```python
import os

from graphbit import LlmConfig, Executor, Workflow, Node, tool

# 초기화 및 구성
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# 실행기 생성
executor = Executor(config)

# LLM 선택을 위한 명확한 설명이 있는 도구 생성
@tool(_description="모든 도시의 현재 날씨 정보 가져오기")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}

@tool(_description="수학 계산을 수행하고 결과 반환")
def calculate(expression: str) -> str:
    return f"Result: {eval(expression)}"

# 워크플로우 구축
workflow = Workflow("Analysis Pipeline")

# 에이전트 노드 생성
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

# 연결 및 실행
id1 = workflow.add_node(smart_agent)
id2 = workflow.add_node(processor)
workflow.connect(id1, id2)

result = executor.execute(workflow)
print(f"Workflow completed: {result.is_success()}")
print("\nSmart Agent Output: \n", result.get_node_output("Smart Agent"))
print("\nData Processor Output: \n", result.get_node_output("Data Processor"))
```

## 문서

전체 문서는 [https://docs.graphbit.ai/](https://docs.graphbit.ai/)를 참조하세요.


### GraphBit으로 첫 번째 에이전트 워크플로우 구축하기

<div align="center">
  <a href="https://www.youtube.com/watch?v=gKvkMc2qZcA">
    <img src="https://img.youtube.com/vi/gKvkMc2qZcA/maxresdefault.jpg" alt="Making Agent Workflow by GraphBit" style="max-width: 100%; height: auto;">
  </a>
  <p><em>GraphBit으로 에이전트 워크플로우 만들기 튜토리얼 보기</em></p>
</div>

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

