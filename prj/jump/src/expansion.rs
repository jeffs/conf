use std::path::{Component, Path, PathBuf};

enum Expansion<'a, 'b> {
    Path(&'a Path),
    Component(Component<'b>),
    String(String),
}

impl AsRef<Path> for Expansion<'_, '_> {
    fn as_ref(&self) -> &Path {
        match self {
            Self::Path(p) => p,
            Self::Component(c) => c.as_ref(),
            Self::String(s) => Path::new(s),
        }
    }
}

pub struct Expander<'a> {
    home: &'a Path,
}

impl<'a> Expander<'a> {
    pub fn with_home(home: &'a Path) -> Self {
        Self { home }
    }

    fn expand_component<'b>(&self, part: Component<'b>) -> Expansion<'a, 'b> {
        let Component::Normal(s) = part else {
            return Expansion::Component(part);
        };

        let Some(s) = s.to_str() else {
            return Expansion::Component(part);
        };

        if s.starts_with('%') {
            let today = chrono::Local::now().date_naive();
            Expansion::String(today.format(s).to_string())
        } else if s == "~" {
            Expansion::Path(self.home)
        } else {
            Expansion::Component(part)
        }
    }

    pub fn expand(&self, path: &Path) -> PathBuf {
        path.components()
            .map(|c| self.expand_component(c))
            .collect()
    }
}
