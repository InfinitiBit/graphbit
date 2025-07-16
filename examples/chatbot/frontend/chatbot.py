import streamlit as st
import websocket
import requests
import json
import uuid
import time

if 'indexed' not in st.session_state:
    st.session_state.indexed = False
if not st.session_state.indexed:
    try:
        with st.spinner('Setting up the knowledge base... Please wait.'):
            response = requests.post(
                "http://localhost:8000/index/",
                timeout=30
            )
        
        if response.ok:
            st.session_state.indexed = True
            message = response.json().get("message", "Indexing complete!")
            st.success(message)
            time.sleep(2)
        else:
            error_detail = response.json().get("detail", "Unknown error.")
            st.error(f"Failed to initialize knowledge base: {error_detail}")
            st.stop()

    except requests.exceptions.RequestException as e:
        st.error(f"Could not connect to the backend to initialize the knowledge base: {e}")
        st.info("Please make sure the backend server is running.")
        st.stop()

# Initialize WebSocket connection
if 'ws_client' not in st.session_state:
    st.session_state.ws_client = None
if 'ws_connected' not in st.session_state:
    st.session_state.ws_connected = False

def connect_websocket():
    try:
        ws = websocket.WebSocket()
        ws.connect("ws://localhost:8000/ws/chat/")
        st.session_state.ws_client = ws
        st.session_state.ws_connected = True
        return True
    except Exception as e:
        st.error(f"Failed to connect: {e}")
        return False

def send_websocket_message(message, session_id):
    if not st.session_state.ws_connected:
        return "Not connected to server"
    
    try:
        data = {
            "message": message,
            "session_id": session_id
        }
        st.session_state.ws_client.send(json.dumps(data))
        response = st.session_state.ws_client.recv()
        response_data = json.loads(response)
        return response_data.get("response", "No response")
    except Exception as e:
        return f"Error: {e}"

# Initialize connection
if not st.session_state.ws_connected:
    with st.spinner('Connecting to server...'):
        if not connect_websocket():
            st.stop()

st.title("GraphBit Chatbot")

if "session_id" not in st.session_state:
    st.session_state.session_id = str(uuid.uuid4())

role_avatar = {
    "assistant": "🦖",
    "user": "🧑‍💻",
}

with st.chat_message("assistant", avatar=role_avatar["assistant"]):
    st.write("Hello, I am your AI assistant. How can I help you today?")

if "messages" not in st.session_state:
    st.session_state.messages = []

for message in st.session_state.messages:
    with st.chat_message(message["role"], avatar=role_avatar.get(message["role"], "🧑")):
        st.markdown(message["content"])

if prompt := st.chat_input("Type your message here..."):
    with st.chat_message("user", avatar=role_avatar["user"]):
        st.markdown(prompt)
    st.session_state.messages.append({"role": "user", "content": prompt})

    with st.spinner('Getting response...'):
        response = send_websocket_message(prompt, st.session_state.session_id)

    with st.chat_message("assistant", avatar=role_avatar["assistant"]):
        st.markdown(response)
    
    st.session_state.messages.append({"role": "assistant", "content": response})