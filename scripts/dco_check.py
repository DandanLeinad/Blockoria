# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 DandanLeinad

#!/usr/bin/env python3
"""DCO check for commit-msg hook - validates commit message has Signed-off-by."""

import os
import sys


def main():
    # pre-commit passes commit message file as first arg for commit-msg hooks
    # But sometimes it might not, so check env var too
    msg_file = (
        sys.argv[1]
        if len(sys.argv) > 1
        else os.environ.get("PRE_COMMIT_COMMIT_MSG_FILENAME")
    )

    if not msg_file:
        print("Error: missing commit message file argument", file=sys.stderr)
        return 1

    try:
        with open(msg_file, encoding="utf-8") as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {msg_file}: {e}", file=sys.stderr)
        return 1

    if "Signed-off-by:" not in content:
        print("Commit message missing Signed-off-by")
        print("Use: git commit -s")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
