use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::string::String;

struct CommentItems {
    file: String,
    line: usize,
    text: String,
    comment_types: String,
}

// TODO: Check
fn main() {
    // TODO: Check2
    let args: Vec<String> = env::args().collect();
    let cdu: String = env::current_dir().unwrap().to_string_lossy().to_string();
    let root_path = if args.len() > 1 { &args[1] } else { &cdu };

    println!("🔍 Scanning for COMMENT items in: {}", root_path);
    println!();

    let mut items = Vec::new();

    if let Err(e) = scan_directory(Path::new(root_path), &mut items) {
        eprintln!("❌ Error scanning directory: {}", e);
        std::process::exit(1);
    }

    if items.is_empty() {
        println!("✅ No TODO items found.");
    } else {
        println!("📋 Found {} COMMENT items:\n", items.len());

        // Groupe by type
        let mut by_type: BTreeMap<String, BTreeMap<String, Vec<CommentItems>>> = BTreeMap::new();
        for item in items {
            by_type
                .entry(item.comment_types.clone())
                .or_default()
                .entry(item.file.clone())
                .or_default()
                .push(item);
        }

        //Display grouped results
        // Define custom order for comment types
        let type_order = [
            "TODO",
            "FIX",
            "NOTE",
            "HACK",
            "BUG",
            "WARNING",
            "DEPRECATED",
        ];

        for comment_type in &type_order {
            if let Some(files) = by_type.get(*comment_type) {
                println!("🏷️  {} Items:", comment_type);
                println!("   {}", "=".repeat(comment_type.len() + 7));

                for (file, file_todos) in files {
                    println!("    📁 {}", file);
                    for todo in file_todos {
                        println!("        📌 Line {}: {}", todo.line + 1, todo.text);
                    }
                    println!();
                }
            }
        }
    }
    // println!("🎉 Scan complete!");
}

// FIX: Check1
fn scan_directory(
    dir: &Path,
    todos: &mut Vec<CommentItems>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        //FIX: Check2
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            // Skip hidden files and common build directories
            if name.starts_with('.') || name == "target" || name == "node_modules" || name == ".git"
            {
                continue;
            }
        }

        // NOTE:Check
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

fn scan_file(
    file_path: &Path,
    pattern_vec: &mut Vec<CommentItems>,
) -> Result<(), Box<dyn std::error::Error>> {
    let patterns = ["//", "#", "--"];

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

                // Vec["Pattern", "Content"]
                let comment_v: Vec<&str> = without_prefix.split(":").collect();
                // println!("Comment Vec: {}", comment_v[0]);
                match comment_v[0] {
                    // Pattern: "TODO"
                    "TODO" => {
                        let content = comment_v[1].to_string();
                        if !content.is_empty() {
                            pattern_vec.push(CommentItems {
                                file: relative_path.clone(),
                                line: line_num,
                                text: content,
                                comment_types: "TODO".to_string(),
                            });
                        }
                    }
                    // Pattern: "FIX"
                    "FIX" => {
                        let content = comment_v[1].to_string();
                        if !content.is_empty() {
                            pattern_vec.push(CommentItems {
                                file: relative_path.clone(),
                                line: line_num,
                                text: content,
                                comment_types: "FIX".to_string(),
                            });
                        }
                    }
                    "NOTE" => {
                        let content = comment_v[1].to_string();
                        if !content.is_empty() {
                            pattern_vec.push(CommentItems {
                                file: relative_path.clone(),
                                line: line_num,
                                text: content,
                                comment_types: "NOTE".to_string(),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
