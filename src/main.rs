use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use colorize::AnsiColor;
use inquire::{Select, Text};
use regex::Regex;
use serde::Deserialize;

#[derive(Default, Deserialize)]
struct Template {
    #[allow(unused)]
    label: String,
    #[allow(unused)]
    description: String,
    vars: Vec<String>,
    branch_pattern: String,
    title: String,
    body: String,
}

#[derive(Parser)]
struct App {
    #[clap(short)]
    template: String,
    /// Pass variables to template using the syntax `var=value`
    #[clap(short)]
    vars: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let templates = read_templates_file()?;

    let app = App::try_parse();

    let template;
    let mut vars = vec![];
    if let Ok(app) = app {
        if let Some(t) = templates.get(&app.template) {
            template = t;
        } else {
            print_error("template does not exist");
            std::process::exit(1);
        }
        vars = app.vars;
    } else {
        let labels = templates.iter().map(|t| &t.1.label).collect();
        let selected_label = Select::new("Choose a template:", labels).prompt()?;
        let selected_template = templates
            .iter()
            .find(|t| t.1.label == *selected_label)
            .map(|t| t.1);
        if let Some(t) = selected_template {
            template = t;
        } else {
            print_error("template does not exist");
            std::process::exit(1);
        }
        for var in &template.vars {
            let value = Text::new(&format!("Value for `{var}`:")).prompt()?;
            vars.push(format!("{var}={value}"));
        }
    };

    let mut title = template.title.clone();
    let mut body = template.body.clone();
    let mut branch_pattern = template.branch_pattern.clone();

    for template_var in &template.vars {
        for var_pair in &vars {
            let mut var_pair = var_pair.split('=');
            let var = var_pair
                .next()
                .context("var must have the format `var=value`")?;
            let value = var_pair
                .next()
                .context("var must have the format `var=value`")?;
            if var == template_var {
                title = title.replace(&format!("{{{{{var}}}}}"), value);
                body = body.replace(&format!("{{{{{var}}}}}"), value);
                branch_pattern = branch_pattern.replace(&format!("{{{{{var}}}}}"), value);
            }
        }
    }

    let commit_msg = format!("{title}\n\n{body}");
    let commit_template = temp_file::with_contents(commit_msg.as_bytes());
    std::process::Command::new("git")
        .args(&[
            "-c",
            "core.commentChar=;",
            "commit",
            "-t",
            commit_template.path().as_os_str().to_str().unwrap(),
            "--allow-empty-message",
        ])
        .spawn()
        .context("failed to commit")?
        .wait()?;

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
        print_warning(format!(
            "branch name does not match pattern `{branch_pattern}`"
        ));
        std::process::exit(1)
    }

    Ok(())
}

fn read_templates_file() -> anyhow::Result<HashMap<String, Template>> {
    let mut current_path = PathBuf::from("./").canonicalize().unwrap();
    println!("{}", current_path.display());
    let mut templates_file = None;
    while current_path.components().count() > 1 {
        let file = std::fs::read_to_string(current_path.join("commit-templates.toml"));
        if let Ok(file) = file {
            templates_file = Some(file);
            break;
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
