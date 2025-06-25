use core::error::Error;
use std::{env, fs::File, io::Write, path::Path, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=../../../Cargo.lock");

    if Command::new("cargo")
        .args(["about", "--version"])
        .status()
        .is_ok_and(|status| status.success())
    {
        cargo_about_installed()?;
    } else {
        cargo_about_not_installed()?;
    }

    Ok(())
}

fn write_file<E: Error + 'static>(
    output: impl TryInto<String, Error = E>,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = output.try_into()?;
    let output = output.replace("&quot;", "\"");
    let out_dir = env::current_dir().unwrap();
    let dest_path = Path::new(&out_dir)
        .join("src")
        .join("third_party_licences_generated.md");
    let mut file = File::create(dest_path)?;
    writeln!(&mut file, "{output}")?;
    Ok(())
}

fn cargo_about_installed() -> Result<(), Box<dyn std::error::Error>> {
    let target = std::env::var("TARGET").unwrap();
    let features = std::env::var("CARGO_CFG_FEATURE").unwrap();
    let res = Command::new("cargo-about")
        .args([
            "-L",
            "warn",
            "generate",
            "-m",
            "../../plane-sweep/Cargo.toml",
            "--target",
            &target,
            "--features",
            &features,
            "about.hbs",
        ])
        .output();

    if let Ok(output) = res {
        let err = String::from_utf8(output.stderr)?;
        if output.status.success() {
            write_file(String::from_utf8(output.stdout)?)?;
        } else {
            println!("cargo:warning=Error while executing cargo-about");
            write_file(FALLBACK)?;
        }
        if !err.is_empty() && features.contains("warnings") {
            for line in err.lines() {
                println!("cargo:warning={line}");
            }
        }
    } else {
        write_file(FALLBACK)?;
    }

    Ok(())
}

fn cargo_about_not_installed() -> Result<(), Box<dyn std::error::Error>> {
    if std::fs::exists("src/third_party_licences.rs").is_ok_and(|v| v) {
        println!("cargo:warning=cargo-about is not installed, using fallback implementation.");
    } else {
        println!(
            "cargo:warning=cargo-about is not installed, but src/third_party_licences.md does not exist. Please install cargo-about or generate the file"
        );
        write_file(FALLBACK)?;
    }
    Ok(())
}

pub const FALLBACK: &str = "# Did not found cargo-about or could not generate third party licences";
