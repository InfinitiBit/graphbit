import os
import logging
from dotenv import load_dotenv
from typing import Annotated, TypedDict, List

from langgraph.graph import StateGraph, END
from langgraph.graph.message import add_messages
from langchain_core.messages import BaseMessage, HumanMessage, AIMessage
from langchain_community.vectorstores import Chroma
from langchain_ollama import OllamaLLM, OllamaEmbeddings
from langchain.schema import Document
from langchain_community.document_loaders import TextLoader
from langchain.text_splitter import RecursiveCharacterTextSplitter
from langchain.prompts import PromptTemplate

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

class ChatState(TypedDict):
    messages: Annotated[List[BaseMessage], add_messages]
    session_id: str
    context: str
    query: str
    response: str

class ChatbotManager:
    def __init__(self, index_name=VECTOR_DB_INDEX_NAME):
        self.index_name = index_name
        self.llm = OllamaLLM(model="llama3.2")
        self.embeddings = OllamaEmbeddings(model="llama3.2")
        self.vectorstore = None
        
        # Initialize vector store
        if os.path.exists(self.index_name):
            self.vectorstore = Chroma(
                persist_directory=self.index_name, 
                embedding_function=self.embeddings
            )
        
        # Creating the LangGraph workflow
        self.graph = self.create_graph()
        
        # Session storage for message history
        self.sessions = {}

    def create_graph(self):
        workflow = StateGraph(ChatState)
        
        # Adding nodes to the workflow
        workflow.add_node("retrieve_context", self.retrieve_context)
        workflow.add_node("generate_response", self.generate_response)
        workflow.add_node("save_to_vectordb", self.save_to_vectordb)
        
        # Creating Graph
        workflow.set_entry_point("retrieve_context")
        workflow.add_edge("retrieve_context", "generate_response")
        workflow.add_edge("generate_response", "save_to_vectordb")
        workflow.add_edge("save_to_vectordb", END)
        
        return workflow.compile()

    def create_index(self, file_path=VECTOR_DB_TEXT_FILE):
        try:
            # Just visualization of vector data in text file
            if not os.path.exists(file_path) or os.path.getsize(file_path) == 0:
                with open(file_path, "w", encoding="utf-8") as f:
                    f.write("Conversation History:\n")

            
            # Initialize vector store from existing text file
            loader = TextLoader(file_path, encoding="utf-8")
            documents = loader.load()
            text_splitter = RecursiveCharacterTextSplitter(chunk_size=1000, chunk_overlap=100)
            docs = text_splitter.split_documents(documents)
            
            self.vectorstore = Chroma.from_documents(
                docs, 
                self.embeddings, 
                persist_directory=self.index_name
            )
            self.vectorstore.persist()
            logging.info(f"Vector store created and saved as {self.index_name}")
        except Exception as e:
            logging.error(f"Error creating vector index: {str(e)}")

    def retrieve_context(self, state: ChatState) -> ChatState:
        try:
            if self.vectorstore is None:
                logging.warning("Vector store not initialized")
                return {"context": "No vector store available"}
            
            # Get the latest user message
            query = state["query"]
            
            # Always perform similarity search
            docs = self.vectorstore.similarity_search(query, k=5)
            
            if docs:
                context = "\n\n".join([doc.page_content for doc in docs])
                logging.info(f"Retrieved {len(docs)} documents for context")
            else:
                context = "No relevant context found in vector database"
                logging.info("No documents found in similarity search")
            
            return {"context": context}
            
        except Exception as e:
            logging.error(f"Error retrieving context: {str(e)}")
            return {"context": f"Error retrieving context: {str(e)}"}

    def generate_response(self, state: ChatState) -> ChatState:
        """Generate response using LLM with context and chat history"""
        try:
            # Get chat history for this session
            session_id = state["session_id"]
            messages = state.get("messages", [])
            context = state.get("context", "")
            query = state["query"]
            
            # Format chat history
            chat_history = ""
            if messages:
                for msg in messages[-10:]:  # Last 10 messages for context
                    if isinstance(msg, HumanMessage):
                        chat_history += f"Human: {msg.content}\n"
                    elif isinstance(msg, AIMessage):
                        chat_history += f"Assistant: {msg.content}\n"
            
            # Create prompt with context and history
            prompt_template = PromptTemplate(
                template="""You are a helpful and friendly AI assistant. You can answer questions, hold normal conversations, and remember what the user has told you in this session.

You have access to external documents and chat history that you should use to enhance your answer when relevant.

Always try to:
- Understand the intent behind short or vague inputs
- Ask clarifying questions if needed
- Keep the conversation engaging and natural
- Use the chat history for personalization
- Reference the document context when it's clearly relevant

---
Document Context from Vector Database:
{context}

---
Recent Chat History:
{chat_history}

---
Current Question: {question}

Assistant:""",
                input_variables=["context", "chat_history", "question"]
            )
            
            # Generate response
            formatted_prompt = prompt_template.format(
                context=context,
                chat_history=chat_history,
                question=query
            )
            
            response = self.llm.invoke(formatted_prompt)
            
            logging.info(f"Generated response for session {session_id}")
            return {"response": response}
            
        except Exception as e:
            logging.error(f"Error generating response: {str(e)}")
            return {"response": f"Sorry, I encountered an error: {str(e)}"}

    def save_to_vectordb(self, state: ChatState) -> ChatState:
        """Save the conversation to vector database"""
        try:
            if self.vectorstore is None:
                logging.warning("Vector store not initialized, skipping save")
                return {}
            
            session_id = state["session_id"]
            query = state["query"]
            response = state["response"]
            
            # Create document with the Q&A pair
            doc_content = f"Question: {query}\nAnswer: {response}"
            doc = Document(
                page_content=doc_content,
                metadata={
                    "session_id": session_id, 
                    "type": "qa_pair", 
                    "source": "chatbot_response"
                }
            )
            
            # Save to text file
            with open(VECTOR_DB_TEXT_FILE, "a", encoding="utf-8") as f:
                f.write(f"\n{doc_content}\n")
            
            # Add to vector store
            self.vectorstore.add_documents([doc])
            self.vectorstore.persist()
            
            logging.info(f"Saved conversation to vector DB for session {session_id}")
            return {}
            
        except Exception as e:
            logging.error(f"Error saving to vector DB: {str(e)}")
            return {}

    def chat(self, session_id: str, query: str) -> str:
        """Main chat method using LangGraph"""
        try:
            # Get or create session messages
            if session_id not in self.sessions:
                self.sessions[session_id] = []
            
            # Add user message to session
            user_message = HumanMessage(content=query)
            self.sessions[session_id].append(user_message)
            
            # Create initial state
            initial_state = {
                "messages": self.sessions[session_id].copy(),
                "session_id": session_id,
                "query": query,
                "context": "",
                "response": ""
            }
            
            # Run the graph
            result = self.graph.invoke(initial_state)
            
            # Get the response
            response = result.get("response", "Sorry, I could not generate a response.")
            
            # Add AI response to session
            ai_message = AIMessage(content=response)
            self.sessions[session_id].append(ai_message)
            
            # Keep only last 20 messages per session
            if len(self.sessions[session_id]) > 20:
                self.sessions[session_id] = self.sessions[session_id][-20:]
            
            return response
            
        except Exception as e:
            logging.error(f"Error in chat: {str(e)}")
            return f"Sorry, I encountered an error: {str(e)}"

    def get_session_history(self, session_id: str) -> List[BaseMessage]:
        """Get message history for a session"""
        return self.sessions.get(session_id, [])

    def clear_session(self, session_id: str):
        """Clear a specific session"""
        if session_id in self.sessions:
            del self.sessions[session_id]
            logging.info(f"Cleared session {session_id}")
