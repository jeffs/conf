use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl<const N: usize> AsBytes for [u8; N] {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl AsBytes for OsStr {
    fn as_bytes(&self) -> &[u8] {
        self.as_encoded_bytes()
    }
}

/// We can't `impl<P: AsRef<Path>> AsBytes for P`, because, according to rustc:
///
/// > conflicting implementations of trait `AsBytes` for type `[u8; _]`
/// >
/// > upstream crates may add a new impl of trait `std::convert::AsRef<std::path::Path>` for type
/// > `[u8; _]` in future versions
///
/// In other words:  We can't have both a specific implementation for arrays, and a blank
/// implementation for paths, because _some future version_ of the upstream implementation might
/// make arrays [`AsRef<Path>`]. But neither can we remove the specific implementation for arrays,
/// because no such expansion has yet taken place.  So, we can't have nice things; viz. a blanket
/// implementation for [`Path`], [`PathBuf`], and [`Component`], as well as an implementation (blank
/// or otherwise) for arrays.  (The same applies to slices, in case you were wondering.)
impl AsBytes for Path {
    fn as_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }
}

impl AsBytes for PathBuf {
    fn as_bytes(&self) -> &[u8] {
        self.as_path().as_bytes()
    }
}

impl AsBytes for Component<'_> {
    fn as_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }
}
