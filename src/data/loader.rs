use docx_rs::*;
use std::fs::File;
use std::io::Read;

/// Load text from a DOCX file
pub fn load_docx_text(path: &str) -> String {
    // Open file
    let mut file = File::open(path)
        .expect("Failed to open .docx file");

    // Read file into memory
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read file");

    // Parse document
    let package = read_docx(&buffer)
        .expect("Failed to parse docx");

    let mut text = String::new();

    // In docx-rs 0.4 text lives inside document children
    for child in package.document.children {
        if let DocumentChild::Paragraph(paragraph) = child {
            for run in paragraph.children {
                if let ParagraphChild::Run(run) = run {
                    for content in run.children {
                        if let RunChild::Text(t) = content {
                            text.push_str(&t.text);
                            text.push(' ');
                        }
                    }
                }
            }
        }
    }

    text
}