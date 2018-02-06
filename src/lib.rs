#[macro_use]
extern crate quick_error;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

#[cfg(not(target_os = "windows"))]
fn get_config_path() -> Option<PathBuf> {
    let home = match env::home_dir() {
        Some(path) => path,
        None => return None,
    };
    let path = Path::new(&home).join(".dropbox/info.json");
    if path.is_file() {
        Some(path)
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
fn get_config_path() -> Option<PathBuf> {
    const CFG_PATH_SUFFIX: &'static str = "Dropbox/info.json";
    let mut appdata = String::new();
    let mut localappdata = String::new();
    for (key, value) in env::vars() {
        if key == "APPDATA" {
            appdata = value;
        } else if key == "LOCALAPPDATA" {
            localappdata = value;
        }
        if appdata != "" && localappdata != "" {
            break;
        }
    }

    let roamingpath = Path::new(&appdata).join(CFG_PATH_SUFFIX);
    let localpath = Path::new(&localappdata).join(CFG_PATH_SUFFIX);
    if roamingpath.is_file() {
        Some(roamingpath)
    } else if localpath.is_file() {
        Some(localpath)
    } else {
        None
    }
}

#[derive(Debug, Deserialize)]
struct Account {
    path: String,
    host: i64,
    is_team: bool,
    subscription_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    personal: Option<Account>,
    business: Option<Account>,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        NotConfiguredError {
            description("Dropbox not configured")
        }
        CantReadConfigError{
            from(std::io::Error)
            description("can't read configuration")
        }
        InvalidConfigError{
            from(serde_json::Error)
            description("configuration invalid")
        }
        AccountNotConfiguredError {
            description("account type not configured")
        }
    }
}

/// Reads info from Dropbox configuration files.
pub fn read_info() -> Result<Info, Error> {
    let cfg_path = match get_config_path() {
        Some(path) => path,
        None => return Err(Error::NotConfiguredError),
    };
    let file = File::open(cfg_path)?;
    let info: Info = serde_json::from_reader(file).unwrap();
    Ok(info)
}

fn get_dir(account_type: &str) -> Result<String, Error> {
    let info = read_info()?;
    let data = match account_type {
        "personal" => info.personal,
        "business" => info.business,
        _ => unreachable!(),
    };
    if let Some(account) = data {
        Ok(account.path.clone())
    } else {
        Err(Error::AccountNotConfiguredError)
    }
}

/// Gets the personal directory path.
pub fn personal_dir() -> Result<String, Error> {
    get_dir("personal")
}

/// Gets the business directory path.
pub fn business_dir() -> Result<String, Error> {
    get_dir("business")
}

pub struct SmartPath {
    local_root: PathBuf,
    target: PathBuf,
}

impl SmartPath {
    fn new<S>(root: S, path: S) -> SmartPath
    where
        S: Into<String>,
    {
        let raw_target = PathBuf::from(path.into());
        let target: PathBuf = if raw_target.starts_with("/") {
            raw_target.strip_prefix("/").unwrap().into()
        } else {
            raw_target.clone()
        };
        SmartPath {
            local_root: PathBuf::from(root.into()),
            target: target,
        }
    }

    /// Creates a new `SmartPath`, the `path` parameter represent a target inside the Dropbox
    /// directory (personal).
    pub fn new_personal<S>(path: S) -> Result<SmartPath, Error>
    where
        S: Into<String>,
    {
        let root = personal_dir()?;
        Ok(SmartPath::new(root, path.into()))
    }

    /// Creates a new `SmartPath`, the `path` parameter represent a target inside the Dropbox
    /// directory (business).
    pub fn new_business<S>(path: S) -> Result<SmartPath, Error>
    where
        S: Into<String>,
    {
        let root = business_dir()?;
        Ok(SmartPath::new(root, path.into()))
    }

    /// Gets the local absolute path of the target.
    pub fn local(&self) -> PathBuf {
        if self.target.components().count() > 0 {
            self.local_root.join(&self.target)
        } else {
            self.local_root.clone()
        }
    }

    /// Gets the remote path of the target.
    pub fn remote(&self) -> PathBuf {
        PathBuf::from("/").join(&self.target)
    }
}
