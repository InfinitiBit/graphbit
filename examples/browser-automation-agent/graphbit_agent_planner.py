"""graphbit_agent_planner."""

import contextlib
import json
import logging
import os
import re
from collections import Counter

import openai
from playwright.sync_api import sync_playwright

from graphbit import Node, Workflow

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("GraphbitAgent")
openai.api_key = os.getenv("OPENAI_API_KEY")

""" Hybrid Selector Filter Settings """
UTILITY_CLASSES = {
    "container",
    "row",
    "col",
    "col-lg-6",
    "text-center",
    "align-center",
    "d-flex",
    "justify-content",
    "align-items",
    "p-0",
    "m-0",
    "clearfix",
    "flex",
    "flex-row",
    "flex-col",
    "flex-center",
    "main",
    "header",
    "footer",
}

CONTENT_TAGS = {"h1", "h2", "h3", "h4", "h5", "h6", "p", "span", "div", "a", "button", "input", "li", "ul", "ol"}
IMPORTANT_KEYWORDS = {"headline", "main", "lead", "price", "title", "hero", "news", "top", "feature", "breaking"}


def is_heading(tag):
    """Check if the tag is a heading (h1-h6)."""
    import re

    return re.match(r"h[1-6]$", tag)


def is_important_string(s):
    """Check if the string contains any important keywords."""
    s = (s or "").lower()
    return any(key in s for key in IMPORTANT_KEYWORDS)


def get_selectors_for_url_hybrid(url, max_elements=100):
    """Extract selectors from a webpage using Playwright with hybrid filtering."""
    selectors = []
    seen = set()
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=False)
        page = browser.new_page()
        try:
            page.goto(url, timeout=60000)
            page.wait_for_load_state("networkidle", timeout=60000)
        except Exception as e:
            browser.close()
            print(f"Page load error: {e}")
            return []
        elements = page.query_selector_all("body *")
        for el in elements:
            try:
                if not el.is_visible():
                    continue
                tag = el.evaluate("el => el.tagName") or ""
                tag = tag.lower()
                id_attr = el.get_attribute("id") or ""
                class_attr = el.get_attribute("class") or ""
                txt = (el.inner_text() or "").strip()
                if not txt:
                    txt = el.get_attribute("value") or el.get_attribute("alt") or el.get_attribute("title") or ""
                key = (id_attr, class_attr, tag, txt[:40])
                if key in seen:
                    continue
                seen.add(key)
                # Hybrid filtering
                if class_attr and any(c in UTILITY_CLASSES for c in class_attr.split()):
                    continue
                important = False
                important = is_heading(tag) or is_important_string(class_attr) or is_important_string(id_attr) or (tag in CONTENT_TAGS and txt and len(txt) > 20)

                if not important:
                    continue
                selectors.append({"id": id_attr if id_attr else None, "class": class_attr if class_attr else None, "tag": tag, "text": txt[:60]})
                if len(selectors) >= max_elements:
                    break
            except Exception as e:
                logger.warning(f"Exception processing element: {e}")
                continue
        browser.close()
    return selectors


def build_llm_prompt_advanced(user_goal, url, selectors, max_total=30, max_per_type=10):
    """Build a detailed prompt for the LLM planner with selector summaries."""
    id_counter = Counter(s["id"] for s in selectors if s.get("id"))
    class_counter = Counter()
    for s in selectors:
        if s.get("class"):
            for c in s["class"].split():
                class_counter[c] += 1
    tag_counter = Counter(s["tag"].lower() for s in selectors if s.get("tag"))

    rare_ids = [i for i, c in id_counter.items() if c == 1][:max_per_type]
    rare_classes = [c for c, n in class_counter.items() if n == 1][:max_per_type]
    tags = list(tag_counter.keys())[:max_per_type]

    def build_lines(key_list, key, sample_size):
        lines, used = [], set()
        for s in selectors:
            val = s.get(key)
            if val and val in key_list and val not in used:
                txt = s.get("text", "")
                line = f"- {key}: {val}"
                if txt:
                    line += f" | text: {txt[:60]}"
                lines.append(line)
                used.add(val)
            if len(lines) >= sample_size:
                break
        return lines

    id_lines = build_lines(rare_ids, "id", max_per_type)
    class_lines = build_lines(rare_classes, "class", max_per_type)
    tag_lines = build_lines(tags, "tag", max_per_type)

    summary = (
        f"User wants: {user_goal}\n"
        f"URL: {url}\n\n"
        "Summary of page elements (by frequency):\n"
        f"- {len(id_counter)} unique IDs, {len(class_counter)} classes, {len(tag_counter)} tag types\n"
        f"- Top unique/rare IDs: {rare_ids}\n"
        f"- Top unique/rare classes: {rare_classes}\n"
        f"- Main tags: {tags}\n\n"
        "Element samples for LLM planning:\n"
        "IDs:\n" + "\n".join(id_lines) + "\n\n"
        "Classes:\n" + "\n".join(class_lines) + "\n\n"
        "Tags:\n" + "\n".join(tag_lines) + "\n\n"
        "Use ONLY these selectors to generate the JSON browser automation plan. "
        "Valid actions: goto, wait_for, click, type, extract_text. "
        "If no selector matches, suggest a fallback plan or site. "
        "Output only a JSON array."
    )
    return summary


def extract_first_json_array(text):
    """Extract the first valid JSON array or object from text, even if surrounded by markdown/code fences or extra text."""
    # Try to extract using regex for array first
    match = re.search(r"(\[.*?\])", text, re.DOTALL)
    if match:
        with contextlib.suppress(Exception):
            return json.loads(match.group(1))
    # Try to extract using regex for object if array failed
    match = re.search(r"(\{.*\})", text, re.DOTALL)
    if match:
        with contextlib.suppress(Exception):
            return json.loads(match.group(1))
    # Fallback: Try raw (may raise error)
    return json.loads(text)


def get_plan_from_graphbit(prompt: str, url: str) -> tuple[list, list]:
    """Generate plan using Graphbit and LLM based on the provided prompt and URL."""
    logger.info(f"Gathering selectors for {url} ...")
    selectors = get_selectors_for_url_hybrid(url)
    if not selectors:
        logger.warning("No selectors found; page may not have loaded.")
    # planner_prompt = build_llm_prompt_advanced(prompt, url, selectors)
    planner_prompt = (
        build_llm_prompt_advanced(prompt, url, selectors)
        + """

        INSTRUCTIONS FOR SELECTOR CHOICE:
        - Prefer unique CSS selectors, IDs or classes (avoid generic tags like just 'p' or 'div').
        - If extracting text from a specific section (like featured article), target the specific container or heading nearby.
        - If multiple elements match, pick the one closest to the described content.
        - Always output valid JSON steps only.
        """
    )

    workflow = Workflow("llm-plan")
    node = Node.agent("Planner", planner_prompt)
    workflow.add_node(node)

    logger.info("Calling GPT-4 with planner prompt.")
    response = openai.chat.completions.create(
        model="gpt-4",
        messages=[
            {"role": "system", "content": "You are a browser automation planner. Convert tasks to JSON steps using only the provided elements/selectors."},
            {"role": "user", "content": planner_prompt},
        ],
        temperature=0,
    )
    content = response.choices[0].message.content
    logger.info("GPT-4 raw output:\n" + content)
    try:
        plan = extract_first_json_array(content)
    except Exception as e:
        logger.error(f"Failed to parse GPT-4 output: {e}")
        plan = []
    # Extra safety check
    if not isinstance(plan, list):
        logger.error("Parsed plan is not a list. Something is wrong with the LLM output.")
        plan = []
    return plan, selectors
