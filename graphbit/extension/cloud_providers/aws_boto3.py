"""
AWS Boto3 Extension for GraphBit

This module provides integration with AWS services using Boto3.
Supports S3, DynamoDB, and other AWS services for data storage and processing.
"""

import os
from typing import Any, Dict, List, Optional, Type, Union
import logging

from ..base import BaseGraphBitExtension, ExtensionMetadata, ExtensionCategory, DependencyChecker

logger = logging.getLogger(__name__)

# Dependencies for this extension
DEPENDENCIES = ["boto3"]


def check_dependencies() -> bool:
    """Check if Boto3 dependencies are available."""
    return DependencyChecker.check_dependency("boto3")


class AWSBoto3Extension(BaseGraphBitExtension):
    """AWS Boto3 cloud provider extension for GraphBit."""

    def _get_metadata(self) -> ExtensionMetadata:
        """Return AWS Boto3 extension metadata."""
        return ExtensionMetadata(
            name="aws_boto3",
            version="1.0.0",
            description="AWS services integration for GraphBit using Boto3",
            category=ExtensionCategory.CLOUD_PROVIDER,
            dependencies=DEPENDENCIES,
            homepage="https://aws.amazon.com/",
            documentation="https://boto3.amazonaws.com/v1/documentation/api/latest/index.html"
        )

    def _get_client_class(self) -> Type:
        """Return the Boto3 session class."""
        try:
            import boto3
            return boto3.Session
        except ImportError as e:
            raise ImportError(
                "Boto3 not available. Install with: pip install graphbit[aws_boto3]"
            ) from e

    def _validate_configuration(self, config: Dict[str, Any]) -> bool:
        """Validate AWS Boto3 configuration."""
        # Boto3 can work with default AWS credentials
        return True


class AWSBoto3Client:
    """
    GraphBit wrapper for AWS Boto3 with enhanced functionality.

    This class provides a production-grade interface to AWS services with
    proper error handling, logging, and GraphBit integration.
    """

    def __init__(self, region_name: Optional[str] = None,
                 aws_access_key_id: Optional[str] = None,
                 aws_secret_access_key: Optional[str] = None,
                 aws_session_token: Optional[str] = None,
                 profile_name: Optional[str] = None, **kwargs):
        """
        Initialize AWS Boto3 client.

        Args:
            region_name: AWS region (defaults to AWS_DEFAULT_REGION env var)
            aws_access_key_id: AWS access key ID
            aws_secret_access_key: AWS secret access key
            aws_session_token: AWS session token (for temporary credentials)
            profile_name: AWS profile name
            **kwargs: Additional session configuration
        """
        self.region_name = region_name or os.getenv("AWS_DEFAULT_REGION", "us-east-1")
        self.aws_access_key_id = aws_access_key_id or os.getenv("AWS_ACCESS_KEY_ID")
        self.aws_secret_access_key = aws_secret_access_key or os.getenv("AWS_SECRET_ACCESS_KEY")
        self.aws_session_token = aws_session_token or os.getenv("AWS_SESSION_TOKEN")
        self.profile_name = profile_name
        self.kwargs = kwargs

        self._session = None
        self._clients = {}
        self._initialize_session()

    def _initialize_session(self) -> None:
        """Initialize the AWS Boto3 session."""
        try:
            import boto3

            session_kwargs = {
                "region_name": self.region_name,
                **self.kwargs
            }

            if self.profile_name:
                session_kwargs["profile_name"] = self.profile_name
            elif self.aws_access_key_id and self.aws_secret_access_key:
                session_kwargs.update({
                    "aws_access_key_id": self.aws_access_key_id,
                    "aws_secret_access_key": self.aws_secret_access_key
                })
                if self.aws_session_token:
                    session_kwargs["aws_session_token"] = self.aws_session_token

            self._session = boto3.Session(**session_kwargs)
            logger.info(f"AWS Boto3 session initialized for region: {self.region_name}")

        except Exception as e:
            logger.error(f"Failed to initialize AWS Boto3 session: {e}")
            raise

    @property
    def session(self):
        """Get the underlying Boto3 session."""
        return self._session

    def get_client(self, service_name: str, **kwargs):
        """Get or create a service client."""
        client_key = f"{service_name}_{hash(str(kwargs))}"

        if client_key not in self._clients:
            try:
                self._clients[client_key] = self._session.client(service_name, **kwargs)
                logger.info(f"Created AWS {service_name} client")
            except Exception as e:
                logger.error(f"Failed to create AWS {service_name} client: {e}")
                raise

        return self._clients[client_key]

    def get_resource(self, service_name: str, **kwargs):
        """Get a service resource."""
        try:
            resource = self._session.resource(service_name, **kwargs)
            logger.info(f"Created AWS {service_name} resource")
            return resource
        except Exception as e:
            logger.error(f"Failed to create AWS {service_name} resource: {e}")
            raise

    # S3 convenience methods
    def get_s3_client(self, **kwargs):
        """Get S3 client."""
        return self.get_client("s3", **kwargs)

    def get_s3_resource(self, **kwargs):
        """Get S3 resource."""
        return self.get_resource("s3", **kwargs)

    def upload_file_to_s3(self, file_path: str, bucket: str, key: str, **kwargs) -> bool:
        """Upload a file to S3."""
        try:
            s3_client = self.get_s3_client()
            s3_client.upload_file(file_path, bucket, key, **kwargs)
            logger.info(f"Uploaded file {file_path} to s3://{bucket}/{key}")
            return True
        except Exception as e:
            logger.error(f"Failed to upload file to S3: {e}")
            raise

    def download_file_from_s3(self, bucket: str, key: str, file_path: str, **kwargs) -> bool:
        """Download a file from S3."""
        try:
            s3_client = self.get_s3_client()
            s3_client.download_file(bucket, key, file_path, **kwargs)
            logger.info(f"Downloaded s3://{bucket}/{key} to {file_path}")
            return True
        except Exception as e:
            logger.error(f"Failed to download file from S3: {e}")
            raise

    # DynamoDB convenience methods
    def get_dynamodb_client(self, **kwargs):
        """Get DynamoDB client."""
        return self.get_client("dynamodb", **kwargs)

    def get_dynamodb_resource(self, **kwargs):
        """Get DynamoDB resource."""
        return self.get_resource("dynamodb", **kwargs)

    def put_item_dynamodb(self, table_name: str, item: Dict[str, Any], **kwargs) -> Dict:
        """Put an item in DynamoDB table."""
        try:
            dynamodb = self.get_dynamodb_resource()
            table = dynamodb.Table(table_name)
            response = table.put_item(Item=item, **kwargs)
            logger.info(f"Put item in DynamoDB table: {table_name}")
            return response
        except Exception as e:
            logger.error(f"Failed to put item in DynamoDB: {e}")
            raise

    def get_item_dynamodb(self, table_name: str, key: Dict[str, Any], **kwargs) -> Optional[Dict]:
        """Get an item from DynamoDB table."""
        try:
            dynamodb = self.get_dynamodb_resource()
            table = dynamodb.Table(table_name)
            response = table.get_item(Key=key, **kwargs)
            item = response.get('Item')
            if item:
                logger.info(f"Retrieved item from DynamoDB table: {table_name}")
            else:
                logger.info(f"No item found in DynamoDB table: {table_name}")
            return item
        except Exception as e:
            logger.error(f"Failed to get item from DynamoDB: {e}")
            raise

    def query_dynamodb(self, table_name: str, **kwargs) -> List[Dict]:
        """Query DynamoDB table."""
        try:
            dynamodb = self.get_dynamodb_resource()
            table = dynamodb.Table(table_name)
            response = table.query(**kwargs)
            items = response.get('Items', [])
            logger.info(f"Query returned {len(items)} items from DynamoDB table: {table_name}")
            return items
        except Exception as e:
            logger.error(f"Failed to query DynamoDB: {e}")
            raise

    def scan_dynamodb(self, table_name: str, **kwargs) -> List[Dict]:
        """Scan DynamoDB table."""
        try:
            dynamodb = self.get_dynamodb_resource()
            table = dynamodb.Table(table_name)
            response = table.scan(**kwargs)
            items = response.get('Items', [])
            logger.info(f"Scan returned {len(items)} items from DynamoDB table: {table_name}")
            return items
        except Exception as e:
            logger.error(f"Failed to scan DynamoDB: {e}")
            raise

    # Lambda convenience methods
    def get_lambda_client(self, **kwargs):
        """Get Lambda client."""
        return self.get_client("lambda", **kwargs)

    def invoke_lambda(self, function_name: str, payload: Optional[Dict] = None, **kwargs) -> Dict:
        """Invoke a Lambda function."""
        try:
            lambda_client = self.get_lambda_client()
            invoke_kwargs = {"FunctionName": function_name, **kwargs}
            if payload:
                import json
                invoke_kwargs["Payload"] = json.dumps(payload)

            response = lambda_client.invoke(**invoke_kwargs)
            logger.info(f"Invoked Lambda function: {function_name}")
            return response
        except Exception as e:
            logger.error(f"Failed to invoke Lambda function: {e}")
            raise


# Create extension instance
extension = AWSBoto3Extension()

# Export main classes and functions
__all__ = [
    "AWSBoto3Extension",
    "AWSBoto3Client",
    "check_dependencies",
    "DEPENDENCIES",
    "extension"
]