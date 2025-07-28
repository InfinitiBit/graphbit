# Marketing Email Extraction Agent

This document explains how to build and use an **AI-powered Marketing Agent** that:
- Uses **Google Search API** to find websites relevant to a query.
- Crawls these websites to extract email addresses.
- Generates a summary using **Graphbit + OpenAI LLM**.

---

## **Who Is This For?**
- **Marketers**: Quickly build lead lists.
- **Researchers**: Automate contact information gathering.
- **Developers**: Learn how to integrate APIs and AI agents.

---

## **Prerequisites**
1. **Python 3.9+**
2. **Google API Key** & **Custom Search Engine (CSE) ID**
   - Get from [Google Cloud Console](https://console.cloud.google.com/apis/credentials).
   - Create a Custom Search Engine (CSE) at [Google CSE](https://cse.google.com/cse/all).
3. **OpenAI API Key** (for AI summaries)
   - Get from [OpenAI](https://platform.openai.com/).
4. **Basic Python Knowledge**

---

## **Technologies Used**
- **Python** (for scripting)
- **Graphbit** (workflow orchestration)
- **Google Custom Search API** (for search)
- **Regex** (for email extraction)
- **dotenv** (for managing environment variables)

