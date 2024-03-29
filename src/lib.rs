use serde::*;

use std::fs;
use std::ffi::OsString;
use std::path::*;
use std::process::*;



macro_rules! fatal {
    ( $($tt:tt)* ) => {{
        eprintln!($($tt)*);
        std::process::exit(1);
    }};
}

#[derive(Deserialize)] struct Metadata { packages: Vec<Package> }
#[derive(Deserialize)] struct Package { name: String, manifest_path: PathBuf }

pub fn metabuild() {
    if !std::env::var("TARGET").ok().unwrap_or(String::new()).ends_with("-msvc") {
        return; // Not an -msvc target, better not use MSVC specific linker flags.
    }

    let output = Command::new("cargo").arg("metadata").arg("--format-version").arg("1").stderr(Stdio::inherit()).output();
    let output = output.unwrap_or_else(|err| fatal!("natvis-pdbs failed to execute `cargo metadata --format-version 1`: {}", err));
    match output.status.code() {
        Some(0) => {},
        Some(n) => fatal!("`cargo metadata --format-version 1`: exit code {}", n),
        None    => fatal!("`cargo metadata --format-version 1`: terminated by signal"),
    }

    let metadata : Metadata = serde_json::from_slice(&output.stdout[..]).unwrap_or_else(|err| fatal!("natvis-pdbs failed to parse `cargo metadata --format-version 1`: {}", err));

    println!("cargo:rerun-if-env-changed=LINK");
    let mut link_args = std::env::var_os("LINK").unwrap_or(OsString::new());
    let mut link_args_changed = false;

    for package in metadata.packages.iter() {
        let root = if let Some(p) = package.manifest_path.parent() {
            p
        } else {
            println!("cargo:warning=Package {:?}'s manifest_path {:?} has no parent directory, skipping in search for natvis files...", &package.name, &package.manifest_path);
            continue
        };

        let src = root.join("src");

        let dm  = root.join("debug_metadata");
        //  https://github.com/Lokathor/tinyvec/pull/167#issuecomment-1238471549
        //  https://github.com/bluss/arrayvec/pull/225/files

        for (dir,           warn_if_missing ) in [
            (root,          true            ),
            (src.as_path(), false           ),
            (dm.as_path(),  false           ),
        ].iter().copied() {
            let entries = if let Ok(e) = fs::read_dir(dir) {
                e
            } else if warn_if_missing {
                println!("cargo:warning={:?} not readable, skipping in search for natvis files...", dir);
                continue
            } else {
                // It is suprisingly common to manually specify the source path to flatten out / not to have a `src` dir
                // (cloudabi, osmesa-sys, html5ever-atoms, gl_generator, percent-encoding, utf-8, fnv, smallvec, ...)
                continue
            };

            println!("cargo:rerun-if-changed={}", dir.display());

            for entry in entries {
                let entry = if let Ok(e) = entry { e } else { continue };
                let path = entry.path();
                if !path.is_file() { continue }
                let ext = if let Some(e) = path.extension().and_then(|os| os.to_str()) { e } else { continue };
                if !ext.eq_ignore_ascii_case("natvis") { continue }

                // Found a .natvis file, lets try to add it to link_args

                let path = path.canonicalize();
                let path = if let Some(p) = path.as_ref().ok().and_then(|p| p.to_str()) {
                    if p.starts_with("\\\\?\\") { &p[4..] } else { p } // linker cannot handle UNC paths
                } else {
                    println!("cargo:warning={:?} cannot be canonicalized, skipping embedding this natvis file...", path);
                    continue;
                };

                if link_args.len() != 0 {
                    link_args.push(" ");
                }
                link_args.push("\"/NATVIS:");
                link_args.push(path);
                link_args.push("\"");
                link_args_changed = true;
                println!("cargo:rerun-if-changed={}", path);
            }
        }
    }

    if link_args_changed {
        std::env::set_var("LINK", &link_args);
        println!("cargo:rustc-env=LINK={}", link_args.to_string_lossy());
    }
}
