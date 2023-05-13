use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
};

use anyhow::{Context, Result as AnyResult};
use rand::{rngs::OsRng, RngCore};
use rsa::pkcs8::der::zeroize::Zeroizing;
use srsa::Keys;

/// # Parameters
/// - `keystore`: The path to where the private and public keys are stored.
/// - `vault`: The place where all your encrypted files are stored.
/// - `accounts`: The path of all encrypted files.
pub struct Rassfi<'key> {
    keystore: PathBuf,
    accounts: Vec<PathBuf>,
    vault: PathBuf,
    key: Option<Keys<'key>>,
}

impl Rassfi<'_> {
    /// Creates an instance of `Rassfi`. Will faill if
    /// `RASSFI_VAULT` or `RASSFI_KEYSTORE` is not set.
    pub fn new() -> AnyResult<Self> {
        // A new variable for specifying the location of public and private keys.
        let keystore = Self::load_keystore()?;
        let accounts = Self::load_accounts()?;

        let vault = env::var("RASSFI_VAULT")?.into();

        Ok(Self {
            keystore,
            accounts,
            vault,
            key: None,
        })
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
impl Rassfi<'_> {
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
impl<'key> Rassfi<'key> {
    /// Display all the encrypted files specified in `RASSFI_VAULT`.
    pub fn display_accounts(&mut self) {
        let mut options = vec![];
        for account in &self.accounts {
            let account = account.file_name().unwrap();
            options.push(account.to_str().unwrap());
        }

        let input = self.form_builder(&options);
        println!("Input: {}", input);
    }

    /// Display all the keys in `RASSFI_KEYSTORE`.
    // TODO: This should return a key.
    pub fn key_selection(&mut self) -> AnyResult<String> {
        let path = fs::read_dir(&self.keystore).with_context(|| {
            let path = &self.keystore.to_str().unwrap();
            format!("Directory '{}' not found at {}:{}", path, file!(), line!())
        })?;

        let mut keys = vec![];
        // TODO: FIX: Will need to look into this.
        for item in path {
            let name = item?.file_name();
            let string = match name.into_string() {
                Ok(string) => string,
                Err(osstring) => format!("[INVALID] original value: {:?}", osstring),
            };

            keys.push(string);
        }

        let all_keys: Vec<&str> = keys.iter().map(AsRef::as_ref).collect();
        let chosen_key = self.form_builder(all_keys.as_slice());
        let vault = &self
            .keystore
            .to_str()
            .context("Unable to convert RASSFI_KEYSTORE to &str.")?;

        let key_path = format!("{}/{}", vault, chosen_key);

        Ok(key_path)
    }

    /// Will decrypt the encrypted private key with the provided password.
    /// Produces an error on failure.
    pub fn authenticate(&mut self, private_key: &'key str) -> AnyResult<()> {
        let password = Zeroizing::new(self.form_builder(&vec!["Enter your password."]));
        let key = Keys::retrieve_private_key(private_key, &password)?;
        self.key = Some(key);

        Ok(())
    }
}

// Functions related to creating a new account
impl Rassfi<'_> {
    pub fn prompt_service_name(&mut self) -> AnyResult<()> {
        // TODO: Find option to change the name of the placeholder text.
        let rofi = Command::new("rofi").arg("-dmenu").output().unwrap();
        let input = String::from_utf8(rofi.stdout).unwrap();

        if input.is_empty() {
            println!("Empty account name.");
            exit(1);
        }

        let path = &self
            .vault
            .to_str()
            .context("Unable to convert RASSFI_VAULT to &str.")?;

        let path = format!("{}{}", path, input);
        let pass = self.generate_password();

        self.key_selection()?;

        // fs::write(path, pass)?;

        Ok(())
    }

    fn generate_password(&self) -> Zeroizing<String> {
        let mut key = [0u8; 16];
        OsRng.fill_bytes(&mut key);

        let mut password = String::new();
        while password.len() < 64 {
            let random = OsRng.next_u32();
            password = format!("{}{}", password, random);
        }

        Zeroizing::new(password)
    }
}
