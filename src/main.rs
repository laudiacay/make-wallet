use clap;
use ethers_signers::{MnemonicBuilder, coins_bip39::English};

fn main() {
    // parse command line arguments to get a value for `dir`
    let matches = clap::App::new("make-wallet")
        .arg(clap::Arg::with_name("wallet-dir")
             .help("the directory to stick the wallet in")
             .required(true))
        .get_matches();

    let dir = matches.value_of("wallet-dir").unwrap();

    // make sure the directory exists
    match std::fs::create_dir_all(dir) {
        Ok(_) => (),
        Err(e) => {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                panic!("failed to create directory: {}", e);
            }
        }
    };

    // get a pathbuf for the wallet directolry
    let path = std::path::PathBuf::from(dir);

    let mut rng = rand::thread_rng();
    // touch the file to create it
    MnemonicBuilder::<English>::default()
        .word_count(24)
        .write_to(path)
        .build_random(&mut rng).unwrap();

}
