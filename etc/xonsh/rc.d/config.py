def load(path: str):
    """Reads an object from a JSON or YAML 1.2 file."""
    import json
    from ruamel.yaml import YAML

    with open(path) as stream:
        if path.endswith(".json"):
            return json.load(stream)
        elif path.endswith(".yaml") or path.endswith(".yml"):
            return YAML().load(stream)
        else:
            raise Exception("unrecognized file type")


def main():
    from pathlib import Path
    from typing import cast
    import sys

    from xonsh.built_ins import XSH
    from xonsh.environ import Env

    sys.path.insert(0, str(Path.home() / "conf/etc/xonsh/modules"))

    env = cast(Env, XSH.env)

    if "JEFF_LOGIN_DONE" not in env:
        env["JEFF_LOGIN_DONE"] = True
        import login

        login.load_env()
        login.load_fnm()

    if env.get("XONSH_INTERACTIVE"):
        import interactive

        interactive.configure()


main()

del main
