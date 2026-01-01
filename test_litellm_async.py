import asyncio
import sys
import os
from unittest.mock import MagicMock, patch

# Add python-src to path
sys.path.append(os.path.abspath("python/python-src"))

from graphbit.providers.litellm.embeddings import LiteLLMEmbeddings
from unittest.mock import AsyncMock

async def test_aembed():
    # Mock litellm.aembedding
    mock_response = MagicMock()
    mock_response.data = [{'embedding': [0.1, 0.2, 0.3]}]
    
    # Use AsyncMock
    # Note: We are patching 'graphbit.providers.litellm.embeddings.aembedding' because the module imports it.
    with patch('graphbit.providers.litellm.embeddings.aembedding', new_callable=AsyncMock) as mock_aembedding:
        mock_aembedding.return_value = mock_response
        
        embeddings = LiteLLMEmbeddings(api_key="test")
        result = await embeddings.aembed(model="test-model", text="hello")
        
        assert result == [0.1, 0.2, 0.3]
        mock_aembedding.assert_awaited_once()
        print("Async embedding test passed with AsyncMock!")

if __name__ == "__main__":
    try:
        asyncio.run(test_aembed())
    except Exception as e:
        print(f"Test failed: {e}")
        import traceback
        traceback.print_exc()
        exit(1)
