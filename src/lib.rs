use std::collections::HashMap;
use std::env;

/// Represents a single configuration item, which may belong to a section.
#[derive(Debug, Clone)]
struct ConfigItem {
    section: Option<String>,
    key: String,
    value: String,
}

/// Organizes configuration items into sections for TOML format output.
#[derive(Debug, Default)]
struct Config {
    global: Vec<ConfigItem>,
    sections: HashMap<String, Vec<ConfigItem>>,
}

impl Config {
    /// Parses environment variables with a given prefix into a structured `Config`.
    fn from_env(prefix: &str) -> Self {
        let mut config = Self::default();
        for (key, value) in env::vars() {
            if let Some(stripped_key) = key.strip_prefix(prefix) {
                let normalized_key = stripped_key.to_lowercase();
                let parts: Vec<&str> = normalized_key.split("__").collect();
                let (section_parts, key) = parts.split_at(parts.len().saturating_sub(1));
                let section = section_parts.join(".");

                let config_item = ConfigItem {
                    section: if section.is_empty() {
                        None
                    } else {
                        Some(section)
                    },
                    key: key.join(""),
                    value,
                };

                if config_item.section.is_some() {
                    config
                        .sections
                        .entry(config_item.section.clone().unwrap())
                        .or_default()
                        .push(config_item);
                } else {
                    config.global.push(config_item);
                }
            }
        }
        config
    }

    /// Converts the structured `Config` into a TOML-formatted string.
    fn to_toml(&self) -> String {
        let mut result = String::new();

        // Add global configuration items.
        for item in &self.global {
            result.push_str(&format!("{} = \"{}\"\n", item.key, item.value));
        }

        // Add sectioned configuration items.
        for (section, items) in &self.sections {
            result.push_str(&format!("\n[{}]\n", section));
            for item in items {
                result.push_str(&format!("{} = \"{}\"\n", item.key, item.value));
            }
        }

        result
    }
}

/// Converts environment variables with a specified prefix into a TOML string.
///
/// # Arguments
///
/// * `prefix` - A string slice that holds the prefix for filtering environment variables.
///
/// # Returns
///
/// A `Result` which is either a `String` containing the TOML representation or an error message.
pub fn env_to_toml(prefix: &str) -> Result<String, String> {
    let config = Config::from_env(prefix);
    Ok(config.to_toml())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use super::*;
    #[test]
    fn test_env_to_toml() {
        dotenvy::dotenv().ok();
        let result = env_to_toml("APP_").unwrap();
        println!("{}\n", result);
    }

    #[test]
    fn test_env_to_toml_and_write_file() {
        dotenvy::dotenv().ok();
        let toml_content = env_to_toml("APP_").unwrap();
        write_to_file("config.toml", &toml_content).expect("Failed to write TOML to file");
        println!("TOML content written to config.toml:\n{}", toml_content);
    }
    fn write_to_file(filename: &str, content: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
