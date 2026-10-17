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
