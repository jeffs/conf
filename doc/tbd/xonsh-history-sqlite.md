# Xonsh: Migrate history from JSON to SQLite

## Context

- `$XONSH_HISTORY_BACKEND = 'sqlite'` is set in `etc/xonsh/rc.d/config.py`
- `history transfer xonsh` is buggy (KeyError on `rtn` field)
- Manual migration script needed instead

## Steps

1. Start and exit a xonsh session so the SQLite DB is created at
   `~/.local/share/xonsh/xonsh-history.sqlite`.

2. Run the migration script (from bash or as `python3 script.py`):

   ```python
   import json, sqlite3, glob, os

   hist_dir = os.path.expanduser("~/.local/share/xonsh")
   db_path = os.path.join(hist_dir, "xonsh-history.sqlite")

   conn = sqlite3.connect(db_path)
   cur = conn.cursor()

   for path in glob.glob(os.path.join(hist_dir, "xonsh-*.json")):
       try:
           with open(path) as f:
               data = json.load(f)
       except (json.JSONDecodeError, KeyError):
           continue
       sessionid = data.get("sessionid", "")
       for cmd in data.get("data", {}).get("cmds", []):
           cur.execute(
               "INSERT OR IGNORE INTO xonsh_history (inp, rtn, tsb, tse, sessionid) VALUES (?, ?, ?, ?, ?)",
               (
                   cmd.get("inp", ""),
                   cmd.get("rtn", 0),
                   cmd.get("tsb", 0.0),
                   cmd.get("tse", 0.0),
                   sessionid,
               ),
           )

   conn.commit()
   conn.close()
   ```

3. Verify: open xonsh, run `history show` and confirm old commands appear.

4. Clean up: `rm ~/.local/share/xonsh/xonsh-*.json`

## References

- https://github.com/xonsh/xonsh/discussions/4132
- https://github.com/xonsh/xonsh/issues/4001
