use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::string::String;

#[derive(Debug, Clone)]
struct Todo {
    file: String,
    line: usize,
    text: String,
}

// TODO: Check
fn main() {
    // TODO: Check2
    let args: Vec<String> = env::args().collect();
    let cdu: String = env::current_dir().unwrap().to_string_lossy().to_string();
    let root_path = if args.len() > 1 { &args[1] } else { &cdu };

    println!("🔍 Scanning for TODO items in: {}", root_path);
    println!();

    let mut todos = Vec::new();

    if let Err(e) = scan_directory(Path::new(root_path), &mut todos) {
        eprintln!("❌ Error scanning directory: {}", e);
        std::process::exit(1);
    }

    if todos.is_empty() {
        println!("✅ No TODO items found.");
    } else {
        println!("📋 Found {} TODO items:\n", todos.len());

        // Group by file
        let mut by_file: BTreeMap<String, Vec<Todo>> = BTreeMap::new();
        for todo in todos {
            by_file.entry(todo.file.clone()).or_default().push(todo);
        }

        for (file, file_todos) in by_file {
            println!("📁 {}", file);
            for todo in file_todos {
                println!("  📌 Line {} - {}", todo.line + 1, todo.text);
            }
            println!();
        }

        println!("🎉 Scan complete!");
    }
}

fn scan_directory(dir: &Path, todos: &mut Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            // Skip hidden files and common build directories
            if name.starts_with('.') || name == "target" || name == "node_modules" || name == ".git"
            {
                continue;
            }
        }

        if path.is_dir() {
            scan_directory(&path, todos)?;
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            // Check if it's a supported file type
            match ext {
                "rs" | "js" | "ts" | "jsx" | "tsx" | "py" | "java" | "go" | "c" | "cpp" | "h"
                | "hpp" | "php" | "rb" | "swift" | "kt" | "scala" | "html" | "css" | "vue"
                | "md" | "txt" | "yaml" | "yml" | "json" | "toml" | "xml" | "sh" | "bash"
                | "zsh" => {
                    scan_file(&path, todos)?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn scan_file(file_path: &Path, todos: &mut Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    let patterns = ["//", "#"];
    let comment_patterns = ["TODO", "FIX"];

    let content = fs::read_to_string(file_path)?;
    let relative_path = file_path.to_string_lossy().to_string();

    for (line_num, line) in content.lines().enumerate() {
        let line_trimmed = line.trim();
        for pt in patterns {
            if line_trimmed.starts_with(pt) {
                let without_prefix = line_trimmed
                    .strip_prefix("//")
                    .or_else(|| line.strip_prefix("#"))
                    .or_else(|| line.strip_prefix("/*"))
                    .or_else(|| line.strip_prefix("*"))
                    .unwrap_or(line)
                    .trim();
                // println!("{}", without_prefix);

                for cpt in comment_patterns {
                    if without_prefix.starts_with(cpt) {
                        // Extract "TODO"
                        if !without_prefix.is_empty() {
                            let todo_content = extract_todo_content(without_prefix);

                            todos.push(Todo {
                                file: relative_path.clone(),
                                line: line_num,
                                text: todo_content,
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn extract_todo_content(line: &str) -> String {
    // let trimmed = line.trim();

    // Find TODO and extract everything after it
    if let Some(todo_pos) = line.to_uppercase().find("TODO") {
        let after_todo = &line[todo_pos + 4..];
        let cleaned = after_todo.strip_prefix(":").unwrap_or(after_todo).trim();

        if cleaned.is_empty() {
            line.to_string()
        } else {
            format!("TODO: {}", cleaned)
        }
    } else {
        line.to_string()
    }
}
