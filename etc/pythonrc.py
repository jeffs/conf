import os
from typing import Final


def init_platform() -> None:
    """Lets Camelot find Ghostscript.

    >>> from ctypes.util import find_library
    >>> find_library("gs")
    '/opt/homebrew/lib/libgs.dylib'
    """

    key = "DYLD_LIBRARY_PATH"
    val = "/opt/homebrew/lib"
    os.environ[key] = f"{old}:{val}" if (old := os.environ.get(key)) else val


init_platform()
del init_platform

JENNY: Final[tuple[int, ...]] = (8, 6, 7, 5, 3, 0, 9)
