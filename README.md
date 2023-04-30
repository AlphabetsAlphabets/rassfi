# rassfi
A password manager written in Rust using Rofi. Which is basically just passmenu-rs.

# Prerequisites
1. rofi

# Usage
There are two ways to use rassfi
1. To create a new password for a new account

To do this run `rassfi encrypt`. A popup will appear and there enter the name of the new account. The password is randomly generated using [OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html#) which implements the [CryptoRng](https://docs.rs/rand/latest/rand/trait.CryptoRng.html) trait.

The password is automatically encrypted using [srsa](https://docs.rs/srsa/0.1.6/srsa/) which uses the already well established [rsa](https://docs.rs/rsa/0.9.0/rsa/) crate.

2. To decrypt a password for an existing accuont with `rassfi login`.

A pop up will appear and you'll choose which account to retrieve the password for. Then you select the key that is needed to decrypt the file. 

# Configuration
- `RASSFI_KEYSTORE`. This is where rassfi will look for when retrieving the keys you have on your system. Public and private.
- `RASSFI_VAULT`. This is the location where rassfi will write passwords to during `rassfi encrypt`.
