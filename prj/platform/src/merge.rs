use toml::Table;
use toml::Value;

/// Deep-merge `overlay` into `base`.
///
/// - Nested tables merge recursively (key by key).
/// - Scalars and arrays in `overlay` replace the corresponding `base` value.
pub fn deep_merge(base: &mut Table, overlay: Table) {
    for (key, overlay_val) in overlay {
        match (base.get_mut(&key), overlay_val) {
            (Some(Value::Table(base_tbl)), Value::Table(overlay_tbl)) => {
                deep_merge(base_tbl, overlay_tbl);
            }
            (_, overlay_val) => {
                base.insert(key, overlay_val);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalars_override() {
        let mut base: Table = toml::from_str(r#"key = "base""#).unwrap();
        let overlay: Table = toml::from_str(r#"key = "overlay""#).unwrap();
        deep_merge(&mut base, overlay);
        assert_eq!(base["key"].as_str().unwrap(), "overlay");
    }

    #[test]
    fn nested_tables_merge() {
        let mut base: Table = toml::from_str(
            r#"
            [env]
            A = "1"
            B = "2"
            "#,
        )
        .unwrap();
        let overlay: Table = toml::from_str(
            r#"
            [env]
            B = "override"
            C = "3"
            "#,
        )
        .unwrap();
        deep_merge(&mut base, overlay);

        let env = base["env"].as_table().unwrap();
        assert_eq!(env["A"].as_str().unwrap(), "1");
        assert_eq!(env["B"].as_str().unwrap(), "override");
        assert_eq!(env["C"].as_str().unwrap(), "3");
    }

    #[test]
    fn lists_replace_entirely() {
        let mut base: Table = toml::from_str(
            r#"
            [paths]
            system_paths = ["/usr/bin", "/bin"]
            "#,
        )
        .unwrap();
        let overlay: Table = toml::from_str(
            r#"
            [paths]
            system_paths = ["/custom/bin"]
            "#,
        )
        .unwrap();
        deep_merge(&mut base, overlay);

        let arr = base["paths"]["system_paths"].as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0].as_str().unwrap(), "/custom/bin");
    }

    #[test]
    fn overlay_adds_new_keys() {
        let mut base: Table = toml::from_str(r#"a = 1"#).unwrap();
        let overlay: Table = toml::from_str(r#"b = 2"#).unwrap();
        deep_merge(&mut base, overlay);
        assert_eq!(base["a"].as_integer().unwrap(), 1);
        assert_eq!(base["b"].as_integer().unwrap(), 2);
    }
}
