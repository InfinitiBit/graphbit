"""Plan execution module for browser automation using Playwright."""

from playwright.sync_api import sync_playwright


def run_browser_plan(plan: list, selectors=None) -> dict:
    """
    Execute the action plan (list of steps) in Playwright.

    If selectors are provided (from initial scan), each selector is validated.
    On error, logs the problem and saves a screenshot for debugging.
    Returns a dictionary of extracted results (by key or step).
    """
    result = {}
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=False)
        page = browser.new_page()
        for idx, step in enumerate(plan):
            action = step.get("action")
            try:
                if action == "goto":
                    page.goto(step["url"])
                elif action in ("type", "wait_for", "click", "extract_text"):
                    sel = step.get("selector")
                    # Validate selector if a scan is provided
                    found = False
                    if selectors and sel:
                        # Try to match selector with those from scan
                        # Accepts CSS selector string (".headline") or dict ({'id': 'foo'})
                        if isinstance(sel, dict):
                            sel_str = None
                            if "id" in sel and sel["id"]:
                                sel_str = f"#{sel['id']}"
                            elif "class" in sel and sel["class"]:
                                sel_str = f".{sel['class']}"
                            elif "tag" in sel and sel["tag"]:
                                sel_str = sel["tag"]
                            else:
                                # Try by text (advanced)
                                if "text" in sel and sel["text"]:
                                    found = any(s.get("text") == sel["text"] for s in selectors)
                            if sel_str:
                                found = any(
                                    (s.get("id") and f"#{s['id']}" == sel_str) or (s.get("class") and f".{s['class']}" == sel_str) or (s.get("tag") and s["tag"].lower() == sel_str.lower())
                                    for s in selectors
                                )
                            if not found:
                                print(f"[plan_executor] Step {idx}: Selector {sel} not found in scan, skipping: {step}")
                                continue
                            # For Playwright, use sel_str or fallback to original dict (advanced: build CSS)
                            sel = sel_str or sel.get("text") or sel.get("value")
                        else:
                            # String selector
                            found = any((s.get("id") and f"#{s['id']}" == sel) or (s.get("class") and f".{s['class']}" == sel) or (s.get("tag") and s["tag"].lower() == sel.lower()) for s in selectors)
                            if not found:
                                print(f"[plan_executor] Step {idx}: Selector '{sel}' not found in scan, skipping: {step}")
                                continue
                    # Now run the action
                    if action == "type":
                        page.fill(sel, step["value"])
                    elif action == "wait_for":
                        page.wait_for_selector(sel, timeout=60000)
                    elif action == "click":
                        page.click(sel)
                    # elif action == "extract_text":
                    #     locator = page.locator(sel)
                    #     if locator.count() == 0:
                    #         print(f"[plan_executor] Step {idx}: Selector '{sel}' not found for extraction.")
                    #         continue
                    #     text = locator.inner_text()
                    #     key = step.get("key") or step.get("extract_to") or step.get("extract") or step.get("output") or f"step_{idx}"
                    #     result[key] = text
                    elif action == "extract_text":
                        locator = page.locator(sel)
                        count = locator.count()
                        if count == 0:
                            print(f"[plan_executor] Step {idx}: Selector '{sel}' not found for extraction.")
                            continue
                        key = step.get("key") or step.get("extract_to") or step.get("extract") or step.get("output") or f"step_{idx}"
                        if count == 1:
                            result[key] = locator.inner_text()
                        else:
                            # Multiple matches → return all texts as list
                            result[key] = locator.all_inner_texts()
                            print(f"[plan_executor] Step {idx}: Multiple matches ({count}) → returning all texts.")

            except Exception as e:
                page.screenshot(path=f"error_{action}_step{idx}.png")
                print(f"[plan_executor] Error at '{action}' (step {idx}) with {step}: {e}")
        browser.close()
    return result
