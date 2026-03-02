//! Login shell launcher. See [`main`] for notes.

use std::{ffi, fs, io, path::Path};

fn path_join(dirs: &[&Path]) -> ffi::OsString {
    let mut dirs = dirs.iter();
    let Some(first) = dirs.next() else {
        return ffi::OsString::new();
    };
    dirs.fold(first.as_os_str().to_os_string(), |mut acc, dir| {
        acc.push(":");
        acc.push(dir);
        acc
    })
}

/// Writes the specified environment to a JSON object in the specified file.
///
/// The order of key/value entries is preserved only if [`serde_json`] has
/// feature `preserve_order` enabled.
fn write_json<'a>(
    env: impl IntoIterator<Item = (&'a String, &'a String)>,
    path: &[&Path],
    dest: &Path,
) -> io::Result<()> {
    let mut map = env
        .into_iter()
        .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
        .collect::<serde_json::Map<_, _>>();
    let path_values: Vec<_> = path
        .iter()
        .filter_map(|p| {
            let Some(s) = p.to_str() else {
                eprintln!("warning: skipping non-UTF-8 PATH item: {}", p.display());
                return None;
            };
            Some(serde_json::Value::String(s.to_owned()))
        })
        .collect();
    map.insert("PATH".into(), serde_json::Value::Array(path_values));
    let mut json = serde_json::to_string_pretty(&map)
        .unwrap_or_else(|e| unreachable!("serde_json::Map is serializable: {e}"));
    json.push('\n');
    fs::write(dest, json)
}

fn escape_sh(s: &ffi::OsStr, out: &mut Vec<u8>) {
    for &b in s.as_encoded_bytes() {
        if b == b'\'' {
            out.extend_from_slice(br"'\''");
        } else {
            out.push(b);
        }
    }
}

fn write_sh_var(key: &ffi::OsStr, value: &ffi::OsStr, out: &mut Vec<u8>) {
    out.extend_from_slice(b"export ");
    out.extend_from_slice(key.as_encoded_bytes());
    out.extend_from_slice(b"='");
    escape_sh(value, out);
    out.extend_from_slice(b"'\n");
}

/// Writes the specified environment to a POSIX shell script.
fn write_sh<'a>(
    env: impl IntoIterator<Item = (&'a String, &'a String)>,
    path: &[&Path],
    dest: &Path,
) -> io::Result<()> {
    let mut out = b"# This file is generated. See ~/conf/prj/jeff-login.\n\n".to_vec();
    for (key, value) in env {
        write_sh_var(key.as_ref(), value.as_ref(), &mut out);
    }
    write_sh_var(ffi::OsStr::new("PATH"), &path_join(path), &mut out);
    fs::write(dest, out)
}

/// # Panics
///
/// Will panic on file output errors or if the platform config cannot be loaded.
fn main() {
    #[allow(deprecated)]
    let home = std::env::home_dir().expect("home dir");
    let conf_root = home.join("conf");

    let platform =
        platform::Platform::load(&conf_root).unwrap_or_else(|e| panic!("platform config: {e}"));
    let path = platform.full_path();

    let var_dir = conf_root.join("var");
    fs::create_dir_all(&var_dir).unwrap_or_else(|e| panic!("{}: {e}", var_dir.display()));

    // Save JSON for Nushell and Xonsh.
    let json_file = var_dir.join("env.json");
    write_json(&platform.env, &path, &json_file)
        .unwrap_or_else(|e| panic!("{}: {e}", json_file.display()));

    // Save exports for POSIX shells.
    let sh_file = var_dir.join("env.sh");
    write_sh(&platform.env, &path, &sh_file)
        .unwrap_or_else(|e| panic!("{}: {e}", sh_file.display()));
}
