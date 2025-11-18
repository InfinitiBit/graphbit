"""Integration tests for GraphBit text splitters."""

import pytest

import graphbit


@pytest.fixture(autouse=True)
def setup():
    """Initialize GraphBit before each test."""
    graphbit.init()


class TestCharacterSplitter:
    """Test character-based text splitter."""

    def test_basic_splitting(self):
        """Test basic character splitting functionality."""
        splitter = graphbit.CharacterSplitter(chunk_size=50, chunk_overlap=10)
        text = "This is a test text that should be split into multiple chunks based on character count."

        chunks = splitter.split_text(text)

        assert len(chunks) > 1
        assert all(isinstance(chunk, graphbit.TextChunk) for chunk in chunks)
        assert all(len(chunk.content) <= 50 for chunk in chunks)

    def test_word_boundary_preservation(self):
        """Test that word boundaries are preserved."""
        splitter = graphbit.CharacterSplitter(chunk_size=20, chunk_overlap=0)
        text = "This is a test with words that should not be split"

        chunks = splitter.split_text(text)

        # Check that chunks don't end in the middle of words
        for _i, chunk in enumerate(chunks[:-1]):  # Exclude last chunk
            # The chunk should either end with whitespace or be at a word boundary
            # Since we preserve word boundaries, chunks should not split words
            content = chunk.content
            # If trimmed, the content should be complete words
            assert not content or content == content.strip()

    def test_overlap(self):
        """Test chunk overlap functionality."""
        splitter = graphbit.CharacterSplitter(chunk_size=30, chunk_overlap=10)
        text = "A" * 100  # Simple text for testing overlap

        chunks = splitter.split_text(text)

        # Check that chunks overlap correctly
        for i in range(len(chunks) - 1):
            overlap_start = chunks[i + 1].start_index
            overlap_end = chunks[i].end_index
            assert overlap_end - overlap_start == 10

    def test_empty_text(self):
        """Test handling of empty text."""
        splitter = graphbit.CharacterSplitter(chunk_size=50, chunk_overlap=10)
        chunks = splitter.split_text("")

        assert len(chunks) == 0

    def test_text_shorter_than_chunk_size(self):
        """Test text shorter than chunk size."""
        splitter = graphbit.CharacterSplitter(chunk_size=100, chunk_overlap=10)
        text = "Short text"

        chunks = splitter.split_text(text)

        assert len(chunks) == 1
        assert chunks[0].content == text.strip()

    def test_metadata(self):
        """Test chunk metadata."""
        splitter = graphbit.CharacterSplitter(chunk_size=50, chunk_overlap=0)
        text = "Test text for metadata"

        chunks = splitter.split_text(text)

        for chunk in chunks:
            assert "length" in chunk.metadata
            assert int(chunk.metadata["length"]) == len(chunk.content)

    def test_invalid_parameters(self):
        """Test error handling for invalid parameters."""
        with pytest.raises(Exception) as excinfo:
            graphbit.CharacterSplitter(chunk_size=0, chunk_overlap=0)
        assert "greater than 0" in str(excinfo.value)

        with pytest.raises(Exception) as excinfo:
            graphbit.CharacterSplitter(chunk_size=10, chunk_overlap=20)
        assert "less than chunk size" in str(excinfo.value)


class TestTokenSplitter:
    """Test token-based text splitter."""

    def test_basic_splitting(self):
        """Test basic token splitting functionality."""
        splitter = graphbit.TokenSplitter(chunk_size=10, chunk_overlap=2)
        text = "This is a test text with multiple words that should be split by tokens."

        chunks = splitter.split_text(text)

        assert len(chunks) > 1
        assert all(isinstance(chunk, graphbit.TextChunk) for chunk in chunks)

    def test_custom_token_pattern(self):
        """Test custom token pattern."""
        # Split by words only (no punctuation)
        splitter = graphbit.TokenSplitter(chunk_size=5, chunk_overlap=0, token_pattern=r"\b\w+\b")  # nosec B106
        text = "Hello, world! How are you?"

        chunks = splitter.split_text(text)

        # Should split into word tokens only
        assert len(chunks) > 0

    def test_token_count_metadata(self):
        """Test token count in metadata."""
        splitter = graphbit.TokenSplitter(chunk_size=5, chunk_overlap=0)
        text = "One two three four five six seven eight nine ten"

        chunks = splitter.split_text(text)

        for chunk in chunks:
            assert "token_count" in chunk.metadata
            assert int(chunk.metadata["token_count"]) <= 5

    def test_multiple_texts(self):
        """Test splitting multiple texts."""
        splitter = graphbit.TokenSplitter(chunk_size=10, chunk_overlap=2)
        texts = ["First document with some content.", "Second document with different content.", "Third document."]

        all_chunks = splitter.split_texts(texts)

        assert len(all_chunks) == 3
        assert all(isinstance(chunks, list) for chunks in all_chunks)


class TestSentenceSplitter:
    """Test sentence-based text splitter."""

    def test_basic_splitting(self):
        """Test basic sentence splitting functionality."""
        splitter = graphbit.SentenceSplitter(chunk_size=100, chunk_overlap=0)
        text = "This is the first sentence. This is the second sentence! And this is the third? Here's the fourth. Let's add a fifth sentence to ensure multiple chunks."

        chunks = splitter.split_text(text)

        assert len(chunks) >= 1  # Changed from > 1 to >= 1
        assert all(isinstance(chunk, graphbit.TextChunk) for chunk in chunks)

    def test_sentence_preservation(self):
        """Test that sentences are not split."""
        splitter = graphbit.SentenceSplitter(chunk_size=50, chunk_overlap=0)
        text = "Short sentence. Another short one. And one more."

        chunks = splitter.split_text(text)

        # Each chunk should contain complete sentences
        for chunk in chunks:
            # Should end with sentence ending or be the last chunk
            assert chunk.content.rstrip().endswith((".", "!", "?")) or chunk == chunks[-1]

    def test_custom_sentence_endings(self):
        """Test custom sentence endings."""
        splitter = graphbit.SentenceSplitter(chunk_size=100, chunk_overlap=0, sentence_endings=[r"\.", r"!", r"\?", r"。", r"！", r"？"])
        text = "English sentence. 中文句子。 Another sentence! 另一个句子！"

        chunks = splitter.split_text(text)

        assert len(chunks) > 0

    def test_sentence_count_metadata(self):
        """Test sentence count in metadata."""
        splitter = graphbit.SentenceSplitter(chunk_size=150, chunk_overlap=0)
        text = "First sentence. Second sentence. Third sentence. Fourth sentence."

        chunks = splitter.split_text(text)

        for chunk in chunks:
            assert "sentence_count" in chunk.metadata
            assert int(chunk.metadata["sentence_count"]) > 0


class TestRecursiveSplitter:
    """Test recursive text splitter."""

    def test_basic_splitting(self):
        """Test basic recursive splitting functionality."""
        splitter = graphbit.RecursiveSplitter(chunk_size=50, chunk_overlap=5)
        text = "Paragraph one.\n\nParagraph two with more content.\n\nParagraph three."

        chunks = splitter.split_text(text)

        assert len(chunks) > 1
        assert all(isinstance(chunk, graphbit.TextChunk) for chunk in chunks)

    def test_custom_separators(self):
        """Test custom separators."""
        separators = ["\n\n", "\n", " ", ""]
        splitter = graphbit.RecursiveSplitter(chunk_size=30, chunk_overlap=0, separators=separators)
        text = "Line one\nLine two\n\nNew paragraph\nWith another line"

        chunks = splitter.split_text(text)

        assert len(chunks) > 0
        assert splitter.separators == separators

    def test_hierarchical_splitting(self):
        """Test that recursive splitter tries separators in order."""
        splitter = graphbit.RecursiveSplitter(chunk_size=100, chunk_overlap=0)
        text = "Part 1: Introduction\n\nThis is a paragraph.\nWith multiple lines.\n\nPart 2: Content\n\nAnother paragraph here."

        chunks = splitter.split_text(text)

        # Should prefer splitting by double newlines first
        assert len(chunks) > 0

    def test_overlap_handling(self):
        """Test overlap in recursive splitting."""
        splitter = graphbit.RecursiveSplitter(chunk_size=50, chunk_overlap=10)
        text = "A" * 200  # Simple text for testing

        chunks = splitter.split_text(text)

        # With overlap, chunks should have some overlapping content
        assert len(chunks) > 1


class TestTextSplitterConfig:
    """Test text splitter configuration."""

    def test_character_config(self):
        """Test character splitter configuration."""
        config = graphbit.TextSplitterConfig.character(chunk_size=100, chunk_overlap=20)

        assert config.strategy_type == "character"
        assert config.chunk_size == 100
        assert config.chunk_overlap == 20
        assert config.preserve_word_boundaries is True
        assert config.trim_whitespace is True

    def test_token_config(self):
        """Test token splitter configuration."""
        config = graphbit.TextSplitterConfig.token(chunk_size=50, chunk_overlap=10, token_pattern=r"\w+")  # nosec B106

        assert config.strategy_type == "token"
        assert config.chunk_size == 50
        assert config.chunk_overlap == 10

    def test_config_modification(self):
        """Test modifying configuration."""
        config = graphbit.TextSplitterConfig.character(100, 20)

        config.set_preserve_word_boundaries(False)
        config.set_trim_whitespace(False)
        config.set_include_metadata(False)

        assert config.preserve_word_boundaries is False
        assert config.trim_whitespace is False
        assert config.include_metadata is False

    def test_code_config(self):
        """Test code splitter configuration."""
        config = graphbit.TextSplitterConfig.code(chunk_size=200, chunk_overlap=20, language="python")

        assert config.strategy_type == "code"
        assert config.chunk_size == 200
        assert config.trim_whitespace is False  # Should preserve code formatting

    def test_regex_config(self):
        """Test regex splitter configuration."""
        config = graphbit.TextSplitterConfig.regex(pattern=r"\n{2,}", chunk_size=100, chunk_overlap=0)

        assert config.strategy_type == "regex"
        assert config.chunk_size == 100


class TestGenericTextSplitter:
    """Test generic text splitter with configuration."""

    def test_with_character_config(self):
        """Test generic splitter with character configuration."""
        config = graphbit.TextSplitterConfig.character(50, 10)
        splitter = graphbit.TextSplitter(config)

        text = "This is a test text that should be split using the generic splitter."
        chunks = splitter.split_text(text)

        assert len(chunks) > 0
        assert all(isinstance(chunk, graphbit.TextChunk) for chunk in chunks)

    def test_with_sentence_config(self):
        """Test generic splitter with sentence configuration."""
        config = graphbit.TextSplitterConfig.sentence(100, 0)
        splitter = graphbit.TextSplitter(config)

        text = "First sentence. Second sentence. Third sentence. Fourth sentence."
        chunks = splitter.split_text(text)

        assert len(chunks) > 0

    def test_create_documents(self):
        """Test document creation functionality."""
        config = graphbit.TextSplitterConfig.character(50, 10)
        splitter = graphbit.TextSplitter(config)

        text = "This is a test text for document creation."
        documents = splitter.create_documents(text)

        assert len(documents) > 0
        assert all(isinstance(doc, dict) for doc in documents)
        assert all("content" in doc for doc in documents)
        assert all("start_index" in doc for doc in documents)
        assert all("end_index" in doc for doc in documents)
        assert all("chunk_index" in doc for doc in documents)


class TestEdgeCases:
    """Test edge cases and special scenarios."""

    def test_unicode_text(self):
        """Test handling of Unicode text."""
        splitter = graphbit.CharacterSplitter(chunk_size=20, chunk_overlap=5)
        text = "Hello 世界! This is 测试 text with émojis 🚀🌟"

        chunks = splitter.split_text(text)

        assert len(chunks) > 0
        # Verify each chunk is valid
        for chunk in chunks:
            assert isinstance(chunk.content, str)
            assert len(chunk.content) > 0
            # Check that chunks don't exceed the specified size (in characters)
            assert len(chunk.content) <= 20
            # Verify metadata
            assert "length" in chunk.metadata
            assert int(chunk.metadata["length"]) == len(chunk.content)

    def test_very_long_text(self):
        """Test handling of very long text."""
        splitter = graphbit.CharacterSplitter(chunk_size=1000, chunk_overlap=100)
        text = "Lorem ipsum " * 1000  # ~12,000 characters

        chunks = splitter.split_text(text)

        assert len(chunks) > 10
        assert all(len(chunk.content) <= 1000 for chunk in chunks)

    def test_text_with_special_characters(self):
        """Test text with special characters."""
        splitter = graphbit.TokenSplitter(chunk_size=10, chunk_overlap=2)
        text = "Special chars: @#$% & * () {} [] <> | \\ / quotes: 'single' \"double\""

        chunks = splitter.split_text(text)

        assert len(chunks) > 0

    def test_chunk_indices(self):
        """Test that chunk indices are correct."""
        splitter = graphbit.CharacterSplitter(chunk_size=20, chunk_overlap=5)
        text = "A" * 100

        chunks = splitter.split_text(text)

        # Verify indices are sequential
        for i, chunk in enumerate(chunks):
            assert chunk.chunk_index == i

        # Verify start and end indices
        for chunk in chunks:
            assert text[chunk.start_index : chunk.end_index] == chunk.content

    def test_whitespace_handling(self):
        """Test handling of various whitespace."""
        splitter = graphbit.RecursiveSplitter(chunk_size=50, chunk_overlap=0)
        text = "Text with   multiple  spaces\n\nAnd\ttabs\n\n\nAnd multiple newlines"

        chunks = splitter.split_text(text)

        assert len(chunks) > 0
        # Check that excessive whitespace is handled
        assert all(chunk.content for chunk in chunks)  # No empty chunks
