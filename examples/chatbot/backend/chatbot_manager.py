import os
import logging
import asyncio
from dotenv import load_dotenv
from typing import Dict, List, Any
import json

import graphbit
from chromadb import Client
from chromadb.config import Settings
import chromadb.utils.embedding_functions as embedding_functions

# Load environment variables
load_dotenv()

# Set up logging
os.makedirs("logs", exist_ok=True)
logging.basicConfig(
    filename="logs/chatbot.log",
    filemode="a",
    format="%(asctime)s - %(levelname)s - %(message)s",
    level=logging.INFO
)

VECTOR_DB_TEXT_FILE = "data/vectordb.txt"
VECTOR_DB_INDEX_NAME = "vector_index_chatbot"

class ChatbotManager:
    def __init__(self, index_name=VECTOR_DB_INDEX_NAME):
        # Initialize GraphBit
        graphbit.init()
        
        self.index_name = index_name
        
        # Configure LLM (using OpenAI)
        self.llm_config = graphbit.LlmConfig.openai(
            model="gpt-4o-mini",
            api_key=os.getenv("OPENAI_API_KEY")
        )
        self.executor = graphbit.Executor(self.llm_config)
        
        # Configure embeddings (using OpenAI embeddings)
        self.embedding_config = graphbit.EmbeddingConfig.openai(
            model="text-embedding-3-small",
            api_key=os.getenv("OPENAI_API_KEY")
        )
        # self.embedding_service = graphbit.PyEmbeddingService(self.embedding_config)
        
        # Initialize ChromaDB
        self.chroma_client = None
        self.collection = None
        self._init_vectorstore()
        
        # Session storage for message history
        self.sessions = {}
        
        # Create GraphBit workflow
        self.workflow = self._create_workflow()

    def _init_vectorstore(self):
        """Initialize ChromaDB vector store"""
        try:
            # Create ChromaDB client
            self.chroma_client = Client(Settings(
                persist_directory=self.index_name,
                is_persistent=True
            ))
            
            # Get or create collection
            try:
                self.collection = self.chroma_client.get_collection(
                    name="chatbot_memory"
                )
                logging.info("Loaded existing ChromaDB collection")
            except:
                # Create new collection if it doesn't exist
                self.collection = self.chroma_client.create_collection(
                    name="chatbot_memory",
                    embedding_function=embedding_functions.OpenAIEmbeddingFunction(
                        api_key=os.getenv("OPENAI_API_KEY"),
                        model_name="text-embedding-3-small"
                    )
                )
                logging.info("Created new ChromaDB collection")
                
        except Exception as e:
            logging.error(f"Error initializing vector store: {str(e)}")
            self.collection = None

    def _create_workflow(self):
        """Create GraphBit workflow for chatbot"""
        workflow = graphbit.Workflow("Chatbot Workflow")
        
        # Context retrieval agent
        context_agent = graphbit.Node.agent(
            name="Context Retriever",
            agent_id="context_retriever",
            prompt="""Based on the user query: {query}
            
Retrieve and summarize the most relevant context from the knowledge base.
If no relevant context is found, respond with 'No relevant context found'.
            
Context from database: {retrieved_docs}
            
Provide a concise summary of relevant information:"""
        )
        
        # Response generation agent
        response_agent = graphbit.Node.agent(
            name="Response Generator",
            agent_id="response_generator",
            prompt="""You are a helpful and friendly AI assistant. You can answer questions, hold normal conversations, and remember what the user has told you in this session.

You have access to external documents and chat history that you should use to enhance your answer when relevant.

Always try to:
- Understand the intent behind short or vague inputs
- Ask clarifying questions if needed
- Keep the conversation engaging and natural
- Use the chat history for personalization
- Reference the document context when it's clearly relevant

---
Document Context:
{context}

---
Recent Chat History:
{chat_history}

---
Current Question: {query}

Provide a helpful and engaging response:"""
        )
        
        # Memory storage agent
        memory_agent = graphbit.Node.agent(
            name="Memory Storage",
            agent_id="memory_storage",
            prompt="""Process this conversation exchange for storage in the knowledge base:
            
User Query: {query}
Assistant Response: {response}

Create a concise summary that captures the key information for future retrieval:"""
        )
        
        # Add nodes to workflow
        context_id = workflow.add_node(context_agent)
        response_id = workflow.add_node(response_agent)
        memory_id = workflow.add_node(memory_agent)
        
        # Connect nodes sequentially
        workflow.connect(context_id, response_id)
        workflow.connect(response_id, memory_id)
        
        return workflow
    def create_index(self, file_path=VECTOR_DB_TEXT_FILE):
        """Create vector store index from text file"""
        try:
            # Create data directory if it doesn't exist
            os.makedirs(os.path.dirname(file_path), exist_ok=True)
            
            # Create initial text file if it doesn't exist
            if not os.path.exists(file_path) or os.path.getsize(file_path) == 0:
                with open(file_path, "w", encoding="utf-8") as f:
                    f.write("Conversation History:\n")
                    f.write("This is the initial knowledge base for the chatbot.\n")
                    f.write("The chatbot can answer questions and hold conversations.\n")
            
            # Read and process the text file
            with open(file_path, "r", encoding="utf-8") as f:
                content = f.read()
            
            # Split content into chunks
            chunks = self._split_text(content, chunk_size=1000, overlap=100)
            
            if self.collection and chunks:
                # Generate embeddings and store in ChromaDB
                for i, chunk in enumerate(chunks):
                    doc_id = f"doc_{i}"
                    self.collection.add(
                        documents=[chunk],
                        ids=[doc_id],
                        metadatas=[{"source": "initial_knowledge", "chunk_id": i}]
                    )
                
                logging.info(f"Vector store created with {len(chunks)} chunks")
            else:
                logging.warning("No content to index or collection not available")
                
        except Exception as e:
            logging.error(f"Error creating vector index: {str(e)}")
            raise

    def _split_text(self, text: str, chunk_size: int = 1000, overlap: int = 100) -> List[str]:
        """Split text into overlapping chunks"""
        chunks = []
        start = 0
        
        while start < len(text):
            end = start + chunk_size
            chunk = text[start:end]
            
            # Try to break at word boundaries
            if end < len(text):
                last_space = chunk.rfind(' ')
                if last_space > chunk_size // 2:
                    chunk = chunk[:last_space]
                    end = start + last_space
            
            chunks.append(chunk.strip())
            start = end - overlap
            
            if start >= len(text):
                break
                
        return [chunk for chunk in chunks if chunk.strip()]

    async def _retrieve_context(self, query: str) -> str:
        """Retrieve relevant context from vector database"""
        try:
            if not self.collection:
                return "No vector store available"
            
            # Query the collection
            results = self.collection.query(
                query_texts=[query],
                n_results=5
            )
            
            if results['documents'] and results['documents'][0]:
                context_docs = results['documents'][0]
                context = "\n\n".join(context_docs)
                logging.info(f"Retrieved {len(context_docs)} documents for context")
                return context
            else:
                logging.info("No documents found in similarity search")
                return "No relevant context found in vector database"
                
        except Exception as e:
            logging.error(f"Error retrieving context: {str(e)}")
            return f"Error retrieving context: {str(e)}"

    async def _save_to_vectordb(self, query: str, response: str, session_id: str):
        """Save conversation to vector database"""
        try:
            if not self.collection:
                logging.warning("Vector store not initialized, skipping save")
                return
            
            # Create document with the Q&A pair
            doc_content = f"Question: {query}\nAnswer: {response}"
            doc_id = f"session_{session_id}_{len(self.sessions.get(session_id, []))}"
            
            # Save to text file
            with open(VECTOR_DB_TEXT_FILE, "a", encoding="utf-8") as f:
                f.write(f"\n{doc_content}\n")
            
            # Add to vector store
            self.collection.add(
                documents=[doc_content],
                ids=[doc_id],
                metadatas=[{
                    "session_id": session_id,
                    "type": "qa_pair",
                    "source": "chatbot_response"
                }]
            )
            
            logging.info(f"Saved conversation to vector DB for session {session_id}")
            
        except Exception as e:
            logging.error(f"Error saving to vector DB: {str(e)}")

    def _format_chat_history(self, messages: List[Dict[str, str]]) -> str:
        """Format chat history for prompt"""
        if not messages:
            return "No previous conversation"
        
        history = ""
        for msg in messages[-10:]:  # Last 10 messages for context
            role = msg.get("role", "unknown")
            content = msg.get("content", "")
            if role == "user":
                history += f"Human: {content}\n"
            elif role == "assistant":
                history += f"Assistant: {content}\n"
        
        return history.strip()

    async def chat(self, session_id: str, query: str) -> str:
        """Main chat method using GraphBit workflow"""
        try:
            # Get or create session messages
            if session_id not in self.sessions:
                self.sessions[session_id] = []
            
            # Add user message to session
            user_message = {"role": "user", "content": query}
            self.sessions[session_id].append(user_message)
            
            # Retrieve context
            retrieved_docs = await self._retrieve_context(query)
            
            # Format chat history
            chat_history = self._format_chat_history(self.sessions[session_id][:-1])  # Exclude current message
            
            # Create workflow context with variables
            context = graphbit.PyWorkflowContext()
            context.set_variable("query", query)
            context.set_variable("retrieved_docs", retrieved_docs)
            context.set_variable("chat_history", chat_history)
            context.set_variable("context", retrieved_docs)  # For response agent
            
            # Execute workflow
            result_context = await self.executor.execute_async(self.workflow, context)
            
            # Get the response from the workflow output
            response = result_context.get_variable("output")
            if not response:
                response = "Sorry, I could not generate a response."
            
            # Add AI response to session
            ai_message = {"role": "assistant", "content": response}
            self.sessions[session_id].append(ai_message)
            
            # Keep only last 20 messages per session
            if len(self.sessions[session_id]) > 20:
                self.sessions[session_id] = self.sessions[session_id][-20:]
            
            # Save to vector database asynchronously
            asyncio.create_task(self._save_to_vectordb(query, response, session_id))
            
            return response
            
        except Exception as e:
            logging.error(f"Error in chat: {str(e)}")
            return f"Sorry, I encountered an error: {str(e)}"

    def get_session_history(self, session_id: str) -> List[Dict[str, str]]:
        """Get message history for a session"""
        return self.sessions.get(session_id, [])

    def clear_session(self, session_id: str):
        """Clear a specific session"""
        if session_id in self.sessions:
            del self.sessions[session_id]
            logging.info(f"Cleared session {session_id}")
