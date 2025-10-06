"""
Streamlit frontend application for GraphBit chatbot.

This module provides a web-based chat interface using Streamlit,
with WebSocket connectivity to the FastAPI backend for real-time
conversation capabilities.
"""

import json
import time
import uuid

import requests
import streamlit as st
import websocket

BACKEND_CHAT_URL = "ws://localhost:8000/ws/chat/"

role_avatar = {
    "assistant": "🦖",
    "user": "🧑‍💻",
}

if "messages" not in st.session_state:
    st.session_state.messages = []

# --- Streamlit UI ---
st.title("GraphBit Chatbot")

if "indexed" not in st.session_state:
    st.session_state.indexed = False
if not st.session_state.indexed:
    try:
        with st.spinner("Setting up the knowledge base... Please wait."):
            response = requests.post("http://localhost:8000/index/", timeout=30)

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


with st.chat_message("assistant", avatar=role_avatar["assistant"]):
    st.write("Hello, I am your AI assistant. How can I help you today?")

for message in st.session_state.messages:
    with st.chat_message(message["role"], avatar=role_avatar.get(message["role"], "🧑")):
        st.markdown(message["content"])

# Initialize WebSocket connection
if "ws_client" not in st.session_state:
    st.session_state.ws_client = None
if "ws_connected" not in st.session_state:
    st.session_state.ws_connected = False


def connect_websocket():
    """Establish a WebSocket connection."""
    try:
        ws = websocket.create_connection(BACKEND_CHAT_URL)
        st.session_state.ws = ws
        st.session_state.ws_connected = True
        return ws
    except websocket.WebSocketException as e:
        st.error(f"Failed to connect to WebSocket: {e}")
        st.session_state.ws_connected = False
        st.session_state.ws = None
        return None


def disconnect_websocket():
    """Close the WebSocket connection."""
    if "ws" in st.session_state and st.session_state.ws:
        try:
            st.session_state.ws.close()
        except Exception as e:
            # Log or handle potential errors on close if necessary
            print(f"Error closing websocket: {e}")
        del st.session_state.ws


def send_websocket_message(message):
    """Connect, send a message, yield responses, and disconnect."""
    ws = connect_websocket()
    if ws:
        try:
            ws.send(json.dumps({"message": message, "session_id": str(uuid.uuid4())}))
            while True:
                try:
                    response = ws.recv()
                    yield response
                except websocket.WebSocketConnectionClosedException:
                    break
        finally:
            disconnect_websocket()


# Accept user input
if prompt := st.chat_input("What is up?"):
    st.session_state.messages.append({"role": "user", "content": prompt})
    st.chat_message("user", avatar=role_avatar.get("user", "🧑")).markdown(prompt)

    full_response = ""

    with st.chat_message("assistant", avatar=role_avatar["assistant"]):
        message_placeholder = st.empty()

        # Enter spinner manually so we can exit it early
        spinner = st.spinner("Thinking...")
        spinner_cm = spinner.__enter__()  # manually enter
        spinner_closed = False  # track spinner state

        try:
            for response in send_websocket_message(prompt):
                try:
                    data = json.loads(response)

                    if data.get("type") == "chunk":
                        if not spinner_closed:
                            spinner.__exit__(None, None, None)
                            spinner_closed = True

                        full_response += data.get("response", "")
                        message_placeholder.markdown(full_response + "▌")

                    elif data.get("type") == "end":
                        break

                except json.JSONDecodeError:
                    if not spinner_closed:
                        spinner.__exit__(None, None, None)
                        spinner_closed = True
                    full_response += response
                    message_placeholder.markdown(full_response + "▌")

        finally:
            if not spinner_closed:
                spinner.__exit__(None, None, None)
            message_placeholder.markdown(full_response)
            st.session_state.generating = False

    st.session_state.messages.append({"role": "assistant", "content": full_response})
