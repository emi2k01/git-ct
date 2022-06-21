use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use colorize::AnsiColor;
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
struct Template {
    #[allow(unused)]
    label: String,
    #[allow(unused)]
    description: String,
    branch_pattern: String,
    title: String,
    body: String,
}

#[derive(Parser)]
struct App {
    template: String,
    /// Pass variables to template using the syntax `var=value`
    #[clap(short)]
    vars: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let app = App::parse();
    let templates = read_templates_file()?;

    let template = if let Some(template) = templates.get(&app.template) {
        template
    } else {
        print_error(format!("{} does not exist", app.template));
        std::process::exit(1);
    };

    let mut title = template.title.clone();
    let mut body = template.body.clone();
    let mut branch_pattern = template.branch_pattern.clone();
    for var_pair in &app.vars {
        let mut var_pair = var_pair.split('=');
        let var = var_pair
            .next()
            .context("var must have the format `var=value`")?;
        let value = var_pair
            .next()
            .context("var must have the format `var=value`")?;
        title = title.replace(&format!("{{{{{var}}}}}"), value);
        body = body.replace(&format!("{{{{{var}}}}}"), value);
        branch_pattern = body.replace(&format!("{{{{{var}}}}}"), value);
    }

    let commit_msg = format!("{title}\n\n{body}");
    let commit_template = temp_file::with_contents(commit_msg.as_bytes());
    std::process::Command::new("git")
        .args(&[
            "commit",
            "-t",
            commit_template.path().as_os_str().to_str().unwrap(),
        ])
        .output()
        .context("failed to commit")?;

    let branch_regex = Regex::new(&branch_pattern)?;
    let branch = String::from_utf8(
        std::process::Command::new("git")
            .args(&["branch", "--show-current"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    if !branch_regex.is_match(&branch) {
        print_error(format!(
            "branch name does not match pattern `{branch_pattern}`"
        ));
        std::process::exit(1)
    }

    Ok(())
}

fn read_templates_file() -> anyhow::Result<HashMap<String, Template>> {
    let mut current_path = PathBuf::from("./").canonicalize().unwrap();
    let mut templates_file = None;
    while current_path.components().count() > 1 {
        let file = std::fs::read_to_string(current_path.join("commit-templates.toml"));
        if let Ok(file) = file {
            templates_file = Some(file);
        } else {
            current_path = current_path
                .parent()
                .context("failed to find commit-templates.toml")?
                .to_path_buf();
        }
    }

    let templates_file = if let Some(templates_file) = templates_file {
        templates_file
    } else {
        anyhow::bail!("failed to find commit-templates.toml")
    };

    toml::from_str(&templates_file).context("failed to deserialize templates file")
}

fn print_error(msg: impl Into<String>) {
    eprintln!(
        "{}{} {}",
        "[ERROR]".b_red().bold(),
        ":".b_red(),
        msg.into().b_red()
    );
}

#[allow(unused)]
fn print_warning(msg: impl Into<String>) {
    eprintln!(
        "{}{} {}",
        "[WARN]".b_yellow().bold(),
        ":".b_yellow(),
        msg.into().b_yellow()
    );
}

#[allow(unused)]
fn print_hint(msg: impl Into<String>) {
    eprintln!(
        "{}{} {}",
        "[HINT]".b_cyan().bold(),
        ":".b_cyan(),
        msg.into().b_cyan()
    );
}
