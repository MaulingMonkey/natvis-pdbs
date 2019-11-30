use cargo_metadata::*;
use std::fs;
use std::ffi::OsString;

pub fn metabuild() {
    if !std::env::var("TARGET").ok().unwrap_or(String::new()).ends_with("-msvc") {
        return; // Not an -msvc target, better not use MSVC specific linker flags.
    }

    let metadata = match MetadataCommand::new().exec() {
        Ok(md) => md,
        Err(e) => {
            eprintln!("natvis-pdbs failed to execute/parse 'cargo metadata': {}", e);
            std::process::exit(1);
        },
    };

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

        for (dir,           warn_if_missing ) in [
            (root,          true            ),
            (src.as_path(), false           ),
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
