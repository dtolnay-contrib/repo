use crate::{config::Config, git, util, Cache, Location, Repository, Tag};
use anyhow::{anyhow, Context, Result};
use std::{collections::HashMap, io::Write};

#[derive(Debug)]
pub struct Workspace {
    config: Config,
    cache: Cache,
}

impl Workspace {
    pub fn new() -> Result<Self> {
        let config = Config::new()?;
        let cache = Cache::new()?;

        Ok(Self { config, cache })
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Adds a repository to the cache
    /// Adding a repository to the cache will also write the repository to disk
    pub fn add_repository(&mut self, repository: Repository) -> Result<()> {
        if self.has_repository(&repository.name) {
            return Err(anyhow!(
                "Repository: {} already exist in repo",
                repository.name
            ));
        }

        self.write_repository(&repository)?;
        self.cache.add_repository(repository);

        Ok(())
    }

    /// Adds a tag to the cache
    ///
    /// Adding a tag to the cache will also write the tag to disk
    pub fn add_tag(&mut self, tag: Tag) -> Result<()> {
        if self.has_tag(&tag.name) {
            return Err(anyhow!("Tag: {} already exist in repo", tag.name));
        }

        self.write_tag(&tag)?;
        self.cache.add_tag(tag);

        Ok(())
    }

    pub fn get_repository(&self, name: &str) -> Option<&Repository> {
        self.cache.get_repository(&name)
    }

    pub fn take_repository(&mut self, name: &str) -> Option<Repository> {
        self.cache.take_repository(&name)
    }

    pub fn get_tag(&mut self, name: &str) -> Option<&Tag> {
        self.cache.get_tag(&name)
    }

    pub fn take_tag(&mut self, name: &str) -> Option<Tag> {
        self.cache.take_tag(&name)
    }

    pub fn has_repository(&self, name: &str) -> bool {
        self.cache.has_repository(&name)
    }

    pub fn has_tag(&self, name: &str) -> bool {
        self.cache.has_tag(&name)
    }

    pub fn remove_repository(&mut self, name: &str) -> Result<()> {
        debug!("Removing repository: '{}' from cache", name);
        self.cache.remove_repository(&name)
    }

    pub fn remove_tag(&mut self, name: &str) -> Result<()> {
        debug!("Removing tag: '{}' from cache", name);
        self.cache.remove_tag(&name)
    }

    pub fn update_remotes(&self, repository: &Repository) -> Result<()> {
        let workspace_path = self
            .config
            .root(None)
            .join(repository.resolve_workspace_path());

        let use_cli = self.config.cli(None);

        if workspace_path.is_dir() {
            git::merge(&workspace_path, use_cli)?;
        } else {
            let remote_name =
                repository.remotes.get(0).map(|r| &r.name).ok_or_else(|| {
                    anyhow!("Repository: {} does not have a remote", repository.name)
                })?;
            let branch = format!("{}/master", remote_name);
            git::clone(
                &workspace_path,
                &branch,
                repository.remotes.as_slice(),
                use_cli,
            )?;
        }

        Ok(())
    }

    pub fn write_repository(&self, repository: &Repository) -> Result<()> {
        let file = &repository.config;
        debug!("Writing repository to: {:#?}", file);

        let ser = toml::to_string_pretty(&repository).context(format!(
            "failed to serialize repository to file\n\n{:#?}",
            repository
        ))?;

        util::write_content(file, |f| {
            f.write_fmt(format_args!("{}", ser))
                .context(format!("failed to write file: {:#?}", file))
                .map_err(Into::into)
        })
    }

    pub fn write_tag(&self, tag: &Tag) -> Result<()> {
        let file = &tag.config;
        debug!("Writing tag to: {:#?}", file);

        let ser = toml::to_string_pretty(&tag)
            .context(format!("failed to serialize tag to file\n\n{:#?}", tag))?;

        util::write_content(&file, |f| {
            f.write_fmt(format_args!("{}", ser))
                .context(format!("failed to write file: {:#?}", file))
                .map_err(Into::into)
        })
    }
}
