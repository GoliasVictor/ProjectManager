//! Providers of information about environments and their metadata
use crate::prelude::*;
mod dotmeta_provider;
mod git_provider;
mod vscode_provider;
///return the name if `opname` is not None and is not whitespace, otherwise return the file name  
fn name_else_filename(opname: Option<String>, path: &Path) -> Option<String> {
    if let Some(name) = opname {
        if !name.trim().is_empty() {
            return Some(name);
        }
    }
    return path
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string());
}
/// if the path is a valid environment, extract metadata of an environment from the environment
/// 
/// Support extracting information from a single source, and the sources are as follows (in order of attempted extraction): 
///  - `.meta` files,  
///  - `<name>.code-workspace` files
///  - `git` 
pub fn get_meta(path: &Path) -> Result<Metadata> {
    let mut metadata = dotmeta_provider::get_meta(path)
        .or_else(|_| vscode_provider::get_meta(path))
        .or_else(|_| git_provider::get_meta(path))?;
    metadata.source = path.to_path_buf();
    metadata.name = name_else_filename(metadata.name.clone(), path);
    if let Some(EnvironmentType::Project) = metadata.environment_type {
        if let Some(children) = &mut metadata.children {
            for child in children.iter_mut() {
                if let Some(child_path) = child
                    .path
                    .as_ref()
                    .and_then(|path| metadata.source.join(path).canonicalize().ok())
                {
                    child.name = name_else_filename(child.name.clone(), &child_path);
                }
                child.environment_type = Some(EnvironmentType::SubProject);
            }
        }
    }
    Ok(metadata)
}
/// returns data from an environment if it is a valid environment
/// Support extracting information from a single source, and the sources are as follows (in order of attempted extraction): 
///  - `.meta` files,  
///  - `<name>.code-workspace` files
///  - `git` 
pub fn get_environment(path: &Path) -> Result<Environment> {
    let meta = get_meta(path)?;
    Environment::from_metadata(meta)
}
