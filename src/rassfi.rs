use std::{
    env,
    fs::{self},
    io,
    path::PathBuf,
    process::{exit, Child, Command, Stdio},
};

use srsa::Keys;

/// - `Keys`: Enable usage of RSA key pairs through this struct.
/// - `accounts`: The path to each encrypted file.
/// - `vault`: The directory where the encrypte files are stored.
pub struct Rassfi<'names> {
    key: Keys<'names>,
    accounts: Vec<PathBuf>,
    vault: PathBuf,
}

impl<'names> Rassfi<'names> {
    /// Creates an instance of `Rassfi`.
    pub fn new(priv_key: &'names str, pub_key: &'names str, password: &str) -> io::Result<Self> {
        let key = Keys::retreive_keys(&priv_key, &password, &pub_key);

        let vault = match env::var("RASSFI_VAULT") {
            Ok(vault) => vault,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        };

        let directory_contents = fs::read_dir(&vault)?;
        // entry.ok() will return None if entry is Err. filter_map will 
        // only save values of Some(_) and ignore Err. So only valid DirEntry is saved.
        let accounts: Vec<PathBuf> = directory_contents
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();

        let vault = PathBuf::from(vault);

        Ok(Self {
            key,
            accounts,
            vault,
        })
    }
}

// Public methods
impl Rassfi<'_> {
    /// Builds the rofi menu
    pub fn show_services(&self) -> String {
        let options = self.feed_options();

        let input = Command::new("rofi")
            .arg("-dmenu")
            .stdin(Stdio::from(options.stdout.unwrap()))
            .output()
            .unwrap();

        // Can get output like this
        String::from_utf8(input.stdout).unwrap()
    }

}

// Private methods
impl Rassfi<'_> {
    /// Returns a spawned `Child`.
    fn feed_options(&self) -> Child {
        let mut options = String::new();
        for account in &self.accounts {
            // This will unwrap if file has no name. Will need to disallow empty file names.
            let entry = account.file_stem().unwrap();
            // If the above passes, this will too. Can remain unwrap.
            let entry = entry.to_str().unwrap();
            options.push_str(format!("{}\n", entry).as_str());
        }

        Command::new("echo")
            .args(["-e", &options])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
    }
}
