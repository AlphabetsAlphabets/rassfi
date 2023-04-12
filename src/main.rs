use std::io;

use rassfi::Rassfi;

mod rassfi;

fn main() -> io::Result<()> {
    let rassfi = Rassfi::new("sec", "pub", "123")?;
    rassfi.show_services();

    Ok(())
}
