"""
Update README.md with data from a Google Sheet.

Fetch CSV data from the Google Sheet, convert it to a Markdown table,
and update the README.md file.
"""

import requests

CSV_URL = "https://docs.google.com/spreadsheets/d/1deQk0p7cCJUeeZw3t8FimxVg4jc99w0Bw1XQyLPN0Zk/export?format=csv&gid=0"
README_PATH = "/home/infinitibit/graphbit/README.md"


def fetch_csv():
    """Fetch CSV data from the Google Sheet."""
    r = requests.get(CSV_URL)
    r.raise_for_status()
    return r.text


def csv_to_md_table(csv_text):
    """Convert CSV data to a Markdown table."""
    lines = csv_text.strip().split("\n")
    rows = [line.split(",") for line in lines]
    header = rows[0]
    md = []
    md.append("| " + " | ".join(header) + " |")
    md.append("|" + "|".join(["---"] * len(header)) + "|")
    for row in rows[1:]:
        md.append("| " + " | ".join(row) + " |")
    return "\n".join(md)


def update_readme(md_table):
    """Update README.md file with the Markdown table."""
    with open(README_PATH, "r") as f:
        content = f.read()
    start_marker = "<!-- START TABLE -->"
    end_marker = "<!-- END TABLE -->"
    before = content.split(start_marker)[0]
    after = content.split(end_marker)[-1]
    new_content = before + start_marker + "\n" + md_table + "\n" + end_marker + after
    with open(README_PATH, "w") as f:
        f.write(new_content)


def main():
    """Run the script."""
    csv_data = fetch_csv()
    md_table = csv_to_md_table(csv_data)
    update_readme(md_table)


if __name__ == "__main__":
    main()
