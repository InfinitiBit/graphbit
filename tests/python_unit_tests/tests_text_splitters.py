"""Unit tests for text splitter functionality."""

import pytest  # noqa: F401

from graphbit import CharacterSplitter, RecursiveSplitter, SentenceSplitter, TextChunk, TextSplitter, TextSplitterConfig, TokenSplitter


class TestTextSplitterConfig:
    """Test text splitter configuration."""

    def test_character_splitter_config(self):
        """Test creating character splitter config."""
        config = TextSplitterConfig.character(chunk_size=100, chunk_overlap=20)
        assert config is not None
        assert config.chunk_size == 100
        assert config.chunk_overlap == 20
        assert config.strategy_type == "character"

    def test_token_splitter_config(self):
        """Test creating token splitter config."""
        config = TextSplitterConfig.token(chunk_size=100, chunk_overlap=20, token_pattern=r"\w+")  # nosec B106: regex, not a password
        assert config is not None
        assert config.chunk_size == 100
        assert config.chunk_overlap == 20
        assert config.strategy_type == "token"

    def test_sentence_splitter_config(self):
        """Test creating sentence splitter config."""
        config = TextSplitterConfig.sentence(chunk_size=2, chunk_overlap=1, sentence_endings=["\\.\\s+", "\\!\\s+", "\\?\\s+"])
        assert config is not None
        assert config.chunk_size == 2
        assert config.chunk_overlap == 1
        assert config.strategy_type == "sentence"

    def test_recursive_splitter_config(self):
        """Test creating recursive splitter config."""
        config = TextSplitterConfig.recursive(chunk_size=1000, chunk_overlap=200, separators=["\n\n", "\n", ". ", " "])
        assert config is not None
        assert config.chunk_size == 1000
        assert config.chunk_overlap == 200
        assert config.strategy_type == "recursive"

    def test_markdown_splitter_config(self):
        """Test creating markdown splitter config."""
        config = TextSplitterConfig.markdown(chunk_size=1000, chunk_overlap=100, split_by_headers=True)
        assert config is not None
        assert config.chunk_size == 1000
        assert config.chunk_overlap == 100
        assert config.strategy_type == "markdown"

    def test_code_splitter_config(self):
        """Test creating code splitter config."""
        config = TextSplitterConfig.code(chunk_size=500, chunk_overlap=50, language="python")
        assert config is not None
        assert config.chunk_size == 500
        assert config.chunk_overlap == 50
        assert config.strategy_type == "code"

    def test_regex_splitter_config(self):
        """Test creating regex splitter config."""
        config = TextSplitterConfig.regex(pattern=r"\n\n+", chunk_size=1000, chunk_overlap=100)
        assert config is not None
        assert config.chunk_size == 1000
        assert config.chunk_overlap == 100
        assert config.strategy_type == "regex"


class TestCharacterSplitter:
    """Test character splitter functionality."""

    def test_character_splitter_creation(self):
        """Test creating character splitter."""
        splitter = CharacterSplitter(chunk_size=100, chunk_overlap=20)
        assert splitter is not None
        assert splitter.chunk_size == 100
        assert splitter.chunk_overlap == 20

    def test_character_splitter_split_text(self):
        """Test splitting text with character splitter."""
        splitter = CharacterSplitter(chunk_size=5, chunk_overlap=1)
        text = "abcdefghij"
        chunks = splitter.split_text(text)

        assert len(chunks) > 0
        assert len(chunks) <= (len(text) // (5 - 1)) + 1

        for chunk in chunks:
            assert isinstance(chunk, TextChunk)
            assert len(chunk.content) <= 5
            assert chunk.start_index >= 0
            assert chunk.end_index <= len(text)
            assert chunk.chunk_index >= 0
            assert chunk.chunk_index < len(chunks)

    def test_character_splitter_empty_text(self):
        """Test splitting empty text."""
        splitter = CharacterSplitter(chunk_size=10)
        chunks = splitter.split_text("")
        assert len(chunks) == 0


class TestTokenSplitter:
    """Test token splitter functionality."""

    def test_token_splitter_creation(self):
        """Test creating token splitter."""
        splitter = TokenSplitter(chunk_size=100, chunk_overlap=20)
        assert splitter is not None
        assert splitter.chunk_size == 100
        assert splitter.chunk_overlap == 20

    def test_token_splitter_with_pattern(self):
        """Test token splitter with custom pattern."""
        splitter = TokenSplitter(chunk_size=100, chunk_overlap=20, token_pattern=r"\w+")  # nosec B106: regex, not a password
        assert splitter is not None

    def test_token_splitter_split_text(self):
        """Test splitting text with token splitter."""
        splitter = TokenSplitter(chunk_size=2, chunk_overlap=0)
        text = "one two three four"
        chunks = splitter.split_text(text)

        assert len(chunks) > 0
        assert len(chunks) <= len(text.split())

        for chunk in chunks:
            assert isinstance(chunk, TextChunk)
            assert chunk.start_index >= 0
            assert chunk.end_index <= len(text)
            assert chunk.chunk_index >= 0
            assert chunk.chunk_index < len(chunks)
            tokens = chunk.content.split()
            assert len(tokens) <= 2


class TestSentenceSplitter:
    """Test sentence splitter functionality."""

    def test_sentence_splitter_creation(self):
        """Test creating sentence splitter."""
        splitter = SentenceSplitter(chunk_size=2)
        assert splitter is not None
        assert splitter.chunk_size == 2

    def test_sentence_splitter_with_endings(self):
        """Test sentence splitter with custom endings."""
        splitter = SentenceSplitter(chunk_size=2, sentence_endings=["\\.\\s+", "\\!\\s+", "\\?\\s+"])
        assert splitter is not None

    def test_sentence_splitter_split_text(self):
        """Test splitting text with sentence splitter."""
        splitter = SentenceSplitter(chunk_size=2, chunk_overlap=0)
        text = "One. Two. Three."
        chunks = splitter.split_text(text)

        assert len(chunks) > 0
        assert len(chunks) <= (len(text.split(".")) - 1)

        for chunk in chunks:
            assert isinstance(chunk, TextChunk)
            assert chunk.start_index >= 0
            assert chunk.end_index <= len(text)
            assert chunk.chunk_index >= 0
            assert chunk.chunk_index < len(chunks)
            sentences = [s.strip() for s in chunk.content.split(".") if s.strip()]
            assert len(sentences) <= 2


class TestRecursiveSplitter:
    """Test recursive splitter functionality."""

    def test_recursive_splitter_creation(self):
        """Test creating recursive splitter."""
        splitter = RecursiveSplitter(chunk_size=1000, chunk_overlap=200)
        assert splitter is not None
        assert splitter.chunk_size == 1000
        assert splitter.chunk_overlap == 200

    def test_recursive_splitter_with_separators(self):
        """Test recursive splitter with custom separators."""
        splitter = RecursiveSplitter(chunk_size=1000, chunk_overlap=200, separators=["\n\n", "\n", ". ", " "])
        assert splitter is not None
        assert len(splitter.separators) > 0

    def test_recursive_splitter_split_text(self):
        """Test splitting text with recursive splitter."""
        splitter = RecursiveSplitter(chunk_size=50, chunk_overlap=5)
        text = "First part.\n\nSecond part.\n\nThird part."
        chunks = splitter.split_text(text)

        assert len(chunks) > 0
        for chunk in chunks:
            assert isinstance(chunk, TextChunk)
            assert chunk.start_index >= 0
            assert chunk.end_index <= len(text)
            assert chunk.chunk_index >= 0
            assert chunk.chunk_index < len(chunks)
            assert len(chunk.content) <= 50


class TestTextChunk:
    """Test text chunk functionality."""

    def test_text_chunk_attributes(self):
        """Test text chunk attributes."""
        splitter = CharacterSplitter(chunk_size=5)
        chunks = splitter.split_text("12345")
        chunk = chunks[0]

        assert isinstance(chunk.content, str)
        assert isinstance(chunk.start_index, int)
        assert isinstance(chunk.end_index, int)
        assert isinstance(chunk.chunk_index, int)
        assert isinstance(chunk.metadata, dict)

    def test_text_chunk_representation(self):
        """Test text chunk string representation."""
        splitter = CharacterSplitter(chunk_size=5)
        chunks = splitter.split_text("12345")
        chunk = chunks[0]

        assert str(chunk) is not None
        assert repr(chunk) is not None


class TestTextSplitter:
    """Test generic text splitter functionality."""

    def test_text_splitter_creation(self):
        """Test creating generic text splitter."""
        config = TextSplitterConfig.character(chunk_size=100)
        splitter = TextSplitter(config)
        assert splitter is not None

    def test_text_splitter_split_text(self):
        """Test splitting text with generic splitter."""
        config = TextSplitterConfig.character(chunk_size=5)
        splitter = TextSplitter(config)
        chunks = splitter.split_text("12345")

        assert len(chunks) > 0
        for chunk in chunks:
            assert isinstance(chunk, TextChunk)
            assert len(chunk.content) <= 5

    def test_text_splitter_create_documents(self):
        """Test creating documents from text."""
        config = TextSplitterConfig.character(chunk_size=5)
        splitter = TextSplitter(config)
        docs = splitter.create_documents("12345")

        assert len(docs) > 0
        for doc in docs:
            assert isinstance(doc, dict)
            assert "content" in doc
            assert "start_index" in doc
            assert "end_index" in doc
            assert "chunk_index" in doc
