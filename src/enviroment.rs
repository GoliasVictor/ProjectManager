use std::{collections::HashMap, path::PathBuf};

use crate::{metadata::Metadata, EnvironmentType};

#[derive(Debug)]
pub enum EnvironmentDetails {
    Folder,
    Project {
        languages: Vec<String>,
        open_command: Option<String>,
        init_command: Option<String>,
        scripts: HashMap<String, String>,
    },
    SubProject {
        path: PathBuf,
    },
}

#[derive(Debug)]
pub struct Environment {
    pub name: String,
    pub description: Option<String>,
    pub source: PathBuf,
    pub children: Vec<Environment>,
    pub details: EnvironmentDetails,
}

impl Environment {
    pub fn from_metadata(meta: Metadata) -> Result<Self, i8> {
        return Ok(Self {
            name: meta.name.ok_or(0)?,
            description: meta.description,
            source: meta.source,
            details: if let Some(environment_type) = meta.environment_type {
                match environment_type {
                    EnvironmentType::Folder => EnvironmentDetails::Folder,
                    EnvironmentType::Project => EnvironmentDetails::Project {
                        languages: meta.languages.unwrap_or(vec![]),
                        open_command: meta.open_command,
                        init_command: meta.init_command,
                        scripts: meta.scripts.unwrap_or(HashMap::new()),
                    },
                    EnvironmentType::SubProject => EnvironmentDetails::SubProject {
                        path: meta.path.ok_or(0)?,
                    },
                }
            } else {
                EnvironmentDetails::Project {
                    languages: vec![],
                    open_command: None,
                    init_command: None,
                    scripts: HashMap::new(),
                }
            },
            children: meta.children.map_or(vec![], |sps| {
                sps.into_iter()
                    .filter_map(|m| Environment::from_metadata(m).ok())
                    .collect()
            }),
        });
    }
}