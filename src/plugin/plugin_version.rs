/*use anyhow::{anyhow, Error};

pub struct PluginVersion(u8, u8, u8);

impl TryFrom<String> for PluginVersion {
    type Error = Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let parts = string.split('.').collect::<Vec<&str>>();
        let major: u8 = parts.get(0).ok_or(anyhow!("Missing major, minor, and patch version numbers."))?.parse()?;
        let minor: u8 = parts.get(1).ok_or(anyhow!("Missing minor and patch version numbers."))?.parse()?;
        let patch: u8 = parts.get(2).ok_or(anyhow!("Missing patch version number."))?.parse()?;
        Ok(PluginVersion(major, minor, patch))
    }
}

impl From<(u8, u8, u8)> for PluginVersion {
    fn from((major, minor, patch): (u8, u8, u8)) -> Self {
        PluginVersion(major, minor, patch)
    }
}*/