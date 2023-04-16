mod rassfi;
use rassfi::Rassfi;
use srsa::errors::KeyError;

fn main() -> Result<(), KeyError>  {
    let rassfi = Rassfi::new()?;
    rassfi.display_accounts();
    Ok(())
}
