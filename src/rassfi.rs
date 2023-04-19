use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
};

use anyhow::{Context, Result as AnyResult};

/// # Parameters
/// - `keystore`: The path to where the private and public keys are stored.
/// - `vault`: The directory where the encrypte files are stored.
pub struct Rassfi {
    keystore: PathBuf,
    accounts: Vec<PathBuf>,
}

impl Rassfi {
    /// Creates an instance of `Rassfi`.
    pub fn new() -> AnyResult<Self> {
        // A new variable for specifying the location of public and private keys.
        let keystore = Self::load_keystore()?;
        let accounts = Self::load_accounts()?;

        Ok(Self { keystore, accounts })
    }

    /// Loads in all encrypted files from the location specified in `RASSFI_VAULT`.
    fn load_accounts() -> AnyResult<Vec<PathBuf>> {
        let vault =
            env::var("RASSFI_VAULT").context("RASSFI_VAULT environment variable not set.")?;

        let directory_contents = fs::read_dir(&vault)?;
        // entry.ok() will return None if entry is Err. filter_map will
        // only save values of Some(_) and ignore Err. So only valid DirEntry is saved.
        let accounts: Vec<PathBuf> = directory_contents
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();

        Ok(accounts)
    }

    /// The location where all your keys are stored public and private specified in
    /// `RASSFI_KEYSTORE`.
    fn load_keystore() -> AnyResult<PathBuf> {
        let keystore =
            env::var("RASSFI_KEYSTORE").context("RASSFI_KEYSTORE environment variable not set.")?;

        Ok(Path::new(&keystore).to_owned())
    }
}

/// Utility functions for rofi
impl Rassfi {
    /// Populates the rofi dropdown with values in `options`.
    /// Then returns the user's selection as a `String`.
    fn form_builder(&self, options: &[&str]) -> String {
        let options = options.join("\n");
        let echo = Command::new("echo")
            .args(["-e", &options])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let rofi = Command::new("rofi")
            .arg("-dmenu")
            .stdin(Stdio::from(echo.stdout.unwrap()))
            .output();

        let rofi = match rofi {
            Ok(output) => output,
            Err(_) => {
                eprintln!("Rofi is not installed.");
                exit(1);
            }
        };

        let input = String::from_utf8(rofi.stdout).unwrap();
        input
    }
}

// Functions related to decrypting files.
impl Rassfi {
    /// Display all the encrypted files specified in `RASSFI_VAULT`.
    pub fn display_accounts(&self) {
        let mut options = vec![];
        for account in &self.accounts {
            let account = account.file_name().unwrap();
            options.push(account.to_str().unwrap());
        }

        let input = self.form_builder(&options);
        println!("Input: {}", input);
        self.key_selection().unwrap();
    }

    /// Display all the keys in `RASSFI_KEYSTORE`.
    fn key_selection(&self) -> AnyResult<()> {
        // Returns Result<T>
        let path = fs::read_dir(&self.keystore).with_context(|| {
            let path = &self.keystore.to_str().unwrap();
            format!("Directory '{}' not found.", path)
        })?;

        let mut entries = vec![];
        for item in path {
            let name = item?.file_name();
            let string = match name.into_string() {
                Ok(string) => string,
                Err(osstring) => format!("[INVALID] original value: {:?}", osstring),
            };

            entries.push(string);
        }

        let entries: Vec<&str> = entries.iter().map(AsRef::as_ref).collect();
        self.form_builder(entries.as_slice());

        Ok(())
    }
}

// Functions related to creating a new account
impl Rassfi {
    pub fn prompt_service_name(&self) {
        // TODO: Find option to change the name of the placeholder text.
        let rofi = Command::new("rofi").arg("-dmenu").output().unwrap();

        let input = String::from_utf8(rofi.stdout).unwrap();
        println!("New account name: {}", input);
    }
}
