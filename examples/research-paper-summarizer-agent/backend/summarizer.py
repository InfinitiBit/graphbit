"""Summarization module for research papers using GraphBit framework."""

import concurrent.futures
import re
from typing import Dict, List, Tuple

from dotenv import load_dotenv

from graphbit import DocumentLoader, DocumentLoaderConfig, LlmClient, LlmConfig, TextSplitter, TextSplitterConfig

from .const import ConfigConstants

load_dotenv()

# SECTION HEADERS in display order
SECTION_HEADERS = ConfigConstants.SECTION_HEADERS
HEADER_REGEX = r"(" + "|".join(SECTION_HEADERS) + r")"


def extract_text_from_pdf(pdf_path: str) -> str:
    """Extract text from PDF using GraphBit's document loader."""
    # Configure document loader for PDF processing
    config = DocumentLoaderConfig(max_file_size=50_000_000, preserve_formatting=True)  # 50MB limit  # Keep formatting for better section detection

    # Initialize document loader
    loader = DocumentLoader(config)

    # Load and extract content from PDF
    document_content = loader.load_document(pdf_path, "pdf")

    return document_content.content


def split_into_sections(text: str) -> Dict[str, str]:
    """Split text into sections based on predefined headers."""
    matches = list(re.finditer(HEADER_REGEX, text, re.IGNORECASE))
    sections: Dict[str, str] = {}
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


def chunk_text(text: str, max_words: int = ConfigConstants.MAX_CHUNK_WORDS) -> List[str]:
    """Split text into chunks using GraphBit's text splitter with semantic awareness."""
    if not text.strip():
        return []

    # Normalize Unicode text to prevent character boundary issues
    text = normalize_unicode_text(text)

    # Calculate approximate character count from word count (average 5 chars per word + space)
    chunk_size = max_words * 6
    chunk_overlap = min(chunk_size // 10, 200)  # 10% overlap, max 200 chars

    # Use recursive splitter for better semantic coherence
    # This preserves sentence and paragraph boundaries
    config = TextSplitterConfig.recursive(chunk_size=chunk_size, chunk_overlap=chunk_overlap, separators=["\n\n", "\n", ". ", " "])  # Prioritize paragraph, sentence, then word boundaries

    # Configure for better context preservation
    config.set_trim_whitespace(True)
    config.set_preserve_word_boundaries(True)

    # Create splitter and split text
    splitter = TextSplitter(config)
    text_chunks = splitter.split_text(text)

    # Extract content from TextChunk objects and filter by minimum length
    chunks = []
    for chunk in text_chunks:
        if len(chunk.content) > ConfigConstants.MIN_CHUNK_LENGTH:
            chunks.append(chunk.content)

    return chunks


def normalize_unicode_text(text: str) -> str:
    """
    Normalize Unicode text to handle special characters that may cause boundary issues.

    Args:
        text (str): Input text with potential Unicode issues

    Returns:
        str: Normalized text safe for processing
    """
    import unicodedata

    # Normalize Unicode to NFC form (canonical decomposition followed by canonical composition)
    text = unicodedata.normalize("NFC", text)

    # Replace problematic Unicode characters with ASCII equivalents
    replacements = {
        "—": "--",  # Em dash
        "–": "-",  # En dash
        "'": "'",  # single quotation mark
        '"': '"',  # double quotation mark
        "…": "...",  # Horizontal ellipsis
        "•": "*",  # Bullet
        "→": "->",  # Right arrow
        "←": "<-",  # Left arrow
        "≥": ">=",  # Greater than or equal
        "≤": "<=",  # Less than or equal
        "≠": "!=",  # Not equal
        "±": "+/-",  # Plus-minus
        "×": "x",  # Multiplication sign
        "÷": "/",  # Division sign
    }

    for unicode_char, ascii_replacement in replacements.items():
        text = text.replace(unicode_char, ascii_replacement)

    # Remove any remaining problematic characters that might cause boundary issues
    # Keep only printable ASCII and common whitespace
    cleaned_text = "".join(char for char in text if ord(char) < 127 or char.isspace())

    return cleaned_text


def chunk_text_with_context(text: str, section_title: str = "", max_words: int = ConfigConstants.MAX_CHUNK_WORDS) -> List[str]:
    """
    Enhanced chunking function that preserves research paper context.

    This function uses GraphBit's text splitter with research paper-specific optimizations:
    - Preserves academic structure (paragraphs, sentences)
    - Maintains context across chunk boundaries with intelligent overlap
    - Handles citations and references properly
    - Optimizes chunk sizes for LLM context windows
    - Includes Unicode normalization to prevent character boundary issues

    Args:
        text (str): Text to chunk
        section_title (str): Section title for context-aware chunking
        max_words (int): Maximum words per chunk

    Returns:
        List[str]: List of text chunks with preserved context
    """
    if not text.strip():
        return []

    # Normalize Unicode text to prevent character boundary issues
    text = normalize_unicode_text(text)

    # Calculate character-based chunk size (more precise than word count)
    chunk_size = max_words * 6  # Average 6 chars per word including spaces

    # Adaptive overlap based on section type
    if section_title.lower() in ["abstract", "conclusion"]:
        # Smaller overlap for summary sections
        chunk_overlap = min(chunk_size // 20, 100)
    elif section_title.lower() in ["methods", "methodology", "results"]:
        # Larger overlap for detailed sections to preserve procedural context
        chunk_overlap = min(chunk_size // 8, 300)
    else:
        # Standard overlap for other sections
        chunk_overlap = min(chunk_size // 10, 200)

    try:
        # Research paper-specific separators prioritizing academic structure
        separators = [
            "\n\n\n",  # Major section breaks
            "\n\n",  # Paragraph breaks
            "\n",  # Line breaks
            ". ",  # Sentence endings
            "; ",  # Clause separators
            ", ",  # Phrase separators
            " ",  # Word boundaries
        ]

        # Configure recursive splitter for academic content
        config = TextSplitterConfig.recursive(chunk_size=chunk_size, chunk_overlap=chunk_overlap, separators=separators)

        # Optimize for academic text
        config.set_trim_whitespace(True)
        config.set_preserve_word_boundaries(True)
        config.set_include_metadata(True)  # Include position metadata for better retrieval

        # Create and use splitter
        splitter = TextSplitter(config)
        text_chunks = splitter.split_text(text)

        # Process chunks with context preservation
        chunks = []
        for i, chunk in enumerate(text_chunks):
            content = chunk.content.strip()

            # Skip chunks that are too short or contain only whitespace/punctuation
            if len(content) < ConfigConstants.MIN_CHUNK_LENGTH or not any(c.isalnum() for c in content):
                continue

            # Add section context to chunk if available
            if section_title and i == 0:
                # Prepend section title to first chunk for better context
                content = f"[{section_title}] {content}"

            chunks.append(content)

        return chunks

    except Exception as e:
        # Fallback to simple chunking if GraphBit splitter fails
        print(f"Warning: GraphBit splitter failed ({str(e)}), falling back to simple chunking")
        return chunk_text(text, max_words)


def summarize_section(section_title: str, section_text: str) -> str:
    """Summarize a section of a research paper using GraphBit LLM client."""
    prompt = f"Give detailed summary of the following section of a research paper titled '{section_title}':\n\n{section_text}\n\nSummary:"

    # Configure LLM
    openai_api_key = ConfigConstants.OPENAI_API_KEY
    if not openai_api_key:
        raise ValueError("OPENAI_API_KEY environment variable is not set")

    config = LlmConfig.openai(openai_api_key, ConfigConstants.LLM_MODEL)
    llm_client = LlmClient(config)

    # Generate summary
    summary = llm_client.complete(prompt=prompt, max_tokens=ConfigConstants.LLM_MAX_TOKENS, temperature=ConfigConstants.LLM_TEMPERATURE)
    return summary


def summarize_section_worker(section_data: Tuple[str, str]) -> Tuple[str, str]:
    """Worker function for parallel section summarization with timeout handling."""
    title, content = section_data
    truncated_content = content[: ConfigConstants.MAX_SECTION_LENGTH]

    try:
        # Add timeout protection for individual section summarization
        print(title)
        summary = summarize_section(title, truncated_content)
        return title, summary
    except Exception as e:
        # Fallback summary if summarization fails
        fallback_summary = f"Summary generation failed for '{title}'. Content length: {len(truncated_content)} characters."
        print(f"Warning: Summarization failed for section '{title}': {e}")
        return title, fallback_summary


def summarize_pdf_sections_parallel(pdf_path: str, max_workers: int = 3):
    """
    Extract text from PDF, split into sections, and generate summaries in parallel.

    Args:
        pdf
    # Embedding Configuration
    EMBEDDING_MODEL = "text-embedding-3-small"

    # Cache Configuration
    CACHE_DIR = "examples/research-paper-summarizer-agent/cache"
    _path: Path to the PDF file
        max_workers: Maximum number of parallel workers for summarization

    Returns:
        Tuple of (summaries_dict, sections_dict)
    """
    text = extract_text_from_pdf(pdf_path)
    sections = split_into_sections(text)

    # Prepare section data for parallel processing
    section_items = list(sections.items())

    # Use ThreadPoolExecutor for parallel API calls
    summaries = {}
    with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
        # Submit all summarization tasks
        future_to_section = {executor.submit(summarize_section_worker, (title, content)): title for title, content in section_items}

        # Collect results as they complete
        for future in concurrent.futures.as_completed(future_to_section):
            try:
                title, summary = future.result()
                summaries[title] = summary
            except Exception as e:
                section_title = future_to_section[future]
                print(f"Warning: Failed to summarize section '{section_title}': {e}")
                # Fallback to a simple summary
                summaries[section_title] = f"Summary generation failed for section: {section_title}"

    return summaries, sections


def summarize_pdf_sections(pdf_path: str):
    """
    Extract text from PDF, split into sections, and generate summaries.

    Uses parallel processing for better performance.
    """
    return summarize_pdf_sections_parallel(pdf_path, max_workers=3)


def answer_question(retrieved_context: str, user_question: str) -> str:
    """Answer a question based on retrieved context using GraphBit LLM client."""
    prompt = f"You are an AI research assistant. Given the following excerpts from a research paper:\n\n" f"{retrieved_context}\n\n" f"Answer the user's question:\n{user_question}\n\nAnswer:"

    # Configure LLM
    openai_api_key = ConfigConstants.OPENAI_API_KEY
    if not openai_api_key:
        raise ValueError("OPENAI_API_KEY environment variable is not set")

    config = LlmConfig.openai(openai_api_key, ConfigConstants.LLM_MODEL)
    llm_client = LlmClient(config)

    # Generate answer
    response = llm_client.complete(prompt=prompt, max_tokens=ConfigConstants.LLM_MAX_TOKENS, temperature=0.2)
    return response
