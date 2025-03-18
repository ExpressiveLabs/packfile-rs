use std::path::{Path, PathBuf};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ContentBundle {
    pub target: InstallationPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<InstallationPath>
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Dependency {
    pub name: String,
    pub version: String
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct InstallationPath {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universal: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macos: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux: Option<PathBuf>
}

impl InstallationPath {
    pub fn get(&self) -> PathBuf {
        if let Some(path) = &self.universal {
            return path.clone();
        }
        
        #[cfg(target_os = "windows")]
        {
            self.windows.clone().unwrap()
        }
        #[cfg(target_os = "linux")]
        {
            self.linux.clone().unwrap()
        }
        #[cfg(target_os = "macos")]
        {
            self.macos.clone().unwrap()
        }
    }
    
    pub fn canonicalize(&self) -> PathBuf {
        let path = self.get().canonicalize().unwrap();

        #[cfg(target_os = "windows")]
            let mut application_dir = PathBuf::from(env!("PROGRAMFILES"));
        #[cfg(target_os = "macos")]
            let mut application_dir = dirs::home_dir().unwrap().join("Applications");
        #[cfg(target_os = "linux")]
            let mut application_dir = dirs::executable_dir().unwrap();

        let home_dir = dirs::home_dir().unwrap();

        fn parse<P: AsRef<Path>>(path: P, app_dir: P, home_dir: P) -> PathBuf {
            PathBuf::from(path.as_ref().to_str().unwrap()
                .replace("$APPLICATIONS", app_dir.as_ref().to_str().unwrap())
                .replace("$HOME", home_dir.as_ref().to_str().unwrap()))
        }

        #[cfg(debug_assertions)] {
            application_dir = dirs::home_dir().unwrap().join("ExpressiveLabs/ELSC/testing");
        }
        
        parse(path, application_dir, home_dir)
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub url: String
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FiletypeAssociation {
    pub extension: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PackageFile {
    pub name: String,
    pub identifier: String,
    pub version: String,
    pub icon: Option<String>,
    
    pub filetypes: Vec<FiletypeAssociation>,
    
    pub developer: Author,
    pub publisher: Author,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_hub_version: Option<String>,
    
    pub entrypoint: InstallationPath,
    pub contents: Vec<ContentBundle>
}

impl PackageFile {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let package: PackageFile = serde_json::from_str(&file)?;
        
        Ok(package)
    }
    
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = serde_json::to_string_pretty(&self)?;
        std::fs::write(path, file)?;
        
        Ok(())
    }
}