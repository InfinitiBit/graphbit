#!/usr/bin/env python3
"""
Workflow Orchestrator for GraphBit CI/CD Pipeline

This script manages the coordination between different workflow phases,
handles status propagation, and ensures proper execution order.
"""

import argparse
import json
import os
import sys
import time
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass
from enum import Enum


class WorkflowPhase(Enum):
    """Workflow execution phases."""
    VALIDATION = "validation"
    TESTING = "testing"
    BUILD = "build"
    RELEASE = "release"
    DEPLOYMENT = "deployment"


class WorkflowStatus(Enum):
    """Workflow execution status."""
    PENDING = "pending"
    RUNNING = "running"
    SUCCESS = "success"
    FAILURE = "failure"
    SKIPPED = "skipped"


@dataclass
class WorkflowResult:
    """Result of a workflow execution."""
    phase: WorkflowPhase
    status: WorkflowStatus
    start_time: datetime
    end_time: Optional[datetime] = None
    duration: Optional[float] = None
    artifacts: List[str] = None
    error_message: Optional[str] = None
    
    def __post_init__(self):
        if self.artifacts is None:
            self.artifacts = []


class WorkflowOrchestrator:
    """Orchestrates the execution of modular workflows."""
    
    def __init__(self, root_path: Path):
        self.root_path = root_path
        self.status_file = root_path / ".github" / "workflow-status.json"
        self.artifacts_dir = root_path / ".github" / "artifacts"
        self.results: Dict[WorkflowPhase, WorkflowResult] = {}
        
        # Ensure directories exist
        self.status_file.parent.mkdir(parents=True, exist_ok=True)
        self.artifacts_dir.mkdir(parents=True, exist_ok=True)
        
        # Load existing status if available
        self._load_status()
    
    def _load_status(self):
        """Load workflow status from file."""
        if self.status_file.exists():
            try:
                with open(self.status_file, 'r') as f:
                    data = json.load(f)
                
                for phase_name, result_data in data.get('results', {}).items():
                    phase = WorkflowPhase(phase_name)
                    result = WorkflowResult(
                        phase=phase,
                        status=WorkflowStatus(result_data['status']),
                        start_time=datetime.fromisoformat(result_data['start_time']),
                        end_time=datetime.fromisoformat(result_data['end_time']) if result_data.get('end_time') else None,
                        duration=result_data.get('duration'),
                        artifacts=result_data.get('artifacts', []),
                        error_message=result_data.get('error_message')
                    )
                    self.results[phase] = result
                    
            except Exception as e:
                print(f"Warning: Could not load workflow status: {e}")
    
    def _save_status(self):
        """Save workflow status to file."""
        data = {
            'last_updated': datetime.now().isoformat(),
            'results': {}
        }
        
        for phase, result in self.results.items():
            data['results'][phase.value] = {
                'status': result.status.value,
                'start_time': result.start_time.isoformat(),
                'end_time': result.end_time.isoformat() if result.end_time else None,
                'duration': result.duration,
                'artifacts': result.artifacts,
                'error_message': result.error_message
            }
        
        with open(self.status_file, 'w') as f:
            json.dump(data, f, indent=2)
    
    def start_phase(self, phase: WorkflowPhase) -> bool:
        """Start a workflow phase."""
        print(f"Starting workflow phase: {phase.value}")
        
        # Check if prerequisites are met
        if not self._check_prerequisites(phase):
            return False
        
        # Initialize result
        result = WorkflowResult(
            phase=phase,
            status=WorkflowStatus.RUNNING,
            start_time=datetime.now()
        )
        
        self.results[phase] = result
        self._save_status()
        
        print(f"Phase {phase.value} started successfully")
        return True
    
    def complete_phase(self, phase: WorkflowPhase, success: bool, 
                      artifacts: List[str] = None, error_message: str = None) -> bool:
        """Complete a workflow phase."""
        if phase not in self.results:
            print(f"Error: Phase {phase.value} was not started")
            return False
        
        result = self.results[phase]
        result.end_time = datetime.now()
        result.duration = (result.end_time - result.start_time).total_seconds()
        result.status = WorkflowStatus.SUCCESS if success else WorkflowStatus.FAILURE
        
        if artifacts:
            result.artifacts.extend(artifacts)
        
        if error_message:
            result.error_message = error_message
        
        self._save_status()
        
        status_text = "completed successfully" if success else "failed"
        print(f"Phase {phase.value} {status_text}")
        
        if not success and error_message:
            print(f"Error: {error_message}")
        
        return success
    
    def _check_prerequisites(self, phase: WorkflowPhase) -> bool:
        """Check if prerequisites for a phase are met."""
        prerequisites = {
            WorkflowPhase.VALIDATION: [],
            WorkflowPhase.TESTING: [WorkflowPhase.VALIDATION],
            WorkflowPhase.BUILD: [WorkflowPhase.VALIDATION, WorkflowPhase.TESTING],
            WorkflowPhase.RELEASE: [WorkflowPhase.VALIDATION, WorkflowPhase.TESTING, WorkflowPhase.BUILD],
            WorkflowPhase.DEPLOYMENT: [WorkflowPhase.VALIDATION, WorkflowPhase.TESTING, WorkflowPhase.BUILD, WorkflowPhase.RELEASE]
        }
        
        required_phases = prerequisites.get(phase, [])
        
        for required_phase in required_phases:
            if required_phase not in self.results:
                print(f"Error: Required phase {required_phase.value} has not been executed")
                return False
            
            if self.results[required_phase].status != WorkflowStatus.SUCCESS:
                print(f"Error: Required phase {required_phase.value} did not complete successfully")
                return False
        
        return True
    
    def get_phase_status(self, phase: WorkflowPhase) -> Optional[WorkflowStatus]:
        """Get the status of a specific phase."""
        if phase in self.results:
            return self.results[phase].status
        return None
    
    def get_phase_artifacts(self, phase: WorkflowPhase) -> List[str]:
        """Get artifacts from a specific phase."""
        if phase in self.results:
            return self.results[phase].artifacts
        return []
    
    def is_pipeline_ready_for_phase(self, phase: WorkflowPhase) -> bool:
        """Check if the pipeline is ready for a specific phase."""
        return self._check_prerequisites(phase)
    
    def get_pipeline_status(self) -> Dict:
        """Get overall pipeline status."""
        status = {
            'phases': {},
            'overall_status': 'pending',
            'completed_phases': 0,
            'total_phases': len(WorkflowPhase),
            'current_phase': None,
            'next_phase': None
        }
        
        completed_count = 0
        failed_phases = []
        running_phases = []
        
        for phase in WorkflowPhase:
            if phase in self.results:
                result = self.results[phase]
                status['phases'][phase.value] = {
                    'status': result.status.value,
                    'duration': result.duration,
                    'artifacts_count': len(result.artifacts)
                }
                
                if result.status == WorkflowStatus.SUCCESS:
                    completed_count += 1
                elif result.status == WorkflowStatus.FAILURE:
                    failed_phases.append(phase.value)
                elif result.status == WorkflowStatus.RUNNING:
                    running_phases.append(phase.value)
                    status['current_phase'] = phase.value
            else:
                status['phases'][phase.value] = {
                    'status': 'not_started',
                    'duration': None,
                    'artifacts_count': 0
                }
        
        status['completed_phases'] = completed_count
        
        # Determine overall status
        if failed_phases:
            status['overall_status'] = 'failed'
        elif running_phases:
            status['overall_status'] = 'running'
        elif completed_count == len(WorkflowPhase):
            status['overall_status'] = 'success'
        else:
            status['overall_status'] = 'pending'
        
        # Determine next phase
        if not failed_phases and not running_phases:
            for phase in WorkflowPhase:
                if phase not in self.results and self._check_prerequisites(phase):
                    status['next_phase'] = phase.value
                    break
        
        return status
    
    def reset_pipeline(self):
        """Reset the entire pipeline."""
        self.results.clear()
        if self.status_file.exists():
            self.status_file.unlink()
        print("Pipeline reset successfully")
    
    def print_status(self):
        """Print current pipeline status."""
        status = self.get_pipeline_status()
        
        print("=" * 60)
        print("WORKFLOW PIPELINE STATUS")
        print("=" * 60)
        print(f"Overall Status: {status['overall_status'].upper()}")
        print(f"Progress: {status['completed_phases']}/{status['total_phases']} phases completed")
        
        if status['current_phase']:
            print(f"Current Phase: {status['current_phase']}")
        
        if status['next_phase']:
            print(f"Next Phase: {status['next_phase']}")
        
        print("\nPhase Details:")
        for phase_name, phase_info in status['phases'].items():
            status_icon = {
                'success': '✅',
                'failure': '❌',
                'running': '🔄',
                'pending': '⏳',
                'not_started': '⚪'
            }.get(phase_info['status'], '❓')
            
            duration_text = f" ({phase_info['duration']:.1f}s)" if phase_info['duration'] else ""
            artifacts_text = f" [{phase_info['artifacts_count']} artifacts]" if phase_info['artifacts_count'] > 0 else ""
            
            print(f"  {status_icon} {phase_name}: {phase_info['status']}{duration_text}{artifacts_text}")


def main():
    """Main entry point for workflow orchestrator."""
    parser = argparse.ArgumentParser(
        description="GraphBit Workflow Orchestrator",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    
    parser.add_argument(
        'command',
        choices=['start', 'complete', 'status', 'reset', 'check'],
        help='Command to execute'
    )
    
    parser.add_argument(
        '--phase',
        choices=[p.value for p in WorkflowPhase],
        help='Workflow phase to operate on'
    )
    
    parser.add_argument(
        '--success',
        action='store_true',
        help='Mark phase as successful (for complete command)'
    )
    
    parser.add_argument(
        '--artifacts',
        nargs='*',
        help='List of artifacts produced by the phase'
    )
    
    parser.add_argument(
        '--error',
        help='Error message (for failed completion)'
    )
    
    parser.add_argument(
        '--root',
        type=Path,
        default=Path.cwd(),
        help='Root directory of the project'
    )
    
    args = parser.parse_args()
    
    orchestrator = WorkflowOrchestrator(args.root)
    
    if args.command == 'start':
        if not args.phase:
            print("Error: --phase is required for start command")
            sys.exit(1)
        
        phase = WorkflowPhase(args.phase)
        success = orchestrator.start_phase(phase)
        sys.exit(0 if success else 1)
    
    elif args.command == 'complete':
        if not args.phase:
            print("Error: --phase is required for complete command")
            sys.exit(1)
        
        phase = WorkflowPhase(args.phase)
        success = orchestrator.complete_phase(
            phase, 
            args.success, 
            args.artifacts or [], 
            args.error
        )
        sys.exit(0 if success else 1)
    
    elif args.command == 'status':
        orchestrator.print_status()
    
    elif args.command == 'reset':
        orchestrator.reset_pipeline()
    
    elif args.command == 'check':
        if not args.phase:
            print("Error: --phase is required for check command")
            sys.exit(1)
        
        phase = WorkflowPhase(args.phase)
        ready = orchestrator.is_pipeline_ready_for_phase(phase)
        
        if ready:
            print(f"Pipeline is ready for phase: {args.phase}")
            sys.exit(0)
        else:
            print(f"Pipeline is NOT ready for phase: {args.phase}")
            sys.exit(1)


if __name__ == "__main__":
    main()
