#!/usr/bin/env python3
"""
GraphBit Meeting Management Workflow - Standalone Script

Completely self-contained script integrating GraphBit agent framework with Google Calendar API.
No dependencies on backend/ directory - all functionality is embedded within this file.

Usage:
    python workflow_create_fetch.py
    OPERATION_CHOICE=fetch python workflow_create_fetch.py
"""

import json
import logging
import os
import sys
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional

from dotenv import load_dotenv
from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

from graphbit import Executor, LlmConfig, Node, Workflow, tool

SCRIPT_DIR = Path(__file__).parent.absolute()

ENV_FILE = SCRIPT_DIR / ".env"
if ENV_FILE.exists():
    load_dotenv(ENV_FILE)
    print(f"✓ Loaded environment variables from: {ENV_FILE}")
else:
    print(f"⚠ Warning: .env file not found at {ENV_FILE}")

OPERATION_CHOICE = os.getenv("OPERATION_CHOICE", "create")
TOKEN_FILE = SCRIPT_DIR / "token_default.json"
LOG_FILE = SCRIPT_DIR / "logs" / "workflow_create_fetch.log"

TEST_MEETING_DATA = {
    "title": "Team Sync Meeting - Workflow Test",
    "description": "Test meeting created by GraphBit workflow script.",
    "location": "Virtual - Zoom",
    "start_datetime": (datetime.now(timezone.utc) + timedelta(days=1)).replace(hour=14, minute=0, second=0, microsecond=0).isoformat(),
    "end_datetime": (datetime.now(timezone.utc) + timedelta(days=1)).replace(hour=15, minute=0, second=0, microsecond=0).isoformat(),
    "participant_emails": [],
}

FETCH_CONFIG = {"time_min": datetime.now(timezone.utc), "time_max": datetime.now(timezone.utc) + timedelta(days=7), "max_results": 10}


def setup_logging():
    log_dir = Path(LOG_FILE).parent
    log_dir.mkdir(parents=True, exist_ok=True)
    logging.basicConfig(level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s", handlers=[logging.FileHandler(LOG_FILE), logging.StreamHandler(sys.stdout)])
    logger = logging.getLogger(__name__)
    logger.info("=" * 80)
    logger.info("GraphBit Meeting Workflow Started")
    logger.info("=" * 80)
    return logger


def load_token_info() -> Optional[Dict[str, Any]]:
    token_path = Path(TOKEN_FILE)
    if not token_path.exists():
        logger.error(f"Token file not found: {TOKEN_FILE}")
        return None
    try:
        with open(token_path, "r") as f:
            token_info = json.load(f)
        return token_info.get("token_info")
    except Exception as e:
        logger.error(f"Failed to load token: {e}")
        return None


def save_token_info(token_info: Dict[str, Any]):
    try:
        token_info = {"token_info": token_info}
        with open(TOKEN_FILE, "w") as f:
            json.dump(token_info, f, indent=2)
    except Exception as e:
        logger.error(f"Failed to save token: {e}")


class GoogleCalendarService:
    """Embedded Google Calendar service for standalone operation."""

    def refresh_access_token(self, token_info: Dict[str, Any]) -> Dict[str, Any]:
        """Refresh expired access token using refresh token."""
        try:
            credentials = Credentials(
                token=token_info.get("access_token"),
                refresh_token=token_info.get("refresh_token"),
                token_uri=token_info.get("token_uri"),
                client_id=token_info.get("client_id"),
                client_secret=token_info.get("client_secret"),
                scopes=token_info.get("scopes"),
            )

            credentials.refresh(Request())

            updated_token_info = {**token_info, "access_token": credentials.token, "expiry": credentials.expiry.isoformat() if credentials.expiry else None}

            logger.info("Successfully refreshed access token")
            return updated_token_info

        except Exception as e:
            logger.error(f"Failed to refresh access token: {e}")
            raise

    def _get_calendar_service(self, token_info: Dict[str, Any]):
        """Create authenticated Google Calendar service instance."""
        try:
            if token_info.get("expiry"):
                expiry_str = token_info["expiry"]
                if expiry_str.endswith("Z"):
                    expiry_str = expiry_str.replace("Z", "+00:00")

                expiry = datetime.fromisoformat(expiry_str)
                if expiry.tzinfo is None:
                    expiry = expiry.replace(tzinfo=timezone.utc)

                now = datetime.now(timezone.utc)

                if expiry <= now:
                    logger.info("Token expired, refreshing...")
                    token_info = self.refresh_access_token(token_info)

            credentials = Credentials(
                token=token_info.get("access_token"),
                refresh_token=token_info.get("refresh_token"),
                token_uri=token_info.get("token_uri"),
                client_id=token_info.get("client_id"),
                client_secret=token_info.get("client_secret"),
                scopes=token_info.get("scopes"),
            )

            service = build("calendar", "v3", credentials=credentials)
            return service, token_info

        except Exception as e:
            logger.error(f"Failed to create calendar service: {e}")
            raise

    def create_calendar_event(self, token_info: Dict[str, Any], event_data: Dict[str, Any]) -> Dict[str, Any]:
        """Create a new event in Google Calendar."""
        try:
            service, updated_token_info = self._get_calendar_service(token_info)

            calendar_event = {
                "summary": event_data.get("title", "Meeting"),
                "description": event_data.get("description", ""),
                "start": {
                    "dateTime": event_data["start_datetime"],
                    "timeZone": "UTC",
                },
                "end": {
                    "dateTime": event_data["end_datetime"],
                    "timeZone": "UTC",
                },
            }

            if event_data.get("location"):
                calendar_event["location"] = event_data["location"]

            if event_data.get("participant_emails"):
                calendar_event["attendees"] = [{"email": email} for email in event_data["participant_emails"]]

            created_event = service.events().insert(calendarId="primary", body=calendar_event).execute()

            logger.info(f"Created Google Calendar event: {created_event.get('id')}")

            return {"success": True, "event_id": created_event.get("id"), "event_link": created_event.get("htmlLink"), "updated_token_info": updated_token_info}

        except HttpError as e:
            logger.error(f"Google Calendar API error: {e}")
            return {"success": False, "error": f"Google Calendar API error: {e}", "updated_token_info": token_info}
        except Exception as e:
            logger.error(f"Failed to create calendar event: {e}")
            return {"success": False, "error": str(e), "updated_token_info": token_info}

    def get_calendar_events(self, token_info: Dict[str, Any], time_min: datetime = None, time_max: datetime = None, max_results: int = 50) -> Dict[str, Any]:
        """Retrieve events from Google Calendar."""
        try:
            service, updated_token_info = self._get_calendar_service(token_info)

            if time_min is None:
                time_min = datetime.now(timezone.utc)
            if time_max is None:
                time_max = time_min + timedelta(days=30)

            events_result = (
                service.events().list(calendarId="primary", timeMin=time_min.isoformat(), timeMax=time_max.isoformat(), maxResults=max_results, singleEvents=True, orderBy="startTime").execute()
            )

            events = events_result.get("items", [])

            processed_events = []
            for event in events:
                processed_event = {
                    "id": event.get("id"),
                    "title": event.get("summary", "No Title"),
                    "description": event.get("description", ""),
                    "location": event.get("location", ""),
                    "start_datetime": event["start"].get("dateTime", event["start"].get("date")),
                    "end_datetime": event["end"].get("dateTime", event["end"].get("date")),
                    "attendees": [{"email": attendee.get("email"), "response_status": attendee.get("responseStatus", "needsAction")} for attendee in event.get("attendees", [])],
                    "html_link": event.get("htmlLink"),
                    "created": event.get("created"),
                    "updated": event.get("updated"),
                }
                processed_events.append(processed_event)

            logger.info(f"Retrieved {len(processed_events)} events from Google Calendar")

            return {"success": True, "events": processed_events, "updated_token_info": updated_token_info}

        except HttpError as e:
            logger.error(f"Google Calendar API error: {e}")
            return {"success": False, "error": f"Google Calendar API error: {e}", "events": [], "updated_token_info": token_info}
        except Exception as e:
            logger.error(f"Failed to retrieve calendar events: {e}")
            return {"success": False, "error": str(e), "events": [], "updated_token_info": token_info}


class MeetingWorkflow:
    """GraphBit workflow for meeting management."""

    def __init__(self, token_info: Dict[str, Any]):
        self.token_info = token_info
        self.service = GoogleCalendarService()
        self.updated_token = None

    @tool(_description="Create a new meeting in Google Calendar with specified details")
    def create_meeting(self, title: str, start_datetime: str, end_datetime: str, participant_emails: List[str] = None, description: str = None, location: str = None) -> Dict[str, Any]:
        """
        Create a new meeting in Google Calendar.

        Args:
            title: Meeting title
            start_datetime: Start time in ISO format
            end_datetime: End time in ISO format
            participant_emails: List of participant email addresses
            description: Meeting description
            location: Meeting location

        Returns:
            Dictionary with success status, event_id, and event_link
        """
        meeting_data = {
            "title": title,
            "start_datetime": start_datetime,
            "end_datetime": end_datetime,
            "participant_emails": participant_emails or [],
            "description": description,
            "location": location,
        }

        logger.info(f"Creating meeting: {title}")
        result = self.service.create_calendar_event(self.token_info, meeting_data)

        if result.get("success"):
            logger.info(f"✅ Meeting created: {result.get('event_id')}")
            self.updated_token = result.get("updated_token_info")
        else:
            logger.error(f"❌ Failed to create meeting: {result.get('error')}")

        return result

    @tool(_description="Fetch meetings from Google Calendar within a time range")
    def fetch_meetings(self, time_min: str = None, time_max: str = None, max_results: int = 10) -> Dict[str, Any]:
        """
        Fetch meetings from Google Calendar.

        Args:
            time_min: Start of time range (ISO format)
            time_max: End of time range (ISO format)
            max_results: Maximum number of events to return

        Returns:
            Dictionary with success status and list of events
        """
        logger.info(f"Fetching meetings (max: {max_results})")
        result = self.service.get_calendar_events(self.token_info, time_min=time_min, time_max=time_max, max_results=max_results)

        if result.get("success"):
            events = result.get("events", [])
            logger.info(f"✅ Retrieved {len(events)} events")
            self.updated_token = result.get("updated_token_info")
        else:
            logger.error(f"❌ Failed to fetch meetings: {result.get('error')}")

        return result


def build_graphbit_workflow(meeting_workflow: MeetingWorkflow, operation: str) -> Workflow:
    """Build GraphBit workflow with meeting tools."""
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        raise ValueError("OPENAI_API_KEY not found in environment")

    llm_config = LlmConfig.openai(api_key, "gpt-4o-mini")
    workflow = Workflow("Meeting Management")

    if operation == "create":
        prompt = f"""Create a meeting with the following details:
Title: {TEST_MEETING_DATA['title']}
Start: {TEST_MEETING_DATA['start_datetime']}
End: {TEST_MEETING_DATA['end_datetime']}
Location: {TEST_MEETING_DATA['location']}
Description: {TEST_MEETING_DATA['description']}
Participants: {', '.join(TEST_MEETING_DATA['participant_emails'])}

Use the create_meeting tool to create this meeting."""

        agent = Node.agent(name="Meeting Creator", prompt=prompt, agent_id="meeting_creator", llm_config=llm_config, tools=[meeting_workflow.create_meeting])
    else:
        time_min = FETCH_CONFIG["time_min"].isoformat()
        time_max = FETCH_CONFIG["time_max"].isoformat()
        prompt = f"""Fetch meetings from Google Calendar between {time_min} and {time_max}.
Maximum results: {FETCH_CONFIG['max_results']}

Use the fetch_meetings tool to retrieve the meetings."""

        agent = Node.agent(name="Meeting Fetcher", prompt=prompt, agent_id="meeting_fetcher", llm_config=llm_config, tools=[meeting_workflow.fetch_meetings])

    workflow.add_node(agent)
    workflow.validate()
    return workflow, llm_config


def main():
    global logger
    logger = setup_logging()

    logger.info(f"Operation: {OPERATION_CHOICE}")
    logger.info(f"Token File: {TOKEN_FILE}")

    token_info = load_token_info()
    if not token_info:
        logger.error(f"Token not found at: {TOKEN_FILE}")
        return 1

    try:
        meeting_workflow = MeetingWorkflow(token_info)
        workflow, llm_config = build_graphbit_workflow(meeting_workflow, OPERATION_CHOICE.lower())

        executor = Executor(llm_config)
        logger.info(f"Executing {OPERATION_CHOICE} operation...")

        result = executor.execute(workflow)
        agent_name = "Meeting Creator" if OPERATION_CHOICE.lower() == "create" else "Meeting Fetcher"
        response = result.get_node_output(agent_name)

        logger.info(f"\n{'=' * 80}")
        logger.info(f"Agent Response:\n{response}")
        logger.info(f"{'=' * 80}\n")

        if meeting_workflow.updated_token:
            save_token_info(meeting_workflow.updated_token)

        logger.info("✅ Workflow completed successfully")
        return 0

    except Exception as e:
        logger.error(f"❌ Workflow failed: {e}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
