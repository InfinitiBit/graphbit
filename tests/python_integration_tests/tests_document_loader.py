#!/usr/bin/env python3
"""Document Loader Integration Tests for GraphBit Python library."""

import json
import os
import tempfile
from typing import Any, Dict

import pytest

import graphbit


class TestDocumentLoader:
    """Test the Document Loader functionality in GraphBit."""

    @classmethod
    def setup_class(cls):
        """Set up test environment."""
        graphbit.init(enable_tracing=False)

    def setup_method(self):
        """Set up test fixtures before each test."""
        self.temp_dir = tempfile.TemporaryDirectory()
        self.temp_path = self.temp_dir.name

    def teardown_method(self):
        """Clean up after each test."""
        self.temp_dir.cleanup()

    def test_document_loader_config_creation(self):
        """Test DocumentLoaderConfig creation with various parameters."""
        # Test default configuration
        config = graphbit.DocumentLoaderConfig()
        assert config.max_file_size == 10 * 1024 * 1024  # 10MB default
        assert config.default_encoding == "utf-8"
        assert config.preserve_formatting is False

        # Test custom configuration
        custom_config = graphbit.DocumentLoaderConfig(max_file_size=5 * 1024 * 1024, default_encoding="utf-8", preserve_formatting=True)  # 5MB
        assert custom_config.max_file_size == 5 * 1024 * 1024
        assert custom_config.preserve_formatting is True

    def test_document_loader_config_validation(self):
        """Test DocumentLoaderConfig parameter validation."""
        # Test invalid max_file_size
        with pytest.raises(ValueError):
            graphbit.DocumentLoaderConfig(max_file_size=0)

        # Test invalid encoding
        with pytest.raises(ValueError):
            graphbit.DocumentLoaderConfig(default_encoding="")

    def test_document_loader_config_properties(self):
        """Test DocumentLoaderConfig property access and modification."""
        config = graphbit.DocumentLoaderConfig()

        # Test getter/setter for max_file_size
        config.max_file_size = 20 * 1024 * 1024
        assert config.max_file_size == 20 * 1024 * 1024

        # Test getter/setter for encoding
        config.default_encoding = "iso-8859-1"
        assert config.default_encoding == "iso-8859-1"

        # Test getter/setter for preserve_formatting
        config.preserve_formatting = True
        assert config.preserve_formatting is True

    def test_document_loader_creation(self):
        """Test DocumentLoader creation."""
        # Test default loader
        loader = graphbit.DocumentLoader()
        assert loader is not None

        # Test loader with custom config
        config = graphbit.DocumentLoaderConfig(max_file_size=1024 * 1024)  # 1MB
        loader_with_config = graphbit.DocumentLoader(config)
        assert loader_with_config is not None

    def test_text_document_loading(self):
        """Test loading text documents."""
        # Create a test text file
        text_content = "This is a test document.\nIt has multiple lines.\nWith unicode: 世界 🌍"
        text_file = os.path.join(self.temp_path, "test.txt")
        with open(text_file, "w", encoding="utf-8") as f:
            f.write(text_content)

        # Load the document
        loader = graphbit.DocumentLoader()
        document = loader.load_document(text_file, "txt")

        # Verify results
        assert document is not None
        assert document.source == text_file
        assert document.document_type == "txt"
        assert document.content == text_content
        assert document.file_size > 0
        assert document.extracted_at > 0
        assert not document.is_empty()
        # Note: content_length() returns byte length of the content string,
        # which may differ from character count for unicode text
        assert document.content_length() >= len(text_content)  # Unicode chars may take more bytes

        # Test metadata
        metadata = document.metadata
        assert "file_size" in metadata
        assert "file_path" in metadata

    def test_json_document_loading(self):
        """Test loading JSON documents."""
        # Create a test JSON file
        json_data = {"name": "Test Document", "type": "JSON", "data": {"values": [1, 2, 3, 4, 5], "metadata": {"created": "2024-01-01", "author": "Test User"}}}
        json_file = os.path.join(self.temp_path, "test.json")
        with open(json_file, "w", encoding="utf-8") as f:
            json.dump(json_data, f, indent=2)

        # Load the document
        loader = graphbit.DocumentLoader()
        document = loader.load_document(json_file, "json")

        # Verify results
        assert document is not None
        assert document.document_type == "json"
        assert "Test Document" in document.content
        assert "JSON" in document.content
        assert document.file_size > 0

    def test_csv_document_loading(self):
        """Test loading CSV documents."""
        # Create a test CSV file
        csv_content = "Name,Age,City\nJohn Doe,30,New York\nJane Smith,25,Los Angeles\nBob Johnson,35,Chicago"
        csv_file = os.path.join(self.temp_path, "test.csv")
        with open(csv_file, "w", encoding="utf-8") as f:
            f.write(csv_content)

        # Load the document
        loader = graphbit.DocumentLoader()
        document = loader.load_document(csv_file, "csv")

        # Verify results
        assert document is not None
        assert document.document_type == "csv"
        assert "Name,Age,City" in document.content
        assert "John Doe" in document.content
        assert document.file_size > 0

    def test_xml_document_loading(self):
        """Test loading XML documents."""
        # Create a test XML file
        xml_content = """<?xml version="1.0" encoding="UTF-8"?>
<document>
    <title>Test Document</title>
    <content>
        <paragraph>This is a test XML document.</paragraph>
        <paragraph>It contains structured data.</paragraph>
    </content>
    <metadata>
        <author>Test User</author>
        <date>2024-01-01</date>
    </metadata>
</document>"""
        xml_file = os.path.join(self.temp_path, "test.xml")
        with open(xml_file, "w", encoding="utf-8") as f:
            f.write(xml_content)

        # Load the document
        loader = graphbit.DocumentLoader()
        document = loader.load_document(xml_file, "xml")

        # Verify results
        assert document is not None
        assert document.document_type == "xml"
        assert "Test Document" in document.content
        assert "structured data" in document.content
        assert document.file_size > 0

    def test_html_document_loading(self):
        """Test loading HTML documents."""
        # Create a test HTML file
        html_content = """<!DOCTYPE html>
<html>
<head>
    <title>Test Document</title>
</head>
<body>
    <h1>Test HTML Document</h1>
    <p>This is a paragraph with <strong>bold text</strong> and <em>italic text</em>.</p>
    <ul>
        <li>List item 1</li>
        <li>List item 2</li>
    </ul>
</body>
</html>"""
        html_file = os.path.join(self.temp_path, "test.html")
        with open(html_file, "w", encoding="utf-8") as f:
            f.write(html_content)

        # Load the document
        loader = graphbit.DocumentLoader()
        document = loader.load_document(html_file, "html")

        # Verify results
        assert document is not None
        assert document.document_type == "html"
        assert "Test HTML Document" in document.content
        assert "paragraph" in document.content
        assert document.file_size > 0

    def test_empty_file_handling(self):
        """Test handling of empty files."""
        # Create an empty file
        empty_file = os.path.join(self.temp_path, "empty.txt")
        with open(empty_file, "w") as f:
            pass  # Create empty file

        # Load the document
        loader = graphbit.DocumentLoader()
        document = loader.load_document(empty_file, "txt")

        # Verify results
        assert document is not None
        assert document.content == ""
        assert document.file_size == 0
        assert document.is_empty()
        assert document.content_length() == 0

    def test_unicode_document_handling(self):
        """Test handling of Unicode content."""
        # Create a Unicode content file
        unicode_content = "Hello, 世界! 🌍 Здравствуй мир! مرحبا بالعالم! नमस्ते दुनिया!"
        unicode_file = os.path.join(self.temp_path, "unicode.txt")
        with open(unicode_file, "w", encoding="utf-8") as f:
            f.write(unicode_content)

        # Load the document
        loader = graphbit.DocumentLoader()
        document = loader.load_document(unicode_file, "txt")

        # Verify results
        assert document is not None
        assert "世界" in document.content
        assert "🌍" in document.content
        assert "мир" in document.content
        assert "العالم" in document.content
        assert "दुनिया" in document.content

    def test_large_document_handling(self):
        """Test handling of large documents."""
        # Create a large document (1MB)
        large_content = "A" * (1024 * 1024)
        large_file = os.path.join(self.temp_path, "large.txt")
        with open(large_file, "w") as f:
            f.write(large_content)

        # Load the document
        loader = graphbit.DocumentLoader()
        document = loader.load_document(large_file, "txt")

        # Verify results
        assert document is not None
        assert len(document.content) == 1024 * 1024
        assert document.file_size == 1024 * 1024

    def test_document_size_limit(self):
        """Test document size limits."""
        # Create a document larger than default limit (>10MB)
        oversized_content = "A" * (11 * 1024 * 1024)
        oversized_file = os.path.join(self.temp_path, "oversized.txt")
        with open(oversized_file, "w") as f:
            f.write(oversized_content)

        # Try to load the document
        loader = graphbit.DocumentLoader()
        with pytest.raises(Exception) as exc_info:
            loader.load_document(oversized_file, "txt")

        assert "exceeds maximum" in str(exc_info.value)

    def test_custom_size_limit(self):
        """Test custom document size limits."""
        # Create a custom config with 1KB limit
        config = graphbit.DocumentLoaderConfig(max_file_size=1024)  # 1KB
        loader = graphbit.DocumentLoader(config)

        # Create a small file that fits
        small_content = "Small content"
        small_file = os.path.join(self.temp_path, "small.txt")
        with open(small_file, "w") as f:
            f.write(small_content)

        # Should load successfully
        document = loader.load_document(small_file, "txt")
        assert document is not None

        # Create a file that exceeds the limit
        large_content = "A" * 2048  # 2KB
        large_file = os.path.join(self.temp_path, "large.txt")
        with open(large_file, "w") as f:
            f.write(large_content)

        # Should fail to load
        with pytest.raises(Exception) as exc_info:
            loader.load_document(large_file, "txt")

        assert "exceeds maximum" in str(exc_info.value)

    def test_nonexistent_file_handling(self):
        """Test handling of non-existent files."""
        loader = graphbit.DocumentLoader()

        with pytest.raises(Exception) as exc_info:
            loader.load_document("/nonexistent/path/file.txt", "txt")

        assert "not found" in str(exc_info.value).lower()

    def test_unsupported_file_type(self):
        """Test handling of unsupported file types."""
        # Create a file with unsupported extension
        unknown_file = os.path.join(self.temp_path, "test.unknown")
        with open(unknown_file, "w") as f:
            f.write("Some content")

        loader = graphbit.DocumentLoader()

        with pytest.raises(Exception) as exc_info:
            loader.load_document(unknown_file, "unknown")

        assert "unsupported" in str(exc_info.value).lower()

    def test_invalid_input_validation(self):
        """Test validation of invalid inputs."""
        loader = graphbit.DocumentLoader()

        # Test empty source path
        with pytest.raises(ValueError):
            loader.load_document("", "txt")

        # Test empty document type
        text_file = os.path.join(self.temp_path, "test.txt")
        with open(text_file, "w") as f:
            f.write("content")

        with pytest.raises(ValueError):
            loader.load_document(text_file, "")

    def test_supported_types(self):
        """Test getting supported document types."""
        supported_types = graphbit.DocumentLoader.supported_types()

        assert isinstance(supported_types, list)
        assert "txt" in supported_types
        assert "json" in supported_types
        assert "csv" in supported_types
        assert "xml" in supported_types
        assert "html" in supported_types
        assert "pdf" in supported_types
        assert "docx" in supported_types
        assert len(supported_types) >= 7

    def test_document_type_detection(self):
        """Test automatic document type detection."""
        assert graphbit.DocumentLoader.detect_document_type("file.txt") == "txt"
        assert graphbit.DocumentLoader.detect_document_type("file.json") == "json"
        assert graphbit.DocumentLoader.detect_document_type("file.csv") == "csv"
        assert graphbit.DocumentLoader.detect_document_type("file.xml") == "xml"
        assert graphbit.DocumentLoader.detect_document_type("file.html") == "html"
        assert graphbit.DocumentLoader.detect_document_type("file.pdf") == "pdf"
        assert graphbit.DocumentLoader.detect_document_type("file.docx") == "docx"
        assert graphbit.DocumentLoader.detect_document_type("file.unknown") is None

    def test_document_source_validation(self):
        """Test document source and type validation."""
        # Create a test file
        text_file = os.path.join(self.temp_path, "file.txt")
        with open(text_file, "w") as f:
            f.write("Test content")

        # Valid combinations should not raise exceptions
        graphbit.DocumentLoader.validate_document_source(text_file, "txt")

        # Invalid type should raise exception
        with pytest.raises(Exception):
            graphbit.DocumentLoader.validate_document_source(text_file, "unknown")

        # Non-existent file should raise exception
        with pytest.raises(Exception):
            graphbit.DocumentLoader.validate_document_source("/nonexistent/file.txt", "txt")

    def test_document_content_methods(self):
        """Test DocumentContent methods and properties."""
        # Create a test file
        content = "Test content for methods"
        text_file = os.path.join(self.temp_path, "methods_test.txt")
        with open(text_file, "w") as f:
            f.write(content)

        # Load the document
        loader = graphbit.DocumentLoader()
        document = loader.load_document(text_file, "txt")

        # Test content methods
        assert document.content_length() >= len(content)  # May be larger due to unicode encoding
        assert not document.is_empty()

        # Test preview method
        preview = document.preview(10)
        assert len(preview) <= 10 + 3  # +3 for "..."

        full_preview = document.preview(1000)  # Larger than content
        assert full_preview == content  # Should return full content

    def test_document_metadata_access(self):
        """Test accessing document metadata."""
        # Create a test file
        content = "Test content for metadata"
        text_file = os.path.join(self.temp_path, "metadata_test.txt")
        with open(text_file, "w") as f:
            f.write(content)

        # Load the document
        loader = graphbit.DocumentLoader()
        document = loader.load_document(text_file, "txt")

        # Test metadata access
        metadata = document.metadata
        assert isinstance(metadata, dict)
        assert "file_size" in metadata
        assert "file_path" in metadata

        # Test property access
        assert document.source == text_file
        assert document.document_type == "txt"
        assert document.content == content
        assert document.file_size == len(content)
        assert document.extracted_at > 0

    def test_document_repr(self):
        """Test document string representations."""
        # Test config repr
        config = graphbit.DocumentLoaderConfig()
        config_repr = repr(config)
        assert "DocumentLoaderConfig" in config_repr
        assert "max_file_size" in config_repr

        # Test loader repr
        loader = graphbit.DocumentLoader()
        loader_repr = repr(loader)
        assert "DocumentLoader" in loader_repr
        assert "supported_types" in loader_repr

        # Test document content repr
        content = "Test content"
        text_file = os.path.join(self.temp_path, "repr_test.txt")
        with open(text_file, "w") as f:
            f.write(content)

        document = loader.load_document(text_file, "txt")
        document_repr = repr(document)
        assert "DocumentContent" in document_repr
        assert "txt" in document_repr
        assert "bytes" in document_repr

    def test_extraction_settings(self):
        """Test custom extraction settings."""
        # Create config with extraction settings
        config = graphbit.DocumentLoaderConfig()
        settings = {"custom_setting": "value", "number_setting": 42}
        config.extraction_settings = settings

        # Verify settings are preserved
        retrieved_settings = config.extraction_settings
        assert retrieved_settings["custom_setting"] == "value"
        assert retrieved_settings["number_setting"] == 42

    def test_url_loading_invalid_format(self):
        """Test URL loading with invalid URL formats."""
        loader = graphbit.DocumentLoader()

        # Test invalid URL format (no protocol) - this will be treated as a file path
        with pytest.raises(Exception) as exc_info:
            loader.load_document("invalid-url", "txt")

        # This will fail as "file not found" since it's treated as a file path
        assert "file not found" in str(exc_info.value).lower() or "invalid url format" in str(exc_info.value).lower()

        # Test unsupported protocol
        with pytest.raises(Exception) as exc_info:
            loader.load_document("ftp://example.com/document.txt", "txt")

        assert "invalid url format" in str(exc_info.value).lower()

    @pytest.mark.skipif(not os.environ.get("TEST_REMOTE_URLS", "").lower() == "true", reason="Remote URL testing disabled (set TEST_REMOTE_URLS=true to enable)")
    def test_url_loading_remote_json(self):
        """Test loading JSON document from URL (requires internet)."""
        loader = graphbit.DocumentLoader()

        # Use a reliable JSON API endpoint
        url = "https://httpbin.org/json"

        try:
            document = loader.load_document(url, "json")

            # Verify document properties
            assert document is not None
            assert document.source == url
            assert document.document_type == "json"
            assert document.file_size > 0
            assert document.extracted_at > 0

            # Should contain valid JSON
            import json

            parsed = json.loads(document.content)
            assert isinstance(parsed, dict)

            # Check metadata
            metadata = document.metadata
            assert "url" in metadata
            assert "content_type" in metadata

        except Exception as e:
            pytest.skip(f"URL loading test skipped due to network error: {e}")

    def test_url_loading_unsupported_types(self):
        """Test URL loading with unsupported document types."""
        loader = graphbit.DocumentLoader()

        # PDF and DOCX should not be supported for URL loading yet
        with pytest.raises(Exception) as exc_info:
            loader.load_document("https://example.com/document.pdf", "pdf")

        assert "not yet supported" in str(exc_info.value).lower()

        with pytest.raises(Exception) as exc_info:
            loader.load_document("https://example.com/document.docx", "docx")

        assert "not yet supported" in str(exc_info.value).lower()


class TestDocumentLoaderPerformance:
    """Performance tests for document loading."""

    @classmethod
    def setup_class(cls):
        """Set up test environment."""
        graphbit.init(enable_tracing=False)

    def setup_method(self):
        """Set up test fixtures before each test."""
        self.temp_dir = tempfile.TemporaryDirectory()
        self.temp_path = self.temp_dir.name

    def teardown_method(self):
        """Clean up after each test."""
        self.temp_dir.cleanup()

    def test_multiple_document_loading_performance(self):
        """Test loading multiple documents for performance."""
        import time

        # Create multiple test files
        num_files = 10
        file_paths = []

        for i in range(num_files):
            content = f"This is test document number {i}.\n" * 100
            file_path = os.path.join(self.temp_path, f"test_{i}.txt")
            with open(file_path, "w") as f:
                f.write(content)
            file_paths.append(file_path)

        # Load documents and measure time
        loader = graphbit.DocumentLoader()
        start_time = time.time()

        documents = []
        for file_path in file_paths:
            document = loader.load_document(file_path, "txt")
            documents.append(document)
            assert f"test document number" in document.content

        end_time = time.time()
        duration = end_time - start_time

        # Verify all documents loaded
        assert len(documents) == num_files

        # Should complete reasonably quickly
        assert duration < 2.0, f"Loading {num_files} files took {duration:.2f}s, expected < 2.0s"

    def test_large_file_performance(self):
        """Test loading large files for performance."""
        import time

        # Create a large file (1MB)
        large_content = "Performance test content. " * (1024 * 1024 // 25)  # ~1MB
        large_file = os.path.join(self.temp_path, "large_perf.txt")
        with open(large_file, "w") as f:
            f.write(large_content)

        # Load the large document and measure time
        loader = graphbit.DocumentLoader()
        start_time = time.time()

        document = loader.load_document(large_file, "txt")

        end_time = time.time()
        duration = end_time - start_time

        # Verify document loaded correctly
        assert document is not None
        assert len(document.content) >= 1024 * 1024

        # Should complete reasonably quickly
        assert duration < 1.0, f"Loading 1MB file took {duration:.2f}s, expected < 1.0s"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
