#!/usr/bin/env python3
"""Stop hook: demand a shorter answer when a closed question got a long one."""

import json
import re
import sys

WORD_LIMIT = 100

AUXILIARIES = frozenset(
    """
    do does did is are was were can could will would should has have any
    """.split()
)


def opens_closed_question(text: str) -> bool:
    words = re.findall(r"[A-Za-z']+", text)
    return bool(words) and words[0].lower() in AUXILIARIES


def word_count(text: str) -> int:
    return len(text.split())


def entries(transcript: str):
    try:
        with open(transcript, encoding="utf-8") as lines:
            for line in lines:
                line = line.strip()
                if line:
                    try:
                        yield json.loads(line)
                    except json.JSONDecodeError:
                        continue
    except OSError:
        return


def text_of(message: dict) -> str:
    content = message.get("content")
    if isinstance(content, str):
        return content
    if isinstance(content, list):
        return " ".join(
            block.get("text", "")
            for block in content
            if isinstance(block, dict) and block.get("type") == "text"
        )
    return ""


def last_prompt_and_reply(transcript: str) -> tuple[str, str]:
    """The last human turn and the assistant text that answered it."""
    prompt, reply = "", ""
    for entry in entries(transcript):
        if entry.get("isSidechain") or entry.get("isMeta"):
            continue
        message = entry.get("message")
        if not isinstance(message, dict):
            continue
        if entry.get("type") == "user":
            if "toolUseResult" in entry:
                continue
            body = text_of(message)
            if body:
                prompt, reply = body, ""
        elif entry.get("type") == "assistant":
            body = text_of(message)
            if body:
                reply = f"{reply} {body}".strip()
    return prompt, reply


def main() -> int:
    try:
        event = json.load(sys.stdin)
    except json.JSONDecodeError:
        return 0

    if event.get("stop_hook_active"):
        return 0

    prompt, reply = last_prompt_and_reply(event.get("transcript_path", ""))
    reply = event.get("last_assistant_message") or reply

    if not opens_closed_question(prompt) or word_count(reply) <= WORD_LIMIT:
        return 0

    print(
        f"That was a yes/no question and your answer ran {word_count(reply)} "
        f"words. Rewrite it in {WORD_LIMIT} words or fewer, opening with yes, "
        "no, or the direct answer. No preamble and no elaboration.",
        file=sys.stderr,
    )
    return 2


if __name__ == "__main__":
    sys.exit(main())
