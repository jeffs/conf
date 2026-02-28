"""TODO: Move this whole module to Rust."""

from collections.abc import Iterable
from datetime import datetime
from itertools import chain
from pathlib import Path
import stat

import polars as pl


def _mode_name(st_mode: int) -> str | None:
    # Order matters, because symlinks.
    if stat.S_ISLNK(st_mode):
        return "symlink"
    if stat.S_ISDIR(st_mode):
        return "dir"
    if stat.S_ISREG(st_mode):
        return "file"
    if stat.S_ISFIFO(st_mode):
        return "fifo"
    if stat.S_ISSOCK(st_mode):
        return "socket"
    if stat.S_ISBLK(st_mode):
        return "block"
    if stat.S_ISCHR(st_mode):
        return "char"
    return None


def _iterate(path: Path) -> Iterable[Path]:
    """
    Return the contents of the specified path if it's a directory, and the path
    itself otherwise.
    """
    return path.iterdir() if path.is_dir() else (path,)


def _make_row(path: Path) -> dict[str, object]:
    """
    Return metadata of the specified file or directory.

    # TODO

    Check `stat.st_mode` for file type, such as symlink.
    """
    path_stat = path.stat(follow_symlinks=False)
    return {
        "name": str(path),
        "type": _mode_name(path_stat.st_mode) or "?",
        "size": path_stat.st_size,
        "modified": datetime.fromtimestamp(path_stat.st_mtime),
    }


def ls(*paths: Path | str) -> pl.DataFrame:
    """
    Return a dataframe representing the specified paths. Note that directories
    are list directly, rather than accessing their contents like `/bin/ls`.
    (This is more like Nushell's built-in `ls`.)
    """
    iters = (_iterate(Path(p)) for p in (paths or (".",)))
    return pl.DataFrame(map(_make_row, chain.from_iterable(iters)))
