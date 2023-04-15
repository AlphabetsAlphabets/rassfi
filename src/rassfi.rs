use std::{
    env,
    fs,
    io,
    path::{PathBuf, Path},
};

/// # Parameters
/// - `keystore`: The path to where the private and public keys are stored.
/// - `vault`: The directory where the encrypte files are stored.
pub struct Rassfi {
    keystore: PathBuf,
    accounts: Vec<PathBuf>,
}

impl Rassfi {
    /// Creates an instance of `Rassfi`.
    pub fn new() -> io::Result<Self> {
        // A new variable for specifying the location of public and private keys.
        let keystore = Self::load_keystore();
        let accounts = Self::load_accounts()?;

        Ok(Self {
            keystore,
            accounts,
        })
    }

    /// Loads in all encrypted files from the location specified in `RASSFI_VAULT`.
    fn load_accounts() -> io::Result<Vec<PathBuf>> {
        let vault = match env::var("RASSFI_VAULT") {
            Ok(vault) => vault,
            Err(e) => panic!("{}", e),
        };

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
        let key_store = match env::var("RASSFI_KEYSTORE") {
            Ok(keys) => keys,
            Err(e) => panic!("{}", e),
        };

        Path::new(&key_store).to_owned()
    }
}
