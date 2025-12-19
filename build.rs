use std::process::Command;

fn main() {
    // Data/hora de build em UTC (portável: usa o próprio `date` em sistemas tipo Unix
    // e fallback para vazio se falhar, para compilar também no Windows sem `date`).
    let build_time = if cfg!(windows) {
        // no Windows, deixa vazio por omissão; podes preencher com outra ferramenta via CI
        String::new()
    } else {
        let out = Command::new("date")
            .arg("-u")
            .arg("+%Y-%m-%dT%H:%M:%SZ")
            .output();
        match out {
            Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
            _ => String::new(),
        }
    };

    // Commit git, se o repositório estiver disponível
    let git_commit = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    // Target triple fornecido pelo Cargo (funciona em qualquer SO/plataforma)
    let target = std::env::var("TARGET").unwrap_or_default(); // ex: x86_64-unknown-linux-gnu [web:71][web:74]

    let build_tag = std::env::var("BUILD_TAG").unwrap_or_default();

    // Exportar para o binário como variáveis de ambiente em tempo de compilação
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rustc-env=GIT_COMMIT={}", git_commit);
    println!("cargo:rustc-env=TARGET_TRIPLE={}", target);
    println!("cargo:rustc-env=BUILD_TAG={}", build_tag);
}
