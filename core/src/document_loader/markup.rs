//! Markup document extractors (XML, HTML).

use quick_xml::events::Event;
use quick_xml::Reader;
use scraper::{Html, Selector};

use crate::errors::{GraphBitError, GraphBitResult};
use std::fmt::Write;

/// Extract content from XML files
pub async fn extract_xml_content(file_path: &str) -> GraphBitResult<String> {
    let content = std::fs::read_to_string(file_path).map_err(|e| {
        GraphBitError::validation("document_loader", format!("Failed to read XML file: {e}"))
    })?;

    match parse_xml_to_structured_text(&content) {
        Ok(structured_content) => Ok(structured_content),
        Err(_) => Ok(content),
    }
}

/// Parse XML content into structured, readable text format
pub fn parse_xml_to_structured_text(
    xml_content: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(xml_content);
    reader.config_mut().trim_text(true);

    let mut result = String::new();
    let mut buf = Vec::new();
    let mut current_path = Vec::new();
    let mut text_content = Vec::new();

    result.push_str("XML Document Content:\n\n");

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = std::str::from_utf8(e.name().as_ref())?.to_string();
                current_path.push(name.clone());

                let indent = "  ".repeat(current_path.len() - 1);
                writeln!(result, "{indent}Element: {name}").unwrap();

                for attr in e.attributes() {
                    let attr = attr?;
                    let key = std::str::from_utf8(attr.key.as_ref())?;
                    let value = std::str::from_utf8(&attr.value)?;
                    writeln!(result, "{indent}  @{key}: {value}").unwrap();
                }
            }
            Ok(Event::Text(ref e)) => {
                let unescaped_text = e.unescape()?;
                let text = unescaped_text.trim();
                if !text.is_empty() {
                    text_content.push(text.to_string());
                    let indent = "  ".repeat(current_path.len());
                    writeln!(result, "{indent}Text: {text}").unwrap();
                }
            }
            Ok(Event::CData(ref e)) => {
                let text = std::str::from_utf8(e.as_ref())?.trim();
                if !text.is_empty() {
                    text_content.push(text.to_string());
                    let indent = "  ".repeat(current_path.len());
                    writeln!(result, "{indent}Text: {text}").unwrap();
                }
            }
            Ok(Event::End(ref e)) => {
                let q_name = e.name();
                let name = std::str::from_utf8(q_name.as_ref())?;
                if current_path.last().map(|n| n == name).unwrap_or(false) {
                    current_path.pop();
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    result.push_str("\nXML parsing completed.\n");
    Ok(result)
}

/// Extract content from HTML files
pub async fn extract_html_content(file_path: &str) -> GraphBitResult<String> {
    let content = std::fs::read_to_string(file_path).map_err(|e| {
        GraphBitError::validation("document_loader", format!("Failed to read HTML file: {e}"))
    })?;

    match parse_html_to_structured_text(&content) {
        Ok(structured_content) => Ok(structured_content),
        Err(_) => Ok(content),
    }
}

/// Parse HTML content into structured, readable text format
pub fn parse_html_to_structured_text(
    html_content: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html_content);
    let mut result = String::new();

    result.push_str("HTML Document Content:\n\n");

    if let Ok(title_selector) = Selector::parse("title") {
        if let Some(title) = document.select(&title_selector).next() {
            writeln!(
                result,
                "Title: {}\n",
                title.text().collect::<String>().trim()
            )
            .unwrap();
        }
    }

    if let Ok(meta_selector) = Selector::parse("meta[name='description']") {
        if let Some(meta) = document.select(&meta_selector).next() {
            if let Some(content) = meta.value().attr("content") {
                writeln!(result, "Description: {}\n", content.trim()).unwrap();
            }
        }
    }

    for level in 1..=6 {
        let selector_str = format!("h{level}");
        if let Ok(heading_selector) = Selector::parse(&selector_str) {
            for heading in document.select(&heading_selector) {
                let text = heading.text().collect::<String>().trim().to_string();
                if !text.is_empty() {
                    let indent = "  ".repeat(level - 1);
                    writeln!(result, "{indent}H{level}: {text}").unwrap();
                }
            }
        };
    }

    if let Ok(p_selector) = Selector::parse("p") {
        result.push_str("\nParagraphs:\n");
        for paragraph in document.select(&p_selector) {
            let text = paragraph.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                writeln!(result, "  {text}\n").unwrap();
            }
        }
    }

    if let Ok(ul_selector) = Selector::parse("ul, ol") {
        result.push_str("Lists:\n");
        for list in document.select(&ul_selector) {
            let list_type = list.value().name();
            writeln!(
                result,
                "  {} List:",
                if list_type == "ul" {
                    "Unordered"
                } else {
                    "Ordered"
                }
            )
            .unwrap();

            if let Ok(li_selector) = Selector::parse("li") {
                for (index, item) in list.select(&li_selector).enumerate() {
                    let text = item.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        let prefix = if list_type == "ul" {
                            "•".to_string()
                        } else {
                            format!("{}.", index + 1)
                        };
                        writeln!(result, "    {prefix} {text}").unwrap();
                    }
                }
            }
            result.push('\n');
        }
    }

    if let Ok(a_selector) = Selector::parse("a[href]") {
        result.push_str("Links:\n");
        for link in document.select(&a_selector) {
            let text = link.text().collect::<String>().trim().to_string();
            if let Some(href) = link.value().attr("href") {
                if !text.is_empty() && !href.is_empty() {
                    writeln!(result, "  {text} -> {href}").unwrap();
                }
            }
        }
        result.push('\n');
    }

    if let Ok(table_selector) = Selector::parse("table") {
        result.push_str("Tables:\n");
        for (table_index, table) in document.select(&table_selector).enumerate() {
            writeln!(result, "  Table {}:", table_index + 1).unwrap();

            if let Ok(th_selector) = Selector::parse("th") {
                let headers: Vec<String> = table
                    .select(&th_selector)
                    .map(|th| th.text().collect::<String>().trim().to_string())
                    .filter(|h| !h.is_empty())
                    .collect();

                if !headers.is_empty() {
                    writeln!(result, "    Headers: {}", headers.join(" | ")).unwrap();
                }
            }

            if let Ok(tr_selector) = Selector::parse("tr") {
                for (row_index, row) in table.select(&tr_selector).enumerate() {
                    if let Ok(td_selector) = Selector::parse("td") {
                        let cells: Vec<String> = row
                            .select(&td_selector)
                            .map(|td| td.text().collect::<String>().trim().to_string())
                            .filter(|c| !c.is_empty())
                            .collect();

                        if !cells.is_empty() {
                            writeln!(
                                result,
                                "    Row {}: {}",
                                row_index + 1,
                                cells.join(" | ")
                            )
                            .unwrap();
                        }
                    }
                }
            }
            result.push('\n');
        }
    }

    result.push_str("HTML parsing completed.\n");
    Ok(result)
}
