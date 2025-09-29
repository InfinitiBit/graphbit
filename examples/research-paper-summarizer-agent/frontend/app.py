"""
Streamlit frontend application for GraphBit Research Paper Summarizer.

This module provides a web-based interface for uploading research papers,
viewing section-wise summaries, and asking questions about the content
using GraphBit's AI capabilities.
"""

import time

import requests
import streamlit as st

BACKEND_URL = "http://localhost:8000"  # Change if your backend runs elsewhere

st.set_page_config(page_title="Research Paper Summarizer", page_icon="📄", layout="wide")

st.title("📄 Research Paper Summarizer & Q&A")
st.markdown("*Powered by GraphBit Framework*")


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
    uploaded_file = st.file_uploader("Choose a PDF file", type=["pdf"], help="Upload a research paper in PDF format for analysis and summarization")

    if uploaded_file is not None and uploaded_file.name != st.session_state["last_uploaded_file_name"]:
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
                data = response.json()
                st.session_state["session_id"] = data["session_id"]
                st.session_state["summaries"] = data["summaries"]
                st.session_state["qa_ready"] = data.get("qa_ready", False)
                st.session_state["last_uploaded_file_name"] = uploaded_file.name

                progress_bar.progress(100)
                status_text.text("✅ Summaries ready!")

                # Clear progress indicators after a short delay
                time.sleep(1)
                progress_bar.empty()
                status_text.empty()

                st.success("✅ Summaries generated successfully!")

                # Show Q&A preparation status
                if not st.session_state["qa_ready"]:
                    st.info("🔄 Preparing Q&A functionality in the background...")

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


# Display section-wise summaries in preferred order
section_order = ["Abstract", "Introduction", "Background", "Related Work", "Methods", "Methodology", "Experiment", "Results", "Discussion", "Conclusion", "References", "Acknowledgments"]

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


def check_qa_status():
    """Check if Q&A functionality is ready and detect status changes."""
    if not st.session_state.get("session_id"):
        return False, False

    # Get previous status to detect changes
    previous_qa_ready = st.session_state.get("qa_ready", False)

    # If already ready, no need to check again
    if previous_qa_ready:
        return True, False

    try:
        response = requests.get(f"{BACKEND_URL}/status/{st.session_state['session_id']}/", timeout=10)
        if response.status_code == 200:
            status = response.json()["status"]
            current_qa_ready = status.get("qa_ready", False)

            # Detect status change from False to True
            status_changed = not previous_qa_ready and current_qa_ready

            # Update session state
            st.session_state["qa_ready"] = current_qa_ready

            return current_qa_ready, status_changed
    except Exception:
        pass

    return st.session_state.get("qa_ready", False), False


if st.session_state.get("session_id"):
    st.header("💬 Q&A about the Paper")

    # Check Q&A status and detect changes
    qa_ready, status_changed = check_qa_status()

    if not qa_ready:
        st.warning("🔄 Q&A functionality is being prepared in the background. Please wait...")

        # Initialize polling state if not exists
        if "qa_polling_start_time" not in st.session_state:
            st.session_state["qa_polling_start_time"] = time.time()
            st.session_state["qa_poll_count"] = 0

        # Calculate elapsed time since polling started
        elapsed_time = time.time() - st.session_state["qa_polling_start_time"]
        st.session_state["qa_poll_count"] += 1

        # Show progress indicator with elapsed time and poll count
        poll_count = st.session_state["qa_poll_count"]
        with st.spinner(f"Preparing Q&A functionality... ({int(elapsed_time)}s elapsed, check #{poll_count})"):
            time.sleep(2)  # Brief pause for visual feedback

        # Show progress information
        if elapsed_time > 30:  # Show additional info after 30 seconds
            st.info("📊 Processing in progress... This may take 1-3 minutes depending on paper complexity.")

        # Dynamic polling with timeout (5 minutes max)
        max_polling_time = 300  # 5 minutes
        polling_interval = 8  # Check every 8 seconds

        if elapsed_time < max_polling_time:
            # Continue polling - wait for polling interval then check again
            time.sleep(polling_interval)
            st.rerun()  # Refresh to check status again
        else:
            # Timeout reached - provide manual refresh option
            st.error("⏰ Q&A preparation is taking longer than expected (5+ minutes).")
            st.info("💡 This may indicate a processing issue. Please try refreshing the page manually or contact support if the problem persists.")

            # Show manual refresh button as fallback
            if st.button("🔄 Check Q&A Status Manually"):
                # Reset polling state for manual retry
                st.session_state["qa_polling_start_time"] = time.time()
                st.session_state["qa_poll_count"] = 0
                st.rerun()
    else:
        st.success("✅ Q&A functionality is ready!")

        # If status just changed from not ready to ready, trigger one refresh to show Q&A interface
        if status_changed:
            # Calculate how long polling took for user feedback (before cleanup)
            if "qa_polling_start_time" in st.session_state:
                total_time = time.time() - st.session_state["qa_polling_start_time"]
                poll_count = st.session_state.get("qa_poll_count", 0)
                st.info(f"🎉 Q&A preparation completed in {int(total_time)} seconds after {poll_count} status checks!")
            st.rerun()

        # Clean up polling state when Q&A becomes ready
        polling_cleanup_keys = ["qa_polling_start_time", "qa_poll_count"]
        for key in polling_cleanup_keys:
            if key in st.session_state:
                del st.session_state[key]

    # Only show Q&A interface if ready
    if qa_ready:
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

        # Chat input using a form
        with st.form(key="chat_form", clear_on_submit=True):
            query = st.text_area("Type your question about the paper:", height=100, placeholder="e.g., What is the main contribution of this paper?")

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
                        response = requests.post(f"{BACKEND_URL}/ask/", data={"session_id": st.session_state["session_id"], "query": query.strip()}, timeout=90)
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
