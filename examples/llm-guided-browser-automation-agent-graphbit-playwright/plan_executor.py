from playwright.sync_api import sync_playwright

def run_browser_plan(plan: list) -> dict:
    result = {}
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        for step in plan:
            action = step.get("action")
            if action == "goto":
                page.goto(step["url"])
            elif action == "wait_for":
                page.wait_for_selector(step["selector"], timeout=10000)
            elif action == "click":
                page.click(step["selector"])
            elif action == "extract_text":
                text = page.locator(step["selector"]).inner_text()
                result[step.get("key", "output")] = text
        browser.close()
    return result
