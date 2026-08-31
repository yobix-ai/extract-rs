#!/usr/bin/env python3
"""Document trust verification — optional companion to Extractous extraction.

Extractous extracts what a document *says*. This example adds the adjacent
question: **can the document itself be trusted?** A tampered invoice or an
AI-generated fake report extracts just as fast and clean as a real one —
the extracted text is identical in quality while the underlying document is
fraudulent. For RAG pipelines, attaching a trust signal at extraction time
means the index can carry provenance metadata from day one.

Uses the Stipple API (https://www.stipple.sh, free anonymous tier — no API
key required; set STIPPLE_API_KEY for your own metering). All functions are
best-effort: failures return None and never break extraction.

Usage:
    python examples/verify_document.py report.pdf
    python examples/verify_document.py report.pdf --json
"""
import json
import os
import sys
import urllib.request
import uuid
from pathlib import Path
from typing import Optional

STIPPLE_BASE_URL = os.getenv("STIPPLE_BASE_URL", "https://www.stipple.sh")
TIMEOUT = 300  # seconds


def _headers() -> dict:
    headers = {"User-Agent": "extractous-verify/1.0", "Accept": "application/json"}
    api_key = os.getenv("STIPPLE_API_KEY", "").strip()
    if api_key:
        headers["Authorization"] = "Bearer " + api_key
    return headers


def _post_file(endpoint: str, file_path: str) -> Optional[dict]:
    """POST a document as multipart to a Stipple endpoint. Best-effort."""
    try:
        path = Path(file_path)
        boundary = "----extractous-verify" + uuid.uuid4().hex
        with open(path, "rb") as f:
            content = f.read()
        body = b"".join(
            [
                (
                    f"--{boundary}\r\n"
                    f'Content-Disposition: form-data; name="file"; '
                    f'filename="{path.name}"\r\n'
                    "Content-Type: application/octet-stream\r\n\r\n"
                ).encode(),
                content,
                b"\r\n",
                f"--{boundary}--\r\n".encode(),
            ]
        )
        req = urllib.request.Request(
            STIPPLE_BASE_URL + endpoint,
            data=body,
            method="POST",
            headers={
                **_headers(),
                "Content-Type": f"multipart/form-data; boundary={boundary}",
            },
        )
        with urllib.request.urlopen(req, timeout=TIMEOUT) as resp:
            return json.loads(resp.read().decode())
    except Exception as e:  # noqa: BLE001 - verification is best-effort
        print(f"verification unavailable: {e}", file=sys.stderr)
        return None


def verify_document(file_path: str) -> Optional[dict]:
    """Forensic authenticity + AI-text probability. Block or None on failure."""
    block: dict = {}
    warrant = _post_file("/v1/warrants", file_path)
    if warrant:
        block["authenticity"] = {
            "warrant_id": warrant.get("warrant_id"),
            "risk_band": warrant.get("risk_band"),
            "risk_score": warrant.get("risk_score"),
            "inspection_quality": warrant.get("inspection_quality"),
            "recommended_action": warrant.get("recommended_action"),
            "summary": warrant.get("summary"),
        }
    ai = _post_file("/v1/detect-ai-text", file_path)
    if ai:
        block["ai_text"] = (
            {"applicable": False}
            if ai.get("applicable") is False
            else {
                "applicable": True,
                "probability": ai.get("probability"),
                "lean": ai.get("lean"),
                "tells": ai.get("tells"),
            }
        )
    return block or None


def main():
    if len(sys.argv) != 2:
        print(f"Usage: '{sys.argv[0]}' <filename>")
        sys.exit(1)
    in_file = sys.argv[1]
    if not os.path.isfile(in_file):
        raise FileNotFoundError(f"No such file: '{in_file}'")

    block = verify_document(in_file)
    if block is None:
        print(json.dumps({"error": "verification unavailable"}, indent=2))
        sys.exit(1)

    if "--json" in sys.argv:
        print(json.dumps(block, indent=2))
    else:
        auth = block.get("authenticity", {})
        print(f"risk_band:          {auth.get('risk_band', '?').upper()}")
        print(f"inspection_quality: {auth.get('inspection_quality', '?')}")
        print(f"recommended_action: {auth.get('recommended_action', '?')}")
        if auth.get("summary"):
            print(f"summary: {auth['summary'][:200]}")
        if block.get("ai_text"):
            print(f"ai_text: {block['ai_text']}")
        print(f"\nwarrant: {auth.get('warrant_id', '?')} (cached by content hash — re-checks are free)")


if __name__ == "__main__":
    main()
