use sha2::{Digest, Sha256};
use std::env;
use std::fs::File;
use std::io::{self, IsTerminal, Read};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_TIME: &str = env!("BUILD_TIME");
const GIT_COMMIT: &str = env!("GIT_COMMIT");
const TARGET: &str = env!("TARGET_TRIPLE");

fn stdin_has_data() -> bool {
    // true se NÃO for terminal → tipicamente pipe/redirect
    !io::stdin().is_terminal()
}

fn hash_reader<R: Read>(mut r: R) -> io::Result<String> {
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = r.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn print_version() {
    // linha principal: tipo `sha256sum-rs v0.1.0 (x86_64-pc-windows-gnu)`
    println!(
        "{name} v{ver} ({target})",
        name = PKG_NAME,
        ver = PKG_VERSION,
        target = TARGET
    );

    // só imprime os campos extra se não estiverem vazios
    if !BUILD_TIME.is_empty() {
        println!("build time: {time}", time = BUILD_TIME);
    }
    if !GIT_COMMIT.is_empty() {
        println!("commit: {commit}", commit = GIT_COMMIT);
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.contains(&"--version".to_string()) {
        print_version();
        return Ok(());
    }

    // Sem argumentos: lê de stdin, como o sha256sum.
    if args.is_empty() {
        if !stdin_has_data() {
            eprintln!("sem dados em stdin; terminar.");
            std::process::exit(1);
        }

        let hash = hash_reader(io::stdin())?;
        // sha256sum não mostra nome quando é stdin.
        println!("{}  -", hash);
        return Ok(());
    }

    // Com argumentos: trata cada ficheiro
    let mut exit_code = 0;

    for filename in args {
        match File::open(&filename) {
            Ok(file) => match hash_reader(file) {
                Ok(hash) => {
                    println!("{}  {}", hash, filename);
                }
                Err(e) => {
                    eprintln!("sha256sum-rs: {}: erro a ler: {}", filename, e);
                    exit_code = 1;
                }
            },
            Err(e) => {
                eprintln!("sha256sum-rs: {}: {}", filename, e);
                exit_code = 1;
            }
        }
    }

    // Em Rust puro não dá para mudar o exit code depois de imprimir,
    // mas podemos simular com std::process::exit:
    if exit_code != 0 {
        std::process::exit(exit_code);
    }

    Ok(())
}
