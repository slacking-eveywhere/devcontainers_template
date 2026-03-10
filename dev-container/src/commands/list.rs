use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::config::{EnvConfig, ProjectPaths};
use crate::docker;

const DEFAULT_SSH_PORT: u16 = 22224;

struct ProjectRow {
    name: String,
    status: String,
    ssh_port: String,
    image: String,
}

pub fn execute() -> Result<()> {
    if let Err(e) = docker::check_docker() {
        eprintln!(
            "warning: docker is not available, container status will show as 'unknown': {}",
            e
        );
    }

    let paths = ProjectPaths::resolve("")?;
    let projects_dir = &paths.projects_top_dir;

    let mut projects: Vec<PathBuf> = Vec::new();

    if projects_dir.is_dir() {
        let entries = std::fs::read_dir(projects_dir).with_context(|| {
            format!(
                "failed to read projects directory: {}",
                projects_dir.display()
            )
        })?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                projects.push(path);
            }
        }
    }

    projects.sort();

    let running_map = build_running_map();

    let mut rows: Vec<ProjectRow> = Vec::new();

    for project_path in &projects {
        let name = match project_path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };

        let env_file = project_path.join(".env");

        let (image, ssh_port) = if env_file.exists() {
            match EnvConfig::load(&env_file) {
                Ok(env) => {
                    let image = env
                        .devcontainer_image_name
                        .unwrap_or_else(|| "(not set)".to_string());
                    let port = env
                        .ssh_port
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| format!("{} (default)", DEFAULT_SSH_PORT));
                    (image, port)
                }
                Err(_) => ("(parse error)".to_string(), "(parse error)".to_string()),
            }
        } else {
            ("(no .env)".to_string(), "(no .env)".to_string())
        };

        let status = running_map
            .get(&name)
            .cloned()
            .unwrap_or_else(|| "stopped".to_string());

        rows.push(ProjectRow {
            name,
            status,
            ssh_port,
            image,
        });
    }

    print_table(&rows);

    Ok(())
}

fn build_running_map() -> HashMap<String, String> {
    let mut map = HashMap::new();

    let json = match docker::compose_ls_json() {
        Ok(j) => j,
        Err(_) => return map,
    };

    parse_compose_ls_json(&json, &mut map);

    map
}

fn parse_compose_ls_json(json: &str, map: &mut HashMap<String, String>) {
    let json = json.trim();

    if !json.starts_with('[') {
        return;
    }

    let inner = json.trim_start_matches('[').trim_end_matches(']');

    for raw_object in split_json_objects(inner) {
        if let (Some(name), Some(status)) = (
            extract_json_string_field(raw_object, "Name"),
            extract_json_string_field(raw_object, "Status"),
        ) {
            map.insert(name, status);
        }
    }
}

fn split_json_objects(s: &str) -> Vec<&str> {
    let mut objects = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;
    let mut in_string = false;
    let mut escape_next = false;
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if escape_next {
            escape_next = false;
            continue;
        }
        match b {
            b'\\' if in_string => escape_next = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            }
            b'}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    objects.push(&s[start..=i]);
                }
            }
            _ => {}
        }
    }

    objects
}

fn extract_json_string_field(object: &str, field: &str) -> Option<String> {
    let needle = format!("\"{}\"", field);
    let pos = object.find(&needle)?;
    let after_key = &object[pos + needle.len()..];

    let colon_pos = after_key.find(':')?;
    let after_colon = after_key[colon_pos + 1..].trim_start();

    if !after_colon.starts_with('"') {
        return None;
    }

    let content = &after_colon[1..];
    let mut result = String::new();
    let mut chars = content.chars();
    let mut escape_next = false;

    for c in chars.by_ref() {
        if escape_next {
            match c {
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                'n' => result.push('\n'),
                't' => result.push('\t'),
                'r' => result.push('\r'),
                other => {
                    result.push('\\');
                    result.push(other);
                }
            }
            escape_next = false;
        } else if c == '\\' {
            escape_next = true;
        } else if c == '"' {
            break;
        } else {
            result.push(c);
        }
    }

    Some(result)
}

fn print_table(rows: &[ProjectRow]) {
    const H_NAME: &str = "NAME";
    const H_STATUS: &str = "STATUS";
    const H_PORT: &str = "SSH_PORT";
    const H_IMAGE: &str = "IMAGE";

    let w_name = rows
        .iter()
        .map(|r| r.name.len())
        .max()
        .unwrap_or(0)
        .max(H_NAME.len());

    let w_status = rows
        .iter()
        .map(|r| r.status.len())
        .max()
        .unwrap_or(0)
        .max(H_STATUS.len());

    let w_port = rows
        .iter()
        .map(|r| r.ssh_port.len())
        .max()
        .unwrap_or(0)
        .max(H_PORT.len());

    let w_image = rows
        .iter()
        .map(|r| r.image.len())
        .max()
        .unwrap_or(0)
        .max(H_IMAGE.len());

    let separator = format!(
        "+-{}-+-{}-+-{}-+-{}-+",
        "-".repeat(w_name),
        "-".repeat(w_status),
        "-".repeat(w_port),
        "-".repeat(w_image),
    );

    println!("{}", separator);
    println!(
        "| {:<w_name$} | {:<w_status$} | {:<w_port$} | {:<w_image$} |",
        H_NAME,
        H_STATUS,
        H_PORT,
        H_IMAGE,
        w_name = w_name,
        w_status = w_status,
        w_port = w_port,
        w_image = w_image,
    );
    println!("{}", separator);

    if rows.is_empty() {
        let total_inner = w_name + w_status + w_port + w_image + 9;
        println!("|{:^width$}|", "(no projects)", width = total_inner);
    } else {
        for row in rows {
            println!(
                "| {:<w_name$} | {:<w_status$} | {:<w_port$} | {:<w_image$} |",
                row.name,
                row.status,
                row.ssh_port,
                row.image,
                w_name = w_name,
                w_status = w_status,
                w_port = w_port,
                w_image = w_image,
            );
        }
    }

    println!("{}", separator);
}
