"""Unit tests for document loading functionality."""

import tempfile

import pytest

from graphbit import DocumentLoader, DocumentLoaderConfig


class TestDocumentLoaderConfig:
    """Test document loader configuration."""

    def test_document_loader_config_creation(self):
        """Test creating document loader config with defaults."""
        config = DocumentLoaderConfig()
        assert config is not None
        assert config.max_file_size > 0
        assert config.default_encoding != ""
        assert isinstance(config.preserve_formatting, bool)

    def test_document_loader_config_with_params(self):
        """Test creating document loader config with parameters."""
        config = DocumentLoaderConfig(max_file_size=1024 * 1024, default_encoding="utf-8", preserve_formatting=True)  # 1MB
        assert config.max_file_size == 1024 * 1024
        assert config.default_encoding == "utf-8"
        assert config.preserve_formatting is True

    def test_document_loader_config_validation(self):
        """Test document loader config validation."""
        with pytest.raises(ValueError):
            DocumentLoaderConfig(max_file_size=0)

        with pytest.raises(ValueError):
            DocumentLoaderConfig(default_encoding="")


class TestDocumentContent:
    """Test document content functionality."""

    def test_document_content_attributes(self):
        """Test document content creation and attributes."""
        # Create a temporary file for testing
        with tempfile.NamedTemporaryFile(mode="w", suffix=".txt") as temp_file:
            temp_file.write("Test content")
            temp_file.flush()

            # Create loader and load document
            loader = DocumentLoader()
            content = loader.load_document(temp_file.name, "txt")

            # Test attributes
            assert content.source == temp_file.name
            assert content.document_type == "txt"
            assert content.content == "Test content"
            assert content.file_size > 0
            assert content.extracted_at > 0
            assert isinstance(content.metadata, dict)

    def test_document_content_empty_text(self):
        """Test document content with empty text."""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".txt") as temp_file:
            temp_file.write("")
            temp_file.flush()

            loader = DocumentLoader()
            content = loader.load_document(temp_file.name, "txt")
            assert content.is_empty()
            assert content.content_length() == 0

    def test_document_content_preview(self):
        """Test document content preview functionality."""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".txt") as temp_file:
            long_text = "A" * 1000
            temp_file.write(long_text)
            temp_file.flush()

            loader = DocumentLoader()
            content = loader.load_document(temp_file.name, "txt")
            preview = content.preview(max_length=100)
            assert len(preview) <= 103  # 100 chars + "..."
            assert preview.endswith("...")


class TestDocumentLoader:
    """Test document loader functionality."""

    def test_document_loader_creation(self):
        """Test creating document loader with default config."""
        loader = DocumentLoader()
        assert loader is not None

    def test_document_loader_creation_with_config(self):
        """Test creating document loader with custom config."""
        config = DocumentLoaderConfig(max_file_size=2 * 1024 * 1024, default_encoding="utf-8", preserve_formatting=True)  # 2MB
        loader = DocumentLoader(config)
        assert loader is not None

    def test_supported_types(self):
        """Test getting supported document types."""
        types = DocumentLoader.supported_types()
        assert isinstance(types, list)
        assert len(types) > 0
        assert all(isinstance(t, str) for t in types)

    def test_detect_document_type(self):
        """Test document type detection."""
        assert DocumentLoader.detect_document_type("test.txt") == "txt"
        assert DocumentLoader.detect_document_type("test.pdf") == "pdf"
        assert DocumentLoader.detect_document_type("test.unknown") is None

    def test_load_text_file(self):
        """Test loading text file."""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".txt") as temp_file:
            temp_file.write("Test content")
            temp_file.flush()

            loader = DocumentLoader()
            content = loader.load_document(temp_file.name, "txt")
            assert content.content == "Test content"
            assert content.document_type == "txt"

    def test_load_with_metadata(self):
        """Test loading document with metadata."""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".txt") as temp_file:
            temp_file.write("Test content")
            temp_file.flush()

            loader = DocumentLoader()
            content = loader.load_document(temp_file.name, "txt")
            assert isinstance(content.metadata, dict)
            assert "file_size" in content.metadata


class TestDocumentLoaderErrorHandling:
    """Test document loader error handling."""

    def test_load_nonexistent_file(self):
        """Test loading nonexistent file."""
        loader = DocumentLoader()
        with pytest.raises(Exception, match="(?i)(no such file|not found|exist|open)"):
            loader.load_document("nonexistent.txt", "txt")

    def test_load_with_invalid_type(self):
        """Test loading with invalid document type."""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".txt") as temp_file:
            temp_file.write("Test content")
            temp_file.flush()

            loader = DocumentLoader()
            with pytest.raises(Exception, match="(?i)(invalid|type)"):
                loader.load_document(temp_file.name, "invalid_type")

    def test_load_empty_path(self):
        """Test loading with empty path."""
        loader = DocumentLoader()
        with pytest.raises(ValueError):
            loader.load_document("", "txt")

    def test_load_empty_type(self):
        """Test loading with empty type."""
        loader = DocumentLoader()
        with pytest.raises(ValueError):
            loader.load_document("test.txt", "")
