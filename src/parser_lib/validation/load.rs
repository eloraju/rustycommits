use std::{
    env,
    fs::{metadata, read_to_string, File},
    io::{self, Result as ioResult, Write},
    path::PathBuf,
};

use super::rules::ValidationRules;

const CONFIG_FILENAME: &str = "rclint.toml";

fn try_parse_config(content: String) -> Result<ValidationRules, String> {
    match toml::from_str(&content) {
        Ok(rules) => Ok(rules),
        Err(err) => Err(err.to_string()),
    }
}

pub fn load_rules() -> ValidationRules {
    let path = look_for_conf_in(None);
    match path {
        Ok(Some(conf)) => {
            let content = read_to_string(conf).unwrap();
            match try_parse_config(content) {
                Ok(rules) => rules,
                Err(err) => {
                    eprintln!("Error while parsing config file: {}", err);
                    println!("Using default rules.");
                    ValidationRules::default()
                }
            }
        }
        Ok(None) => ValidationRules::default(),
        Err(_) => {
            eprintln!("Error while looking for config file. Using default rules.");
            ValidationRules::default()
        }
    }
}

// Start looking for the config file in current directory, then in parent directories
fn look_for_conf_in(path: Option<PathBuf>) -> ioResult<Option<PathBuf>> {
    let cur_dir = path.unwrap_or_else(|| env::current_dir().unwrap());
    let conf = cur_dir.join(CONFIG_FILENAME);

    if let Ok(meta) = metadata(&conf) {
        if meta.is_file() {
            return Ok(Some(conf));
        }
    }

    if let Ok(meta) = metadata(&cur_dir.join(".git")) {
        if meta.is_dir() {
            // We are in a git project root, good enough
            return create_default_config(&cur_dir);
        }
    }

    match cur_dir.parent() {
        Some(parent) => look_for_conf_in(Some(parent.to_path_buf())),
        // This should only happen when rcommitlint is called outside a cargo project
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No config file found",
        )),
    }
}

fn create_default_config(path: &PathBuf) -> ioResult<Option<PathBuf>> {
    let conf = path.join(CONFIG_FILENAME);
    println!(
        "No config file found. Creating default config into {:?}",
        conf
    );

    let default_rules = ValidationRules::default();
    let toml = toml::to_string(&default_rules).unwrap();
    let mut file = File::create(&conf)?;
    file.write_all(toml.as_bytes())?;
    println!("{}", toml);
    return Ok(Some(conf));
}

#[cfg(test)]
mod tests {
    use std::process::{Command, Output};

    use super::*;
    use tempfile::{tempdir, TempDir};

    fn create_test_dir(path: &str, cwd: &TempDir) -> Output {
        Command::new("mkdir")
            .args(["-p", path])
            .current_dir(cwd.path())
            .output()
            .unwrap()
    }

    fn init_mock_dirs() -> ioResult<PathBuf> {
        let dir = tempdir()?;
        create_test_dir("src/lib/logic", &dir);
        create_test_dir("src/bin/cli", &dir);
        create_test_dir("tests", &dir);
        create_test_dir(".git", &dir);

        let ls_res = Command::new("ls")
            .current_dir(&dir.path().join("src"))
            .output()?;

        let lsa_res = Command::new("ls")
            .arg("-a")
            .current_dir(&dir.path())
            .output()?;

        assert_eq!(String::from_utf8(ls_res.stdout).unwrap(), "bin\nlib\n");
        assert_eq!(
            String::from_utf8(lsa_res.stdout).unwrap(),
            ".\n..\n.git\nsrc\ntests\n"
        );
        Ok(dir.into_path())
    }

    #[test]
    fn should_generate_default_config_when_none_found() {
        let mock_dir = init_mock_dirs().unwrap();
        let no_conf = look_for_conf_in(Some(mock_dir.to_path_buf().join("src/lib/logic")));
        assert_eq!(
            no_conf.unwrap().unwrap().to_str().unwrap(),
            mock_dir.join(CONFIG_FILENAME).to_str().unwrap()
        );
        let default_conf = look_for_conf_in(Some(mock_dir.to_path_buf().join("src/bin/cli")));
        assert!(&default_conf.is_ok());
        assert_eq!(
            &default_conf.unwrap().unwrap().to_str().unwrap(),
            &mock_dir.join(CONFIG_FILENAME).to_str().unwrap()
        );
    }
}
