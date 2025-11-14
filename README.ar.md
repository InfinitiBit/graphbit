<div align="center">

# GraphBit - إطار عمل الوكلاء عالي الأداء (العربية)

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

**سير عمل وكلاء الذكاء الاصطناعي الآمنة من حيث النوع مع أداء Rust**

</div>

---

🚧 **الترجمة قيد التقدم** - يتم ترجمة هذا المستند من الإنجليزية.

📖 **[Read in English](README.md)** | **[اقرأ بالإنجليزية](README.md)**

---

**اقرأ بلغات أخرى**: [🇨🇳 简体中文](README.zh-CN.md) | [🇨🇳 繁體中文](README.zh-TW.md) | [🇪🇸 Español](README.es.md) | [🇫🇷 Français](README.fr.md) | [🇩🇪 Deutsch](README.de.md) | [🇯🇵 日本語](README.ja.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇮🇹 Italiano](README.it.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

## حول GraphBit

GraphBit هو إطار عمل ذكاء اصطناعي مفتوح المصدر للمطورين الذين يحتاجون إلى تنفيذ حتمي ومتزامن ومنخفض التكلفة.

## لماذا GraphBit؟

الكفاءة تحدد من يمكنه التوسع. تم بناء GraphBit للمطورين الذين يحتاجون إلى تنفيذ ذكاء اصطناعي حتمي ومتزامن وفائق الكفاءة بدون تكاليف إضافية.

تم بناؤه بنواة Rust وطبقة Python بسيطة، يوفر GraphBit استخدامًا أقل بـ 68 مرة لوحدة المعالجة المركزية وبصمة ذاكرة أقل بـ 140 مرة من الأطر الأخرى، مع الحفاظ على إنتاجية مساوية أو أعلى.

يدعم سير عمل متعدد الوكلاء يعمل بالتوازي، ويحافظ على الذاكرة عبر الخطوات، ويتعافى ذاتيًا من الأعطال، ويضمن موثوقية 100٪ للمهام. تم بناء GraphBit لأحمال العمل الإنتاجية، من أنظمة الذكاء الاصطناعي للمؤسسات إلى عمليات النشر على الحافة منخفضة الموارد.

## الميزات الرئيسية

- **اختيار الأدوات** - تختار نماذج اللغة الكبيرة الأدوات بذكاء بناءً على الأوصاف
- **أمان النوع** - كتابة قوية عبر كل طبقة تنفيذ
- **الموثوقية** - قواطع الدوائر، وسياسات إعادة المحاولة، ومعالجة الأخطاء والتعافي من الأعطال
- **دعم متعدد LLM** - OpenAI، Azure OpenAI، Anthropic، OpenRouter، DeepSeek، Replicate، Ollama، TogetherAI والمزيد
- **إدارة الموارد** - ضوابط التزامن وتحسين الذاكرة
- **قابلية المراقبة** - تتبع مدمج، وسجلات منظمة، ومقاييس الأداء

## البدء السريع

### التثبيت

يوصى باستخدام بيئة افتراضية.

```bash
pip install graphbit
```

### إعداد البيئة

إنشاء ملف `.env`:

```env
OPENAI_API_KEY=your_api_key_here
```

### مثال أساسي

```python
from graphbit import Agent

# إنشاء وكيل
agent = Agent(
    name="assistant",
    model="gpt-4",
    instructions="You are a helpful assistant."
)

# تشغيل الوكيل
result = agent.run("Hello, GraphBit!")
print(result)
```

## التوثيق

للحصول على التوثيق الكامل، قم بزيارة: [https://docs.graphbit.ai/](https://docs.graphbit.ai/)

## المساهمة

نرحب بالمساهمات! راجع ملف [Contributing](CONTRIBUTING.md) للحصول على إعداد التطوير والإرشادات.

## الأمان

إذا اكتشفت ثغرة أمنية، يرجى الإبلاغ عنها بمسؤولية عبر GitHub Security أو البريد الإلكتروني بدلاً من إنشاء مشكلة عامة.

للحصول على إجراءات الإبلاغ التفصيلية والجداول الزمنية للاستجابة، راجع [Security Policy](SECURITY.md).

## الترخيص

GraphBit مرخص بموجب نموذج ثلاثي المستويات: **النموذج A (استخدام مجاني)** للأفراد والمؤسسات الأكاديمية والفرق الصغيرة (حتى 10 موظفين/مستخدمين)، **النموذج B (تجربة مجانية)** للتقييم لمدة 30 يومًا، و**النموذج C (المؤسسات)** للاستخدام التجاري/الإنتاجي. إعادة التوزيع محظورة بموجب جميع النماذج بدون ترخيص مؤسسي صريح.

للحصول على الشروط والأحكام الكاملة، راجع [Full License](LICENSE.md).

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.

---

**ملاحظة**: يتم صيانة هذه الترجمة من قبل المجتمع. إذا وجدت أي أخطاء أو ترغب في تحسين الترجمة، يرجى تقديم Pull Request.

