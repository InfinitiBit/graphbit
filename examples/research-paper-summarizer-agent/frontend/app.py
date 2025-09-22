"""
Streamlit frontend application for GraphBit Research Paper Summarizer.

This module provides a web-based interface for uploading research papers,
viewing section-wise summaries, and asking questions about the content
using GraphBit's AI capabilities.
"""

import streamlit as st
import requests
import time

BACKEND_URL = "http://localhost:8000"  # Change if your backend runs elsewhere

st.set_page_config(
    page_title="Research Paper Summarizer",
    page_icon="📄",
    layout="wide"
)

st.title("📄 Research Paper Summarizer & Q&A")
st.markdown("*Powered by GraphBit Framework*")

# Add sidebar with information
with st.sidebar:
    st.header("ℹ️ About")
    st.markdown("""
    This application uses GraphBit framework to:
    - Extract and summarize research paper sections
    - Create semantic embeddings for content search
    - Answer questions about the paper content
    - Cache processed papers for faster access
    """)

    st.header("🚀 How to Use")
    st.markdown("""
    1. Upload a PDF research paper
    2. Wait for processing and summarization
    3. Review section-wise summaries
    4. Ask questions about the content
    """)

    # Add stats if available
    try:
        stats_response = requests.get(f"{BACKEND_URL}/stats/", timeout=5)
        if stats_response.status_code == 200:
            stats = stats_response.json()["stats"]
            st.header("📊 Stats")
            st.metric("Active Sessions", stats.get("active_sessions", 0))
            st.text(f"LLM Model: {stats.get('llm_model', 'N/A')}")
            st.text(f"Embedding Model: {stats.get('embedding_model', 'N/A')}")
    except:
        pass

# State management for session_id and summaries
if "session_id" not in st.session_state:
    st.session_state["session_id"] = None
if "summaries" not in st.session_state:
    st.session_state["summaries"] = None
if "last_uploaded_file_name" not in st.session_state:
    st.session_state["last_uploaded_file_name"] = None


# Main content area
col1, col2 = st.columns([2, 1])

with col1:
    # Upload PDF
    st.header("📤 Upload Your Research Paper")
    uploaded_file = st.file_uploader(
        "Choose a PDF file",
        type=["pdf"],
        help="Upload a research paper in PDF format for analysis and summarization"
    )

    if uploaded_file is not None:
        if uploaded_file.name != st.session_state["last_uploaded_file_name"]:
            # Create progress indicators
            progress_bar = st.progress(0)
            status_text = st.empty()

            try:
                status_text.text("📄 Uploading PDF...")
                progress_bar.progress(10)

                files = {"file": (uploaded_file.name, uploaded_file, "application/pdf")}

                status_text.text("🔄 Processing PDF and extracting text...")
                progress_bar.progress(30)

                status_text.text("📝 Generating section summaries (this may take 1-2 minutes)...")
                progress_bar.progress(50)

                response = requests.post(f"{BACKEND_URL}/upload/", files=files, timeout=180)

                if response.status_code == 200:
                    status_text.text("🧩 Creating text chunks and embeddings...")
                    progress_bar.progress(80)

                    data = response.json()
                    st.session_state["session_id"] = data["session_id"]
                    st.session_state["summaries"] = data["summaries"]
                    st.session_state["last_uploaded_file_name"] = uploaded_file.name

                    progress_bar.progress(100)
                    status_text.text("✅ Processing complete!")

                    # Clear progress indicators after a short delay
                    time.sleep(1)
                    progress_bar.empty()
                    status_text.empty()

                    st.success("✅ Summary generated successfully!")
                    st.balloons()
                else:
                    progress_bar.empty()
                    status_text.empty()
                    error_detail = response.json().get("detail", "Unknown error")
                    st.error(f"❌ Error processing PDF: {error_detail}")

            except requests.exceptions.Timeout:
                progress_bar.empty()
                status_text.empty()
                st.error("⏰ Request timed out. The PDF is taking longer than expected to process.")
                st.info("💡 **Tips to reduce processing time:**")
                st.info("• Try a smaller PDF (< 20 pages)")
                st.info("• Ensure the PDF has clear text (not scanned images)")
                st.info("• Wait a moment and try again - the server may be busy")

            except requests.exceptions.RequestException as e:
                progress_bar.empty()
                status_text.empty()
                st.error(f"🔌 Connection error: {e}")
                st.info("Please make sure the backend server is running on http://localhost:8000")

with col2:
    if st.session_state.get("session_id"):
        st.header("📋 Session Info")
        st.info(f"**Session ID:** `{st.session_state['session_id'][:8]}...`")
        st.info(f"**File:** {st.session_state.get('last_uploaded_file_name', 'N/A')}")

        # Add clear session button
        if st.button("🗑️ Clear Session", help="Clear current session and start fresh"):
            try:
                response = requests.delete(f"{BACKEND_URL}/sessions/{st.session_state['session_id']}/")
                if response.status_code == 200:
                    st.session_state.clear()
                    st.success("Session cleared!")
                    st.rerun()
                else:
                    st.warning("Could not clear session on server")
            except:
                st.session_state.clear()
                st.success("Session cleared locally!")
                st.rerun()

# Display section-wise summaries in preferred order
section_order = [
    "Abstract", "Introduction", "Background", "Related Work",
    "Methods", "Methodology", "Experiment", "Results", "Discussion",
    "Conclusion", "References", "Acknowledgments"
]

if st.session_state.get("summaries"):
    st.header("📖 Section-wise Summaries")
    summaries = st.session_state["summaries"]

    # Create tabs for better organization
    if len(summaries) > 3:
        # Use tabs for many sections
        tab_names = []
        tab_contents = []

        # Add ordered sections first
        for section in section_order:
            if section in summaries:
                tab_names.append(section)
                tab_contents.append(summaries[section])

        # Add any remaining sections
        for section in summaries:
            if section not in section_order:
                tab_names.append(section)
                tab_contents.append(summaries[section])

        if tab_names:
            tabs = st.tabs(tab_names)
            for tab, content in zip(tabs, tab_contents):
                with tab:
                    st.write(content)
    else:
        # Use expandable sections for fewer sections
        for section in section_order:
            if section in summaries:
                with st.expander(f"📄 {section}", expanded=True):
                    st.write(summaries[section])

        # Show any remaining sections
        for section in summaries:
            if section not in section_order:
                with st.expander(f"📄 {section}", expanded=True):
                    st.write(summaries[section])

# Q&A Section
if "chat_history" not in st.session_state:
    st.session_state["chat_history"] = []

if st.session_state.get("session_id"):
    st.header("💬 Q&A about the Paper")

    # Display chat history in a more visually appealing way
    if st.session_state["chat_history"]:
        st.subheader("📝 Conversation History")
        for i, (q, a) in enumerate(st.session_state["chat_history"]):
            with st.container():
                col1, col2 = st.columns([1, 10])
                with col1:
                    st.markdown("🧑‍💻")
                with col2:
                    st.markdown(f"**Question {i+1}:** {q}")

                col1, col2 = st.columns([1, 10])
                with col1:
                    st.markdown("🤖")
                with col2:
                    st.markdown(f"**Answer:** {a}")
                st.divider()

    st.subheader("❓ Ask a New Question")

    # Provide example questions
    with st.expander("💡 Example Questions", expanded=False):
        example_questions = [
            "What is the main contribution of this paper?",
            "What methodology was used in this research?",
            "What are the key findings and results?",
            "What are the limitations of this study?",
            "How does this work compare to previous research?",
            "What future work is suggested?"
        ]
        for eq in example_questions:
            if st.button(f"📋 {eq}", key=f"example_{eq[:20]}"):
                st.session_state["example_query"] = eq

    # Chat input using a form
    with st.form(key="chat_form", clear_on_submit=True):
        # Use example query if selected
        default_query = st.session_state.get("example_query", "")
        if default_query:
            del st.session_state["example_query"]

        query = st.text_area(
            "Type your question about the paper:",
            value=default_query,
            height=100,
            placeholder="e.g., What is the main contribution of this paper?"
        )

        col1, col2, col3 = st.columns([1, 1, 3])
        with col1:
            submitted = st.form_submit_button("🚀 Ask Question", use_container_width=True)
        with col2:
            clear_history = st.form_submit_button("🗑️ Clear History", use_container_width=True)

        if clear_history:
            st.session_state["chat_history"] = []
            st.success("Chat history cleared!")
            st.rerun()

        if submitted and query.strip():
            with st.spinner("🔍 Searching paper content and generating answer..."):
                try:
                    response = requests.post(
                        f"{BACKEND_URL}/ask/",
                        data={"session_id": st.session_state["session_id"], "query": query.strip()},
                        timeout=90
                    )
                    if response.status_code == 200:
                        answer = response.json()["answer"]

                        # Display the new Q&A immediately
                        st.success("✅ Answer generated!")
                        with st.container():
                            col1, col2 = st.columns([1, 10])
                            with col1:
                                st.markdown("🧑‍💻")
                            with col2:
                                st.markdown(f"**Your Question:** {query}")

                            col1, col2 = st.columns([1, 10])
                            with col1:
                                st.markdown("🤖")
                            with col2:
                                st.markdown(f"**Answer:** {answer}")

                        # Append to chat history
                        st.session_state["chat_history"].append((query, answer))

                    elif response.status_code == 400:
                        error_detail = response.json().get("detail", "Bad request")
                        st.error(f"❌ {error_detail}")
                    else:
                        st.error("❌ Failed to retrieve answer from backend.")

                except requests.exceptions.Timeout:
                    st.error("⏰ Request timed out. Please try a simpler question or try again.")
                except requests.exceptions.RequestException as e:
                    st.error(f"🔌 Connection error: {e}")
        elif submitted:
            st.warning("⚠️ Please enter a question before submitting.")

else:
    st.info("📤 Please upload a PDF file to start asking questions about it.")
