use super::CliCommand;
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use repo::prelude::*;

pub struct WorkCommand {
    name: String,
}

impl CliCommand for WorkCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Generate work command for a repostory")
            .setting(AppSettings::Hidden)
            .arg(
                Arg::with_name("NAME")
                    .help("Name of the tracked repository to be worked on")
                    .required(true),
            )
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            name: m
                .value_of("NAME")
                .map(String::from)
                .expect("NAME is a required argument"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let workspace = Workspace::new()?;
        let repo = workspace
            .get_repository(&self.name)
            .ok_or_else(|| anyhow!("Repository: '{}' is not tracked by repo", &self.name))?;

        let path = workspace
            .config()
            .root(None)
            .join(repo.resolve_workspace_path());

        if !path.is_dir() {
            return Err(anyhow!("Could not find repository: '{}' in workspace path: '{}'. Repository needs to be cloned.", self.name, path.display()));
        }

        let mut commands = Vec::new();
        commands.push(format!("cd {}", path.display()));

        if let Some(work) = &repo.work {
            commands.push(work.clone());
        }

        println!("{}", commands.join(" && "));

        Ok(())
    }
}