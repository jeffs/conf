from pathlib import Path
from typing import cast
import json
import os
import subprocess
import sys

from xonsh.built_ins import XSH
from xonsh.environ import Env, EnvPath

env = cast(Env, XSH.env)


def load_env():
    try:
        env_json = json.loads(Path("~/conf/var/env.json").expanduser().read_text())
        env.update(env_json)

        # TODO: Can't the Xonsh environment handle rich data? Why stringify everything?
        def format(v):
            if isinstance(v, str):
                return v
            if isinstance(v, list):
                return os.pathsep.join(v)
            if isinstance(v, bool):
                return "true" if v else "false"
            return str(v)

        os.environ.update({k: format(v) for k, v in env_json.items()})
    except Exception as e:
        print(f"error: env: {e}", file=sys.stderr)


def load_fnm():
    # FNM is a version manager for Node.js.
    try:
        fnm = "/opt/homebrew/bin/fnm"
        fnm_env = json.loads(
            subprocess.run(
                [fnm, "env", "--json"],
                capture_output=True,
                text=True,
                check=True,
            ).stdout
        )
        env.update(fnm_env)
        os.environ.update(fnm_env)
        fnm_bin = f"{fnm_env['FNM_MULTISHELL_PATH']}/bin"
        cast(EnvPath, env["PATH"]).insert(0, fnm_bin)
        os.environ["PATH"] = os.pathsep.join([fnm_bin, os.environ["PATH"]])
        if "NVMRC" in os.environ:
            version = Path(os.environ["NVMRC"]).read_text()
            command = [fnm, "--log-level=quiet", "use", version]
            subprocess.run(command, check=True)
    except Exception as e:
        print(f"error: fnm: {e}", file=sys.stderr)
