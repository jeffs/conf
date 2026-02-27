//! Login shell launcher. See [`main`] for notes.

use std::{
    env, ffi, fs, io,
    path::{Path, PathBuf},
};

fn path_join<P: AsRef<Path>, I: IntoIterator<Item = P>>(dirs: I) -> ffi::OsString {
    let mut dirs = dirs.into_iter();
    let path = dirs
        .next()
        .map(|p| p.as_ref().as_os_str().to_os_string())
        .unwrap_or_default();
    dirs.fold(path, |mut path, dir| {
        path.push(":");
        path.push(dir.as_ref());
        path
    })
}

fn build_path(home: &Path) -> Vec<PathBuf> {
    [
        "usr/bin",
        "conf/bin",
        ".local/bin",
        ".cargo/bin",
        "go/bin",
        "/usr/local/go/bin",
        "/opt/homebrew/bin",
        "/opt/homebrew/opt/sqlite/bin",
        "/usr/local/bin",
        "/usr/bin",
        "/bin",
        "/usr/sbin",
        "/sbin",
        "/Library/Developer/CommandLineTools/usr/bin",
    ]
    .iter()
    .map(|dir| {
        if dir.starts_with('/') {
            PathBuf::from(dir)
        } else {
            home.join(dir)
        }
    })
    .collect()
}

fn into_json_map_entry((k, v): (&str, String)) -> (String, serde_json::Value) {
    let v = serde_json::to_value(v)
        .unwrap_or_else(|e| unreachable!("any Rust String should be valid JSON: {e}"));
    (k.to_owned(), v)
}

/// Writes the specified environment to a JSON object in the specified file.
///
/// # Notes
///
/// The order of key/value entries is preserved only if [`serde_json`] has
/// feature `preserve_order` enabled. (This is unrelated to the order of
/// directories within `PATH`, which is always preserved.)
fn write_json<'a>(
    env: impl IntoIterator<Item = (&'a str, String)>,
    path: impl IntoIterator<Item = String>,
    dest: &Path,
) -> io::Result<()> {
    let mut map = env
        .into_iter()
        .map(into_json_map_entry)
        .collect::<serde_json::Map<_, _>>();
    let path = serde_json::Value::Array(path.into_iter().map(serde_json::Value::String).collect());
    map.insert("PATH".into(), path);
    let mut json = serde_json::to_string_pretty(&map).unwrap_or_else(|e|
            // Serialization could theoretically fail on excessive recursion.
            // In our case, the values are only strings and a list of paths.
            unreachable!("serde_json::Map is serializable as String: {e}"));
    json.push('\n');
    fs::write(dest, json)
}

fn escape_sh(s: &ffi::OsStr, escaped: &mut Vec<u8>) {
    for &b in s.as_encoded_bytes() {
        if b == b'\'' {
            escaped.extend_from_slice(br"'\''");
        } else {
            escaped.push(b);
        }
    }
}

fn write_sh_var(key: &ffi::OsStr, value: &ffi::OsStr, output: &mut Vec<u8>) {
    output.extend_from_slice(b"export ");
    output.extend_from_slice(key.as_encoded_bytes());
    output.extend_from_slice(b"='");
    escape_sh(value, output);
    output.extend_from_slice(b"'\n");
}

/// Writes the specified environment to a POSIX shell script.
fn write_sh(
    env: impl IntoIterator<Item = (impl AsRef<ffi::OsStr>, impl AsRef<ffi::OsStr>)>,
    path: &[PathBuf],
    dest: &Path,
) -> io::Result<()> {
    let mut output = b"# This file is generated. See ~/conf/prj/jeff-login.\n\n".to_vec();
    for (key, value) in env {
        write_sh_var(key.as_ref(), value.as_ref(), &mut output);
    }
    write_sh_var(ffi::OsStr::new("PATH"), &path_join(path), &mut output);
    fs::write(dest, output)
}

/// # Panics
///
/// Will panic on file output errors.
///
/// # Notes
///
/// Non-UTF-8 values are skipped (with warnings) in JSON serialization.
fn main() {
    let home = env::home_dir().expect("home dir");

    // Env vars whose values are relative to your home directory.
    let home_env = [
        ("EDITOR", ".cargo/bin/hx"),
        ("VISUAL", ".cargo/bin/hx"),
        ("XDG_CONFIG_HOME", ".config"),
        ("FZF_DEFAULT_OPTS_FILE", "conf/etc/fzf"),
        ("RIPGREP_CONFIG_PATH", "conf/etc/ripgreprc"),
        (
            "COPILOT_CUSTOM_INSTRUCTIONS_DIRS",
            "conf/etc/copilot/instructions.md",
        ),
        ("JUMP_PREFIXES", "conf/etc/jump"),
        ("HELIX_RUNTIME", "usr/src/helix/runtime"),
        ("JUMP_HOME", ""),
    ]
    .map(|(k, v)| (k, home.join(v)));

    // An env var whose value *is* your home directory.
    let jump_home = ("JUMP_HOME", home.as_os_str());

    // Env vars whose values are static strings.
    let static_env = [
        ("LESS", "-FRX -j5"),
        ("MANPAGER", "col -b | bat -pl man"),
        ("HOMEBREW_NO_ENV_HINTS", "true"),
        ("RUSTC_WRAPPER", "/opt/homebrew/bin/sccache"),
        ("ENABLE_LSP_TOOL", "1"),
        ("ENABLE_LSP_TOOLS", "1"),
        ("GRIT_TRUNKS", "dev,main,master,trunk"),
    ];

    // Represent `PATH` as a list for JSON. We'll colon-separate it for POSIX.
    let path = build_path(&home);

    let env = home_env
        .iter()
        .map(|(k, v)| (*k, v.as_os_str()))
        .chain(std::iter::once(jump_home))
        .chain(static_env.map(|(k, v)| (k, ffi::OsStr::new(v))));

    // Make sure the destination directory exists.
    let var_dir = home.join("conf/var");
    fs::create_dir_all(&var_dir).map_err(|e| panic!("{}: {e}", var_dir.display()));

    // Save JSON for Nushell and Xonsh.
    //
    // JSON doesn't really support non-UTF-8 encodings. Nushell panics if any
    // inherited environment variable contains non-UTF characters, anyway: It
    // has no equivalent of Rust [`std::env::var_os`]. Python has `os.environb`,
    // but Xonsh doesn't seem to use it. (Xonsh also doesn't crash if vars have
    // non-Unicode values; it presents them as strings containing `\u` escape
    // sequences.)
    let json_file = var_dir.join("env.json");
    let env_strings = env.clone().filter_map(|(k, v)| {
        let Some(v) = v.to_str() else {
            eprintln!("warning: skipping non-UTF-8 value of {k}: {}", v.display());
            return None;
        };
        Some((k, v.to_owned()))
    });

    let path_strings = path.iter().filter_map(|p| {
        let Some(p) = p.to_str() else {
            eprintln!("warning: skipping non-UTF-8 PATH item: {}", p.display());
            return None;
        };
        Some(p.to_owned())
    });

    write_json(env_strings, path_strings, &json_file)
        .map_err(|e| panic!("{}: {e}", json_file.display()));

    // Save exports for POSIX shells.
    let sh_file = var_dir.join("env.sh");
    write_sh(env, &path, &sh_file).map_err(|e| panic!("{}: {e}", sh_file.display()));
}
