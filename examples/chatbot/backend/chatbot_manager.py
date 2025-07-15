from dataclasses import dataclass
from enum import Enum
import os
import logging
from dotenv import load_dotenv
from typing import Any, Dict, List

import graphbit
from chromadb import Client
from chromadb.config import Settings

load_dotenv()

os.makedirs("logs", exist_ok=True)
logging.basicConfig(
    filename="logs/chatbot.log",
    filemode="a",
    format="%(asctime)s - %(levelname)s - %(message)s",
    level=logging.INFO
)

VECTOR_DB_TEXT_FILE = "data/vectordb.txt"
VECTOR_DB_INDEX_NAME = "vector_index_chatbot"
CHUNK_SIZE = 1000
OVERLAP_SIZE = 100

class AgentType(Enum):
    CONTEXT_RETRIEVER = "context_retriever"
    RESPONSE_GENERATOR = "response_generator"
    MEMORY_STORAGE = "memory_storage"

@dataclass
class AgentConfig:
    name: str
    agent_type: AgentType
    prompt_template: str
    agent_id: str
    description: str = ""

class ChatbotManager:
    def __init__(self, index_name=VECTOR_DB_INDEX_NAME):
        graphbit.init()
        
        self.index_name = index_name
        
        # Configure LLM
        self.llm_config = graphbit.LlmConfig.openai(
            model="gpt-4o-mini",
            api_key=os.getenv("OPENAI_API_KEY")
        )
        self.executor = graphbit.Executor(self.llm_config)
        
        # Configure embeddings
        self.embedding_config = graphbit.EmbeddingConfig.openai(
            model="text-embedding-3-small",
            api_key=os.getenv("OPENAI_API_KEY")
        )
        self.embedding_client = graphbit.EmbeddingClient(self.embedding_config)
        
        # Initialize ChromaDB
        self.chroma_client = None
        self.collection = None
        self._init_vectorstore()
        
        # Session storage for message history
        self.sessions = {}
        
        self.agents: Dict[str, AgentConfig] = {}
        self.agent_outputs: Dict[str, Any] = {}
        self._setup_agents()

    def _init_vectorstore(self):
        try:
            self.chroma_client = Client(Settings(
                persist_directory=self.index_name,
                is_persistent=True
            ))
            if "chatbot_memory" in [c.name for c in self.chroma_client.list_collections()]:
                self.collection = self.chroma_client.get_collection(
                    name="chatbot_memory"
                )
                logging.info("Loaded existing ChromaDB collection")
            else:
                self.collection = self.chroma_client.create_collection(
                    name="chatbot_memory"
                )
                logging.info("Created new ChromaDB collection")
                
        except Exception as e:
            logging.error(f"Error initializing vector store: {str(e)}")
            self.collection = None

    def _create_index(self, file_path=VECTOR_DB_TEXT_FILE):
        try:
            os.makedirs(os.path.dirname(file_path), exist_ok=True)
            
            if not os.path.exists(file_path) or os.path.getsize(file_path) == 0:
                with open(file_path, "w", encoding="utf-8") as f:
                    f.write("Conversation History:\n")
                    f.write("This is the initial knowledge base for the chatbot.\n")
                    f.write("The chatbot can answer questions and hold conversations.\n")
            
            with open(file_path, "r", encoding="utf-8") as f:
                content = f.read()
            
            chunks = self._split_text(content, chunk_size=CHUNK_SIZE, overlap=OVERLAP_SIZE)
            
            if self.collection and chunks:
                embeddings = self.embedding_client.embed_many(chunks)
                
                for i, (chunk, embedding) in enumerate(zip(chunks, embeddings)):
                    doc_id = f"doc_{i}"
                    self.collection.add(
                        documents=[chunk],
                        embeddings=[embedding],
                        ids=[doc_id],
                        metadatas=[{"source": "initial_knowledge", "chunk_id": i}]
                    )
                
                logging.info(f"Vector store created with {len(chunks)} chunks")
            else:
                logging.warning("No content to index or collection not available")
                
        except Exception as e:
            logging.error(f"Error creating vector index: {str(e)}")
            raise

    def _split_text(self, text: str, chunk_size: int = CHUNK_SIZE, overlap: int = OVERLAP_SIZE) -> List[str]:
        chunks = []
        start = 0
        
        while start < len(text):
            end = start + chunk_size
            chunk = text[start:end]
            
            if end < len(text):
                last_space = chunk.rfind(' ')
                if last_space > chunk_size:
                    chunk = chunk[:last_space]
                    end = start + last_space
            
            chunks.append(chunk.strip())
            start = end - overlap
            
            if start >= len(text):
                break
                
        return [chunk for chunk in chunks if chunk.strip()]

    def _retrieve_context(self, query: str) -> str:
        try:
            if not self.collection:
                return "No vector store available"
            
            query_embedding = self.embedding_client.embed(query)
            
            results = self.collection.query(
                query_embeddings=[query_embedding],
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

    def _save_to_vectordb(self, query: str, session_id: str, vector_db_data: Dict[str, Any]):
        try:
            if not self.collection:
                logging.warning("Vector store not initialized, skipping save")
                return
            
            # Processed summary from memory storage agent
            if vector_db_data.get("success") and vector_db_data.get("output"):
                doc_content = vector_db_data["output"]
            else:
                doc_content = f"Question: {query}\nAnswer: No processed summary available"
            
            doc_id = f"session_{session_id}_{len(self.sessions.get(session_id, []))}"
            
            doc_embedding = self.embedding_client.embed(doc_content)
            
            with open(VECTOR_DB_TEXT_FILE, "a", encoding="utf-8") as f:
                f.write(f"\n{doc_content}\n")
            
            # Add to vector store
            self.collection.add(
                documents=[doc_content],
                embeddings=[doc_embedding],
                ids=[doc_id],
                metadatas=[{
                    "session_id": session_id,
                    "type": "processed_summary",
                    "source": "memory_storage_agent"
                }]
            )
            
            logging.info(f"Saved conversation to vector DB for session {session_id}")
            
        except Exception as e:
            logging.error(f"Error saving to vector DB: {str(e)}")

    def extract_output(self, context, fallback_name="Response Generator") -> str:
        all_results = context.get_all_variables()
        if all_results:
            for value in all_results.values():
                value_str = str(value).strip()
                if value_str and value_str.lower() not in ["null", "none", '"null"', '"none"']:
                    return value_str
        return f"{fallback_name} completed successfully, but no detailed output was captured."

    def _setup_agents(self):
        
        # Context Retriever Agent
        context_config = AgentConfig(
            name="Context Retriever",
            agent_type=AgentType.CONTEXT_RETRIEVER,
            agent_id="context_retriever",
            prompt_template="""Based on the user query: {query}
            
Retrieve and summarize the most relevant context from the knowledge base.
If no relevant context is found, respond with 'No relevant context found'.
            
Context from database: {retrieved_docs}
            
Provide a concise summary of relevant information:""",
            description="Retrieves and summarizes relevant context"
        )
        
        # Response Generator Agent
        response_config = AgentConfig(
            name="Response Generator",
            agent_type=AgentType.RESPONSE_GENERATOR,
            agent_id="response_generator",
            prompt_template="""You are a helpful and friendly AI assistant.

Document Context:
{context}

Recent Chat History:
{chat_history}

Current Question: {query}

Provide a helpful and engaging response:""",
            description="Generates helpful responses"
        )
        
        # Memory Storage Agent
        memory_config = AgentConfig(
            name="Memory Storage",
            agent_type=AgentType.MEMORY_STORAGE,
            agent_id="memory_storage",
            prompt_template="""Process this conversation exchange for storage:
            
User Query: {query}
Assistant Response: {response}

Create a concise summary for future retrieval:""",
            description="Processes conversation for storage"
        )
        
        self.agents["context_retriever"] = context_config
        self.agents["response_generator"] = response_config
        self.agents["memory_storage"] = memory_config

    def execute_agent(self, agent_id: str, input_data: Dict[str, Any]) -> Dict[str, Any]:
        if agent_id not in self.agents:
            raise ValueError(f"Agent '{agent_id}' not found")
        
        agent_config = self.agents[agent_id]
        logging.info(f"Executing agent: {agent_config.name}")
        
        try:
            formatted_prompt = agent_config.prompt_template.format(**input_data)
            
            workflow_name = agent_config.name
            workflow = graphbit.Workflow(workflow_name)
            
            agent_node = graphbit.Node.agent(
                name=agent_config.name,
                prompt=formatted_prompt,
                agent_id=agent_config.agent_id
            )
            
            workflow.add_node(agent_node)
            workflow.validate()
            
            result_context = self.executor.execute(workflow)
            output = self.extract_output(result_context, fallback_name=agent_config.name)
            
            self.agent_outputs[agent_id] = output
            
            return {
                "agent_id": agent_id,
                "agent_name": agent_config.name,
                "output": output,
                "success": True
            }
            
        except Exception as e:
            error_msg = f"Failed to execute agent {agent_config.name}: {str(e)}"
            logging.error(error_msg)
            return {
                "agent_id": agent_id,
                "agent_name": agent_config.name,
                "output": None,
                "success": False,
                "error": error_msg
            }

    async def chat(self, session_id: str, query: str) -> str:
        try:
            if session_id not in self.sessions:
                self.sessions[session_id] = []
            
            user_message = {"role": "user", "content": query}
            self.sessions[session_id].append(user_message)
            
            self.clear_agent_outputs()
            
            # Retrieve Context
            retrieved_docs = self._retrieve_context(query)
            context_result = self.execute_agent("context_retriever", {
                "query": query,
                "retrieved_docs": retrieved_docs
            })
            
            if not context_result["success"]:
                return "Sorry, I encountered an error retrieving context."
            
            context_summary = context_result["output"]
            
            # Generate Response
            chat_history = "\n".join([
                f"{msg['role'].title()}: {msg['content']}" 
                for msg in self.sessions[session_id][-5:]
            ])
            
            response_result = self.execute_agent("response_generator", {
                "query": query,
                "context": context_summary,
                "chat_history": chat_history
            })
            
            if not response_result["success"]:
                return "Sorry, I encountered an error generating a response."
            
            response = response_result["output"]
            
            # Store in Vector DB
            vector_db_data=self.execute_agent("memory_storage", {
                "query": query,
                "response": response
            })
            
            # Add AI response to session
            ai_message = {"role": "assistant", "content": response}
            self.sessions[session_id].append(ai_message)
            
            # Save to vector database
            self._save_to_vectordb(query, session_id, vector_db_data)
            
            return response
            
        except Exception as e:
            logging.error(f"Error in chat: {str(e)}")
            return f"Sorry, I encountered an error: {str(e)}"
   
    def clear_agent_outputs(self):
        self.agent_outputs.clear()
