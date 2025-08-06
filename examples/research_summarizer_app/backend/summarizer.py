"""This is the summarizer code."""

import os
import re
from typing import Dict, List

from dotenv import load_dotenv
from PyPDF2 import PdfReader

from graphbit import CharacterSplitter
from graphbit import LLMClient as gb_lct
from graphbit import LLMConfig as gb_lcg

load_dotenv()

api_key = os.getenv("OPENAI_API_KEY")

llm_config = gb_lcg.openai(api_key, model="gpt-4o")
llm_client = gb_lct(llm_config)

# text_splitter_config = gb_tsc.openai(api_key, model="gpt-4o")
# text_splitter_client = gb_tsc(text_splitter_config)

splitter = CharacterSplitter(chunk_size=1000, chunk_overlap=200)  # Maximum characters per chunk  # Overlap between chunks


# SECTION HEADERS in display order
SECTION_HEADERS = [r"abstract", r"introduction", r"background", r"related work", r"methods", r"methodology", r"experiment", r"results", r"discussion", r"conclusion", r"references", r"acknowledgments"]
HEADER_REGEX = r"(" + "|".join(SECTION_HEADERS) + r")"


def extract_text_from_pdf(pdf_path: str) -> str:
    """Extract text from a pdf file."""
    reader = PdfReader(pdf_path)
    text = ""
    for page in reader.pages:
        page_text = page.extract_text()
        if page_text:
            text += page_text + "\n"
    return text


def split_into_sections(text: str) -> Dict[str, str]:
    """Split the text into sections."""
    matches = list(re.finditer(HEADER_REGEX, text, re.IGNORECASE))
    sections = {}
    for i, match in enumerate(matches):
        start = match.start()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        section_title = match.group(0).strip().title()
        section_text = text[start:end].strip()
        # Combine if section already found
        if section_title in sections:
            sections[section_title] += "\n" + section_text
        else:
            sections[section_title] = section_text
    return sections


def chunk_text(text: str, max_words: int = 200) -> List[str]:
    """Split the text into chunks."""
    # Splits text into chunks of up to max_words words
    # words = text.split()

    # chunks = []
    # for i in range(0, len(words), max_words):
    #     chunk = " ".join(words[i:i+max_words])
    #     if len(chunk) > 20:  # Avoid empty/too short
    #         chunks.append(chunk)
    chunks = splitter.split_text(text)

    return chunks


def summarize_section(section_title: str, section_text: str) -> str:
    """Summarize a section of a research paper."""
    prompt = f"Summarize the following section of a research paper titled '{section_title}':\n\n{section_text}\n\nSummary:"
    summary = llm_client.chat(prompt)
    return summary.content


def summarize_pdf_sections(pdf_path: str):
    """Summarize the sections of a research paper."""
    text = extract_text_from_pdf(pdf_path)
    sections = split_into_sections(text)
    summaries = {}
    for title, content in sections.items():
        summaries[title] = summarize_section(title, content[:3000])  # Truncate to 3000 chars if needed
    return summaries, sections


def answer_question(retrieved_context: str, user_question: str) -> str:
    """Answer a question based on the retrieved context."""
    prompt = f"You are an AI research assistant. Given the following excerpts from a research paper:\n\n" f"{retrieved_context}\n\n" f"Answer the user's question:\n{user_question}\n\nAnswer:"

    response = llm_client.chat_stream(prompt)
    return response.content
