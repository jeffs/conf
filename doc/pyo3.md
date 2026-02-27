# Writing Python Packages in Rust (PyO3 + maturin)

## Toolchain

| Component | Current Version | Purpose |
|-----------|----------------|---------|
| [PyO3](https://pyo3.rs/) | 0.28.0 | Rust <-> Python bindings |
| [maturin](https://www.maturin.rs/) | 1.12.4 | Build, develop, publish |

Minimum Rust: **1.83**. Python: **3.7+**.

## Quickstart

```bash
# Install maturin (uv or pipx both work)
uv tool install maturin

# Scaffold a new project
maturin new -b pyo3 my_fast_lib
cd my_fast_lib
```

This gives you:

```
my_fast_lib/
├── Cargo.toml        # crate-type = ["cdylib"], pyo3 dep
├── pyproject.toml    # maturin as build backend
└── src/
    └── lib.rs        # your Rust code
```

## Minimal example

**`Cargo.toml`**
```toml
[package]
name = "my_fast_lib"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_fast_lib"
crate-type = ["cdylib"]

[dependencies]
pyo3 = "0.28.0"
```

**`src/lib.rs`**
```rust
#[pyo3::pymodule]
mod my_fast_lib {
    use pyo3::prelude::*;

    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
```

## Dev loop

```bash
# Build and install into the active venv
maturin develop

# Then in Python/Xonsh/notebook:
import my_fast_lib
my_fast_lib.sum_as_string(5, 20)  # => '25'
```

`maturin develop` compiles the Rust and installs the resulting `.so` directly into your current virtualenv. Rebuild after changes by re-running it.

## For Xonsh and notebooks specifically

Both just need the package installed in the same venv they're running from:

- **Xonsh**: activate your venv, run `maturin develop`, then `import my_fast_lib` works directly in Xonsh sessions.
- **Jupyter**: make sure the notebook kernel points to the same venv. After `maturin develop`, the import works in cells. If you change the Rust code, rebuild with `maturin develop` and restart the kernel (or use `importlib.reload` for simple cases).

## Returning richer types

PyO3 handles automatic conversion for common types -- `Vec<T>` <-> `list`, `HashMap` <-> `dict`, `Option<T>` <-> `None`-or-value, numpy arrays via [rust-numpy](https://github.com/PyO3/rust-numpy). For exposing classes:

```rust
#[pyo3::pymodule]
mod my_fast_lib {
    use pyo3::prelude::*;

    #[pyclass]
    struct Point {
        #[pyo3(get)]
        x: f64,
        #[pyo3(get)]
        y: f64,
    }

    #[pymethods]
    impl Point {
        #[new]
        fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }

        fn distance(&self, other: &Point) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }
}
```

## Numpy interop

Add `numpy` to your dependencies:

```toml
[dependencies]
pyo3 = "0.28.0"
numpy = "0.28"   # version-matched to pyo3
```

This lets you accept and return numpy arrays with zero-copy where possible -- useful for notebook workflows.

## References

- [PyO3 Getting Started](https://pyo3.rs/v0.28.0/getting-started.html)
- [Maturin Tutorial](https://www.maturin.rs/tutorial.html)
- [Maturin on PyPI](https://pypi.org/project/maturin/)
- [PyO3/maturin GitHub](https://github.com/PyO3/maturin)
