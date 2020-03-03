mod cli;
mod config;

use std::{
    fs::{
        self,
        File,
    },
    io::{Write},
    env,
    process::{Stdio, Command},
};

use g_k_crates_io_client as crates_io;
use tempdir::TempDir;
use lazy_static::lazy_static;

use config::{Config, TraverseOrder};

static DEFAULT_REGISTRY: &str = "https://crates.io/";

lazy_static! {
    static ref STABLE_RUST_VERS: Vec<&'static str> = vec![
        "1.0.0",
        "1.1.0",
        "1.2.0",
        "1.3.0",
        "1.4.0",
        "1.5.0",
        "1.6.0",
        "1.7.0",
        "1.8.0",
        "1.9.0",
        "1.10.0",
        "1.11.0",
        "1.12.1",
        "1.13.0",
        "1.14.0",
        "1.15.1",
        "1.16.0",
        "1.17.0",
        "1.18.0",
        "1.19.0",
        "1.20.0",
        "1.21.0",
        "1.22.1",
        "1.23.0",
        "1.24.1",
        "1.25.0",
        "1.26.2",
        "1.27.2",
        "1.28.0",
        "1.29.2",
        "1.30.1",
        "1.31.1",
        "1.32.0",
        "1.33.0",
        "1.34.2",
        "1.35.0",
        "1.36.0",
        "1.37.0",
        "1.38.0",
        "1.39.0",
        "1.40.0",
        "1.41.1",
    ];
}

fn main() {
    let m = cli::build().get_matches();

    if let Some(sub_m) = m.subcommand_matches("msrv-table") {
        let cfg = Config::from(sub_m);
        let mut req = crates_io::Registry::new(DEFAULT_REGISTRY.to_string(), None);

        if let Ok(response) = req.get_crate_data(&*cfg.tgt_crate) {
            let tmp_dir = TempDir::new("msrv_table").unwrap();
            let file_path = tmp_dir.path().join(format!("{}_raw_resp", &*cfg.tgt_crate));
            let mut tmp_file = File::create(&file_path).unwrap();
            writeln!(tmp_file, "{}", &*response).unwrap();

            let tmp_file_path = file_path.to_str().unwrap();

            let jql = Command::new("jql")
                .arg("\"versions\"|\"num\"")
                .arg(tmp_file_path)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let grep = Command::new("grep")
                .arg("-oP")
                .arg("\"\\K[0-9]+\\.[0-9]+")
                .stdin(jql.stdout.unwrap())
                .stdout(Stdio::piped())
                .spawn().unwrap();

            let uniq = Command::new("uniq")
                .stdin(grep.stdout.unwrap())
                .output()
                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

            let s_vers = String::from_utf8_lossy(&uniq.stdout);
            let vers: Vec<_> = s_vers.lines().collect();

            traverse(&cfg, &vers);
        }
    }
}

fn traverse(cfg: &Config, crate_vers: &[&str]) {
    use TraverseOrder::*;

    let mut results: Vec<_> = vec![];

    let tmp_dir = TempDir::new("cargo_msrv_table").unwrap();

    match (cfg.crate_order, cfg.rust_order) {
        (Ascending, Ascending) => {
            for cv in crate_vers.iter().rev().filter(|v| !(cfg.skip_prereleases && v.starts_with('0'))) {
                setup_crate(&cfg.tgt_crate, &tmp_dir, cv);
                let mut min_rv = None;
                for rv in STABLE_RUST_VERS.iter() {
                    println!("-> Compiling {} v{} on Rust v{}", cfg.tgt_crate, cv, rv);
                    if try_compile(rv, cfg.timeout).is_ok() {
                        println!("  => Found MSRV v{}", rv);
                        min_rv = Some(rv);
                        break;
                    }
                }
                results.push((cv, min_rv));
            }
        },
        (Ascending, Descending) => {
            for cv in crate_vers.iter().rev().filter(|v| !(cfg.skip_prereleases && v.starts_with('0'))) {
                setup_crate(&cfg.tgt_crate, &tmp_dir, cv);
                let mut min_rv = None;
                let mut prev_rv = None;
                for rv in STABLE_RUST_VERS.iter().rev() {
                    println!("-> Compiling {} v{} on Rust v{}", cfg.tgt_crate, cv, rv);
                    if try_compile(rv, cfg.timeout).is_err() {
                        if let Some(prev) = prev_rv {
                            println!("  => Found MSRV v{}", prev);
                            min_rv = Some(rv);
                            if cfg.eager_end {
                                break;
                            }
                        }
                    } else {
                        prev_rv = Some(rv);
                    }
                }
                results.push((cv, min_rv));
            }
        }
        (Descending, Ascending) => {
            for cv in crate_vers.iter().filter(|v| !(cfg.skip_prereleases && v.starts_with('0'))) {
                setup_crate(&cfg.tgt_crate, &tmp_dir, cv);
                let mut min_rv = None;
                for rv in STABLE_RUST_VERS.iter() {
                    println!("-> Compiling {} v{} on Rust v{}", cfg.tgt_crate, cv, rv);
                    if try_compile(rv, cfg.timeout).is_ok() {
                        println!("  => Found MSRV v{}", rv);
                        min_rv = Some(rv);
                        break;
                    }
                }
                results.push((cv, min_rv));
            }
        }
        (Descending, Descending) => {
            for cv in crate_vers.iter().filter(|v| !(cfg.skip_prereleases && v.starts_with('0'))) {
                setup_crate(&cfg.tgt_crate, &tmp_dir, cv);
                let mut min_rv = None;
                let mut prev_rv = None;
                for rv in STABLE_RUST_VERS.iter().rev() {
                    println!("-> Compiling {} v{} on Rust v{}", cfg.tgt_crate, cv, rv);
                    if try_compile(rv, cfg.timeout).is_err() {
                        if let Some(prev) = prev_rv {
                            println!("  => Found MSRV v{}", prev);
                            min_rv = Some(rv);
                            if cfg.eager_end {
                                break;
                            }
                        }
                    } else {
                        prev_rv = Some(rv);
                    }
                }
                results.push((cv, min_rv));
            }
        }
    }

    println!();
    println!();
    for (cv, min_rv) in results {
        if let Some(rv) = min_rv {
            println!("{}\t{}", cv, rv);
        } else {
            println!("{}\t None", cv);
        }
    }
}

fn setup_crate(c: &str, tmp_dir: &TempDir, cv: &str) {
    env::set_current_dir(tmp_dir).unwrap();
    let tmp_crate = &*format!("{}_{}", c, cv.replace(".", "_"));

    let _ = Command::new("cargo")
        .arg("new")
        .arg(tmp_crate)
        .status()
        .unwrap();

    let cd_path = tmp_dir.path().join(tmp_crate);
    env::set_current_dir(&cd_path).unwrap();

    let _ = Command::new("cargo")
        .arg("add")
        .arg(&*format!("{}@~{}", c, cv))
        .status().unwrap();
}

fn try_compile(rv: &str, timeout: Option<usize>) -> Result<(), ()> {
    let _ = Command::new("cargo")
        .arg("clean")
        .status().unwrap();

    let _ = fs::remove_file("Cargo.lock");

    let _ = Command::new("rustup")
        .arg("override")
        .arg("add")
        .arg(rv)
        .status().unwrap();

    let compile_res = if let Some(to) = timeout {
        Command::new("timeout")
            .arg(&*format!("{}", to))
            .arg("cargo")
            .arg("build")
            .status().unwrap()
    } else {
        Command::new("cargo")
            .arg("build")
            .status().unwrap()
    };

    if compile_res.success() {
        Ok(())
    } else {
        Err(())
    }
}
