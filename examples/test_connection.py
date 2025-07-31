"""
This script is used to test the connection to the database.

It uses the psycopg2 library to connect to the database.
It uses the DB_NAME, DB_USER, DB_PASSWORD, DB_HOST environment variables to connect to the database.
It uses the DB_PORT environment variable to connect to the database.
It uses the DB_NAME, DB_USER, DB_PASSWORD, DB_HOST environment variables to connect to the database.
"""

import os

import psycopg2

# Define the connection details
dbname = os.getenv("DB_NAME")  # Replace with your actual database name
user = os.getenv("DB_USER")  # Your PostgreSQL username
password = os.getenv("DB_PASSWORD")  # Your PostgreSQL password
host = os.getenv("DB_HOST")  # Public IP address (or use private IP if applicable)
port = "5432"  # Default PostgreSQL port

# Establish connection with AlloyDB
try:
    conn = psycopg2.connect(
        dbname=dbname,
        user=user,
        password=password,
        host=host,
        port=port,
    )
    cur = conn.cursor()
    print("Connection successful")
    # Your queries or logic go here

except psycopg2.OperationalError as e:
    print(f"Error: {e}")
