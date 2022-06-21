use std::collections::HashMap;

use anyhow::Context;
use clap::Parser;
use colorize::AnsiColor;
use serde::Deserialize;

#[derive(Deserialize)]
struct Template {
    label: String,
    description: String,
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
    let templates_file = std::fs::read_to_string("./commit-templates.toml")
        .context("failed to find `commit-templates.toml` file")?;
    let templates: HashMap<String, Template> =
        toml::from_str(&templates_file).context("failed to deserialize templates file")?;

    let template = if let Some(template) = templates.get(&app.template) {
        template
    } else {
        print_error(format!("{} does not exist", app.template));
        std::process::exit(1);
    };

    let mut title = template.title.clone();
    let mut body = template.body.clone();
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
    }

    let commit_msg = format!("{title}\n\n{body}");

    Ok(())
}

fn print_error(msg: impl Into<String>) {
    eprintln!(
        "{}{} {}",
        "[ERROR]".b_red().bold(),
        ":".b_red(),
        msg.into().b_red()
    );
}

fn print_warning(msg: impl Into<String>) {
    eprintln!(
        "{}{} {}",
        "[WARN]".b_yellow().bold(),
        ":".b_yellow(),
        msg.into().b_yellow()
    );
}

fn print_hint(msg: impl Into<String>) {
    eprintln!(
        "{}{} {}",
        "[HINT]".b_cyan().bold(),
        ":".b_cyan(),
        msg.into().b_cyan()
    );
}
