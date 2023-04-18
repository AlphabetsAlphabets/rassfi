use std::{
    env,
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
};

/// # Parameters
/// - `keystore`: The path to where the private and public keys are stored.
/// - `vault`: The directory where the encrypte files are stored.
pub struct Rassfi {
    keystore: PathBuf,
    accounts: Vec<PathBuf>,
}

/// Returns takes a `Result` of any type then returns the `ok`.
fn handle_error<T, U: Display>(result: Result<T, U>) -> T {
    match result {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    }
}

impl Rassfi {
    /// Creates an instance of `Rassfi`.
    pub fn new() -> io::Result<Self> {
        // A new variable for specifying the location of public and private keys.
        let keystore = Self::load_keystore();
        let accounts = Self::load_accounts()?;

        Ok(Self { keystore, accounts })
    }

    /// Loads in all encrypted files from the location specified in `RASSFI_VAULT`.
    fn load_accounts() -> io::Result<Vec<PathBuf>> {
        let vault = handle_error(env::var("RASSFI_VAULT"));

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
    fn load_keystore() -> PathBuf {
        let keystore = handle_error(env::var("RASSFI_KEYSTORE"));
        Path::new(&keystore).to_owned()
    }
}

/// Utility functions for rofi
impl Rassfi {
    /// Populates the rofi dropdown with values in `options`.
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
    fn key_selection(&self) -> io::Result<()> {
        // Returns Result<T>
        let entries = fs::read_dir(&self.keystore)?
            // `res` is a Result<DirEntry, Error>
            .map(|res| res.map(|e| e.path()))
            // res.map() will return a Vec<PathBuf>.
            // Which is why Vec<_> is in Result<U, V>
            // The ? will panic if there is any Error.
            .collect::<Result<Vec<_>, io::Error>>()?;

        let entires: Vec<&str> = entries
            .iter()
            .map(|entry| {
                let name = entry.file_name().unwrap().to_str().unwrap();
                name
            })
            .collect();

        let input = self.form_builder(&entires);
        println!("Key: {}", input);

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
