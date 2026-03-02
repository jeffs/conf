//! Login shell launcher. See [`main`] for notes.

use std::{env, error::Error, ffi, fs, io, path::{Path, PathBuf}};

/// Writes the specified environment to a JSON object in the specified file.
///
/// The order of key/value entries is preserved only if [`serde_json`] has
/// feature `preserve_order` enabled.
fn paths_to_json(paths: &[&Path]) -> Vec<serde_json::Value> {
    paths
        .iter()
        .filter_map(|p| {
            let Some(s) = p.to_str() else {
                eprintln!("warning: skipping non-UTF-8 path item: {}", p.display());
                return None;
            };
            Some(serde_json::Value::String(s.to_owned()))
        })
        .collect()
}

fn write_json<'a>(
    env: impl IntoIterator<Item = (&'a String, &'a String)>,
    path: &[&Path],
    jump_prefixes: &[&Path],
    dest: &Path,
) -> io::Result<()> {
    let mut map = env
        .into_iter()
        .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
        .collect::<serde_json::Map<_, _>>();
    map.insert("PATH".into(), serde_json::Value::Array(paths_to_json(path)));
    map.insert(
        "JUMP_PREFIXES".into(),
        serde_json::Value::Array(paths_to_json(jump_prefixes)),
    );
    let mut json = serde_json::to_string_pretty(&map)
        .unwrap_or_else(|e| unreachable!("serde_json::Map is serializable: {e}"));
    json.push('\n');
    fs::write(dest, json)
}

fn write_sh_escaped(out: &mut Vec<u8>, s: &ffi::OsStr) {
    for &b in s.as_encoded_bytes() {
        if b == b'\'' {
            out.extend_from_slice(br"'\''");
        } else {
            out.push(b);
        }
    }
}

fn write_sh_var(out: &mut Vec<u8>, key: &ffi::OsStr, value: &ffi::OsStr) {
    out.extend_from_slice(b"export ");
    out.extend_from_slice(key.as_encoded_bytes());
    out.extend_from_slice(b"='");
    write_sh_escaped(out, value);
    out.extend_from_slice(b"'\n");
}

/// Writes the specified environment to a POSIX shell script.
fn write_sh<'a>(
    dest: &Path,
    env: impl IntoIterator<Item = (&'a String, &'a String)>,
    path: &[&Path],
    jump_prefixes: &[&Path],
) -> Result<(), Box<dyn Error>> {
    let mut out = b"# This file is generated. See ~/conf/prj/jeff-login.\n\n".to_vec();
    for (key, value) in env {
        write_sh_var(&mut out, key.as_ref(), value.as_ref());
    }
    write_sh_var(&mut out, ffi::OsStr::new("PATH"), &env::join_paths(path)?);
    write_sh_var(
        &mut out,
        ffi::OsStr::new("JUMP_PREFIXES"),
        &env::join_paths(jump_prefixes)?,
    );
    Ok(fs::write(dest, out)?)
}

/// # Panics
///
/// Will panic on file output errors or if the platform config cannot be loaded.
fn main() {
    // Load OS-specific values.
    let home = std::env::home_dir().expect("home dir");
    let conf = home.join("conf");
    let platform = platform::Platform::load(&conf).expect("loading platform config");
    let path = platform.full_path();
    let jump_prefixes: Vec<&Path> = platform
        .paths
        .jump_prefixes
        .iter()
        .map(PathBuf::as_path)
        .collect();

    // Make sure the destination directory exists.
    let var = conf.join("var");
    fs::create_dir_all(&var).unwrap_or_else(|e| panic!("{}: {e}", var.display()));

    // Save JSON for Nushell and Xonsh.
    let json = var.join("env.json");
    write_json(&platform.env, &path, &jump_prefixes, &json)
        .unwrap_or_else(|e| panic!("{}: {e}", json.display()));

    // Save exports for POSIX shells.
    let sh = var.join("env.sh");
    write_sh(&sh, &platform.env, &path, &jump_prefixes)
        .unwrap_or_else(|e| panic!("{}: {e}", sh.display()));
}
