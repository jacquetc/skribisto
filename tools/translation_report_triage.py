#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""
Triage a translation report: label it, point at the line, flag duplicates.

Run by `.github/workflows/translation-report.yml` on every issue opened or edited
whose body carries the report marker. Skribisto's Help ▸ Report a Translation
Error… writes that marker and a small machine block into the issue form's
`report-data` field; this script reads the block and:

1. **parses it strictly.** Every value is checked against a fixed shape (a locale
   Skribisto ships, a Fluent id, a version, a file name with no path in it). A
   block that fails any check is not acted on beyond `needs-triage`: the issue
   body is written by whoever opened the issue, and nothing in it is trusted.
2. **labels the issue** `translation`, `lang: <tag>`, `kind: <slug>`,
   `area: <name>` and `upstream: teksilo` where the fix belongs in teksilo.
   Labels are only ever added: an edit never removes what a maintainer set.
3. **chooses a safe ref** to quote the line from: the release tag when the
   report came from one; otherwise the reported commit, but only once the API has
   shown it is on `dev`'s history (the API serves commits from every fork of the
   repository, and a fork's commit must never be quoted as the project's text);
   otherwise the crate version's tag; otherwise `dev`.
4. **posts one comment**, edited in place on later runs: the permalink to the
   line, the text at that ref and on `dev`, the English source, and the other
   messages of the language that read the same. Every piece of repository text
   is quoted in a code block whose fence is longer than any run of backticks in
   it, so nothing in a catalogue can turn into markup or a mention.
5. **flags duplicates** by listing the open `translation` issues and parsing
   their blocks for the same locale and message. GitHub's search splits an id
   like `edit-undo` into two words and lags behind new issues, so it is not
   used.

The block's format is a contract with the application, pinned on both sides by
`tools/fixtures/translation_report_block.txt` (the Rust test asserts the
application writes exactly it; this script's tests parse it).

Standard library only, like the other tools here, so CI needs nothing but a
Python interpreter. Run it with `python3 -I`.
"""

from __future__ import annotations

import json
import os
import re
import sys
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass, field
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import check_locales  # noqa: E402  (after the path tweak above)

BLOCK_MARKER = "skribisto-translation-report: 1"
COMMENT_MARKER = "<!-- skribisto-translation-triage -->"
FRAMEWORK_REPO = "FernTech-EU/teksilo"
DEV_BRANCH = "dev"

REPO_ROOT = Path(__file__).resolve().parent.parent
LOADER = REPO_ROOT / "crates" / "teksilo_ui" / "src" / "startup.rs"

ID_RE = re.compile(r"^[a-z0-9][a-z0-9-]*$")
VERSION_RE = re.compile(
    r"^(?P<base>\d+\.\d+\.\d+(?:-[0-9A-Za-z.]+)?)(?:\+(?P<hash>[0-9a-f]{7,40}))?$"
)
BARE_HASH_RE = re.compile(r"^[0-9a-f]{7,40}$")
CRATE_VERSION_RE = re.compile(r"^\d+\.\d+\.\d+$")
FRAMEWORK_RE = re.compile(r"^teksilo-widgets (?P<version>\d+\.\d+\.\d+)$")
WORD_RE = re.compile(r"^[a-z][a-z-]*$")

CATALOGUES = {"app", "framework", "widget-override", "export", "help"}
AREAS = {"interface", "help", "default content", "export", "unknown"}
KINDS = {
    "wrong-meaning",
    "spelling",
    "not-translated",
    "unnatural",
    "inconsistent",
    "plural-form",
    "cut-off",
    "other",
}
APP_FILES = {"main.ftl", "tooltips.ftl", "tags.ftl", "templates.ftl", "story_bible.ftl"}

# One colour per label family, so a list of issues reads at a glance.
LABEL_COLOURS = {
    "translation": "c5def5",
    "needs-triage": "ededed",
    "possible duplicate": "cfd3d7",
    "upstream: teksilo": "5319e7",
    "lang:": "1d76db",
    "kind:": "fbca04",
    "area:": "0e8a16",
}


def supported_locales(loader: Path = LOADER) -> set[str]:
    """The locales Skribisto ships, read from `SUPPORTED_LOCALES` in startup.rs,
    the list the application itself compiles."""
    text = loader.read_text(encoding="utf-8")
    match = re.search(r"SUPPORTED_LOCALES:\s*&\[&str\]\s*=\s*&\[(.*?)\];", text, re.S)
    if not match:
        raise RuntimeError(f"SUPPORTED_LOCALES not found in {loader}")
    return set(re.findall(r'"([^"]+)"', match.group(1)))


# --- The block -----------------------------------------------------------------


@dataclass
class Block:
    """A parsed and validated report block."""

    locale: str
    catalogue: str
    area: str
    file: str
    line: int
    message: str
    composed: str
    also: list[str]
    upstream: bool
    kind: str
    version: str
    crate_version: str
    framework: str  # the teksilo-widgets version, or ""
    channel: str
    os: str

    @property
    def found(self) -> bool:
        """Whether the report names a message (rather than a place it was seen)."""
        return bool(self.message)


class InvalidBlock(ValueError):
    """The block is there but does not pass validation."""


def extract_block(body: str) -> dict[str, str] | None:
    """The `key: value` lines after the marker, or None when there is no marker.

    The block may sit in the form's code block or have been pasted into a blank
    issue; either way it runs from the marker to the first line that is not a
    `key: value` line.
    """
    lines = body.replace("\r\n", "\n").split("\n")
    for start, line in enumerate(lines):
        if line.strip() == BLOCK_MARKER:
            break
    else:
        return None
    values: dict[str, str] = {}
    for line in lines[start + 1 :]:
        match = re.match(r"^([a-z][a-z-]*):(?: (.*))?$", line.strip())
        if not match:
            break
        key = match.group(1)
        if key in values:
            raise InvalidBlock(f"{key} appears twice")
        values[key] = (match.group(2) or "").strip()
    return values


def parse_block(body: str, locales: set[str]) -> Block | None:
    """The validated block in `body`, None when there is none.

    Raises InvalidBlock when the marker is there but a value is not of its shape.
    """
    raw = extract_block(body)
    if raw is None:
        return None

    def need(key: str, ok) -> str:
        value = raw.get(key, "")
        if not ok(value):
            raise InvalidBlock(f"{key}: {value!r} is not valid")
        return value

    locale = need("locale", lambda v: v in locales)
    catalogue = need("catalogue", lambda v: v == "" or v in CATALOGUES)
    area = need("area", lambda v: v in AREAS)
    message = need("message", lambda v: v == "" or bool(ID_RE.match(v)))
    found = bool(message)
    if found != bool(catalogue):
        raise InvalidBlock("a message needs a catalogue, and a catalogue a message")

    def file_ok(v: str) -> bool:
        if not found:
            return v == ""
        if "/" in v or "\\" in v or ".." in v:
            return False
        return {
            "app": v in APP_FILES,
            "framework": v == f"{locale}.ftl",
            "widget-override": v == f"{locale}.ftl",
            "export": bool(re.match(r"^[a-z]{2,3}\.ftl$", v)),
            "help": bool(re.match(r"^[a-z0-9-]+\.djot$", v)),
        }[catalogue]

    file = need("file", file_ok)
    line_text = need("line", lambda v: (not found and v == "") or bool(re.match(r"^[1-9]\d{0,6}$", v)))
    composed = need("composed", lambda v: v == "" or bool(ID_RE.match(v)))
    also_text = need(
        "also",
        lambda v: v == "" or all(ID_RE.match(x.strip()) for x in v.split(",")),
    )
    upstream = need("upstream", lambda v: v in ("", "teksilo"))
    kind = need("kind", lambda v: v in KINDS)
    version = need("version", lambda v: bool(VERSION_RE.match(v) or BARE_HASH_RE.match(v)))
    crate_version = need("crate-version", lambda v: bool(CRATE_VERSION_RE.match(v)))
    framework = need("framework", lambda v: v == "" or bool(FRAMEWORK_RE.match(v)))
    channel = need("channel", lambda v: bool(WORD_RE.match(v)))
    os_name = need("os", lambda v: bool(WORD_RE.match(v)))
    if upstream and catalogue != "framework":
        raise InvalidBlock("only a framework string is fixed upstream")

    return Block(
        locale=locale,
        catalogue=catalogue,
        area=area,
        file=file,
        line=int(line_text) if line_text else 0,
        message=message,
        composed=composed,
        also=[x.strip() for x in also_text.split(",") if x.strip()],
        upstream=bool(upstream),
        kind=kind,
        version=version,
        crate_version=crate_version,
        framework=(FRAMEWORK_RE.match(framework).group("version") if framework else ""),
        channel=channel,
        os=os_name,
    )


def labels_for(block: Block | None) -> list[str]:
    """The labels a report gets. Without a valid block, only `needs-triage`."""
    if block is None:
        return ["needs-triage"]
    labels = [
        "translation",
        f"lang: {block.locale}",
        f"kind: {block.kind}",
        f"area: {block.area}",
    ]
    if block.upstream:
        labels.append("upstream: teksilo")
    return labels


def label_colour(name: str) -> str:
    for prefix, colour in LABEL_COLOURS.items():
        if name == prefix or (prefix.endswith(":") and name.startswith(prefix)):
            return colour
    return "ededed"


def repository_path(block: Block) -> str:
    """The file's path in its repository, as the application computes it
    (`CatalogueEntry::repository_path`)."""
    return {
        "app": f"crates/teksilo_ui/locales/{block.locale}/{block.file}",
        "framework": f"crates/teksilo-widgets/locales/{block.file}",
        "widget-override": f"crates/teksilo_ui/widget_locales/{block.file}",
        "export": f"crates/skribisto_compiler/locales/{block.file}",
        "help": f"crates/teksilo_ui/help/{block.locale}/{block.file}",
    }[block.catalogue]


def english_path(block: Block) -> str | None:
    """Where the same message's English source is, or None for a help paragraph
    (two languages' pages are not aligned paragraph by paragraph)."""
    return {
        "app": f"crates/teksilo_ui/locales/en-US/{block.file}",
        "framework": "crates/teksilo-widgets/locales/en-US.ftl",
        "widget-override": "crates/teksilo-widgets/locales/en-US.ftl",
        "export": "crates/skribisto_compiler/locales/en.ftl",
    }.get(block.catalogue)


# --- The GitHub API ----------------------------------------------------------


class Api:
    """The few REST calls the triage needs. A test substitutes a fake."""

    def __init__(self, token: str, repo: str) -> None:
        self.token = token
        self.repo = repo

    def _request(self, method: str, url: str, data: dict | None = None, raw: bool = False):
        headers = {
            "Authorization": f"Bearer {self.token}",
            "Accept": "application/vnd.github.raw+json" if raw else "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
            "User-Agent": "skribisto-translation-triage",
        }
        body = None
        if data is not None:
            body = json.dumps(data).encode()
            headers["Content-Type"] = "application/json"
        request = urllib.request.Request(url, data=body, headers=headers, method=method)
        try:
            with urllib.request.urlopen(request, timeout=30) as response:
                payload = response.read()
        except urllib.error.HTTPError as error:
            if error.code in (404, 422):
                return None
            raise
        if raw:
            return payload.decode("utf-8", errors="replace")
        return json.loads(payload) if payload else {}

    def _url(self, path: str, repo: str | None = None, **query: str) -> str:
        base = f"https://api.github.com/repos/{repo or self.repo}{path}"
        return base + ("?" + urllib.parse.urlencode(query) if query else "")

    def tag_exists(self, tag: str, repo: str | None = None) -> bool:
        return self._request("GET", self._url(f"/git/ref/tags/{urllib.parse.quote(tag)}", repo)) is not None

    def commit_sha(self, sha: str) -> str | None:
        found = self._request("GET", self._url(f"/commits/{sha}"))
        return found.get("sha") if found else None

    def compare_status(self, base: str, head: str) -> str | None:
        found = self._request("GET", self._url(f"/compare/{base}...{head}"))
        return found.get("status") if found else None

    def file_text(self, path: str, ref: str, repo: str | None = None) -> str | None:
        quoted = urllib.parse.quote(path)
        return self._request("GET", self._url(f"/contents/{quoted}", repo, ref=ref), raw=True)

    def labels(self) -> set[str]:
        names: set[str] = set()
        for page in range(1, 11):
            found = self._request("GET", self._url("/labels", per_page="100", page=str(page))) or []
            names.update(label["name"] for label in found)
            if len(found) < 100:
                break
        return names

    def create_label(self, name: str, colour: str) -> None:
        self._request("POST", self._url("/labels"), {"name": name, "color": colour})

    def add_labels(self, number: int, labels: list[str]) -> None:
        self._request("POST", self._url(f"/issues/{number}/labels"), {"labels": labels})

    def open_translation_issues(self) -> list[dict]:
        issues: list[dict] = []
        for page in range(1, 6):
            found = self._request(
                "GET",
                self._url("/issues", state="open", labels="translation", per_page="100", page=str(page)),
            ) or []
            issues.extend(i for i in found if "pull_request" not in i)
            if len(found) < 100:
                break
        return issues

    def comments(self, number: int) -> list[dict]:
        return self._request("GET", self._url(f"/issues/{number}/comments", per_page="100")) or []

    def post_comment(self, number: int, body: str) -> None:
        self._request("POST", self._url(f"/issues/{number}/comments"), {"body": body})

    def edit_comment(self, comment_id: int, body: str) -> None:
        self._request("PATCH", self._url(f"/issues/comments/{comment_id}"), {"body": body})


# --- Choosing what to quote ----------------------------------------------------


def choose_ref(api: Api, block: Block) -> str:
    """The ref the report's text is quoted at. See step 3 of the module docs."""
    match = VERSION_RE.match(block.version)
    if match and not match.group("hash"):
        tag = f"v{match.group('base')}"
        if api.tag_exists(tag):
            return tag
    commit = (match.group("hash") if match else None) or (
        block.version if BARE_HASH_RE.match(block.version) else None
    )
    if commit:
        sha = api.commit_sha(commit)
        if sha and api.compare_status(sha, DEV_BRANCH) in ("behind", "identical"):
            return sha
    tag = f"v{block.crate_version}"
    if api.tag_exists(tag):
        return tag
    return DEV_BRANCH


def framework_ref(api: Api, block: Block, skribisto_ref: str) -> str:
    """The teksilo tag a framework string is quoted at: the block's version, or the
    one `Cargo.lock` pins at the Skribisto ref, or teksilo's `dev`."""
    version = block.framework
    if not version:
        lock = api.file_text("Cargo.lock", skribisto_ref) or ""
        found = re.search(r'name = "teksilo-widgets"\nversion = "([^"]+)"', lock)
        version = found.group(1) if found else ""
    if version and api.tag_exists(f"v{version}", FRAMEWORK_REPO):
        return f"v{version}"
    return DEV_BRANCH


@dataclass
class Located:
    """A message (or paragraph) found in a file at a ref."""

    line: int
    text: str


def locate(block: Block, text: str) -> Located | None:
    """The reported message in `text`: by id for a Fluent file, by its reported
    line for a help page (a paragraph has no id of its own)."""
    if block.catalogue == "help":
        lines = text.splitlines()
        if not 1 <= block.line <= len(lines) or not lines[block.line - 1].strip():
            return None
        end = block.line - 1
        while end < len(lines) and lines[end].strip():
            end += 1
        return Located(block.line, "\n".join(lines[block.line - 1 : end]))
    entry = fluent_entries(text).get(block.message)
    return Located(entry.line, entry.value) if entry else None


def fluent_entries(text: str) -> dict[str, check_locales.Entry]:
    parsed = check_locales.ParsedLocale(locale="")
    check_locales.parse_ftl_text(text, Path("fetched.ftl"), "", parsed)
    return parsed.entries


def quote(text: str) -> str:
    """`text` in a fenced code block nothing inside it can close: the fence is one
    backtick longer than the longest run of backticks in the text."""
    longest = max((len(run) for run in re.findall(r"`+", text)), default=0)
    fence = "`" * max(3, longest + 1)
    return f"{fence}text\n{text}\n{fence}"


def code(text: str) -> str:
    """An inline code span for an id or a path (both validated, so no backtick)."""
    return f"`{text}`"


# --- The comment -------------------------------------------------------------


@dataclass
class Triage:
    block: Block
    ref: str
    repo: str
    path: str
    file_repo: str
    at_ref: Located | None = None
    on_dev: Located | None = None
    english: Located | None = None
    same_text: list[str] = field(default_factory=list)
    duplicates: list[int] = field(default_factory=list)


def build_comment(t: Triage) -> str:
    b = t.block
    parts = [COMMENT_MARKER, ""]
    if not b.found:
        parts.append(
            "This report names no message: the reporter did not find the text in the "
            "application's catalogues. Where they saw it is in the **Message** field."
        )
    else:
        if t.at_ref:
            permalink = f"https://github.com/{t.file_repo}/blob/{t.ref}/{t.path}#L{t.at_ref.line}"
            parts.append(f"**Where it is:** [{code(t.path)} line {t.at_ref.line}]({permalink})")
            parts.append("")
            parts.append(f"At {code(t.ref)}:")
            parts.append(quote(t.at_ref.text))
        else:
            parts.append(
                f"{code(b.message)} was not found in {code(t.path)} at {code(t.ref)}."
            )
        parts.append("")
        if t.on_dev is None:
            parts.append(f"On {code(DEV_BRANCH)}: removed or renamed.")
        elif t.at_ref is not None and t.on_dev.text == t.at_ref.text:
            parts.append(f"On {code(DEV_BRANCH)}: unchanged.")
        else:
            parts.append(f"On {code(DEV_BRANCH)}, already changed:")
            parts.append(quote(t.on_dev.text))
        if t.english:
            parts.append("")
            parts.append("English source:")
            parts.append(quote(t.english.text))
        if b.composed:
            parts.append("")
            parts.append(f"Composed with {code(b.composed)}: the fix may belong in either message.")
        if b.upstream:
            parts.append("")
            parts.append(
                f"A teksilo string: fix it in {code(FRAMEWORK_REPO)}, and if it cannot wait for "
                f"a teksilo release, override it in Skribisto's widget_locales meanwhile."
            )
        if t.same_text:
            shown = ", ".join(code(i) for i in t.same_text[:20])
            more = f" and {len(t.same_text) - 20} more" if len(t.same_text) > 20 else ""
            parts.append("")
            parts.append(f"Other messages in {code(b.locale)} with the same text: {shown}{more}.")
    if t.duplicates:
        parts.append("")
        refs = ", ".join(f"#{n}" for n in t.duplicates)
        parts.append(f"Possible duplicate of {refs} (same language and message).")
    return "\n".join(parts).rstrip() + "\n"


def find_duplicates(api: Api, block: Block, number: int, locales: set[str]) -> list[int]:
    """Open translation reports about the same message in the same language."""
    if not block.found:
        return []
    found: list[int] = []
    for issue in api.open_translation_issues():
        if issue.get("number") == number:
            continue
        try:
            other = parse_block(issue.get("body") or "", locales)
        except InvalidBlock:
            continue
        if other and other.locale == block.locale and other.message == block.message:
            found.append(int(issue["number"]))
    return sorted(found)


def triage(api: Api, event: dict, locales: set[str]) -> str:
    """Do the whole triage for one `issues` event; returns a one-line summary."""
    issue = event.get("issue") or {}
    number = int(issue.get("number", 0))
    if not number:
        return "no issue in the event"
    try:
        block = parse_block(issue.get("body") or "", locales)
    except InvalidBlock as error:
        ensure_labels(api, number, ["needs-triage"])
        return f"#{number}: invalid block ({error})"
    if block is None:
        return f"#{number}: no report block"

    labels = labels_for(block)
    duplicates = find_duplicates(api, block, number, locales)
    if duplicates:
        labels.append("possible duplicate")
    ensure_labels(api, number, labels)

    ref = choose_ref(api, block)
    t = Triage(block=block, ref=ref, repo=api.repo, path="", file_repo=api.repo, duplicates=duplicates)
    if block.found:
        t.path = repository_path(block)
        file_repo = FRAMEWORK_REPO if block.catalogue == "framework" else api.repo
        file_ref = framework_ref(api, block, ref) if block.catalogue == "framework" else ref
        t.file_repo, t.ref = file_repo, file_ref
        at_ref_text = api.file_text(t.path, file_ref, file_repo)
        dev_text = api.file_text(t.path, DEV_BRANCH, file_repo)
        if at_ref_text is not None:
            t.at_ref = locate(block, at_ref_text)
            if block.catalogue != "help" and t.at_ref is not None:
                value = t.at_ref.text
                t.same_text = sorted(
                    name
                    for name, entry in fluent_entries(at_ref_text).items()
                    if entry.value == value and name != block.message
                )
        if dev_text is not None:
            t.on_dev = locate(block, dev_text)
        english = english_path(block)
        if english:
            # A framework string and its override translate teksilo's English.
            in_teksilo = block.catalogue in ("framework", "widget-override")
            english_repo = FRAMEWORK_REPO if in_teksilo else api.repo
            english_ref = framework_ref(api, block, ref) if in_teksilo else ref
            english_text = api.file_text(english, english_ref, english_repo)
            entry = fluent_entries(english_text).get(block.message) if english_text else None
            t.english = Located(entry.line, entry.value) if entry else None
    upsert_comment(api, number, build_comment(t))
    return f"#{number}: {', '.join(labels)} at {t.ref}"


def ensure_labels(api: Api, number: int, labels: list[str]) -> None:
    existing = api.labels()
    for name in labels:
        if name not in existing:
            api.create_label(name, label_colour(name))
    api.add_labels(number, labels)


def upsert_comment(api: Api, number: int, body: str) -> None:
    for comment in api.comments(number):
        user = (comment.get("user") or {}).get("login", "")
        if user == "github-actions[bot]" and COMMENT_MARKER in (comment.get("body") or ""):
            api.edit_comment(int(comment["id"]), body)
            return
    api.post_comment(number, body)


def main() -> int:
    event_path = os.environ.get("GITHUB_EVENT_PATH")
    token = os.environ.get("GITHUB_TOKEN")
    repo = os.environ.get("GITHUB_REPOSITORY")
    if not (event_path and token and repo):
        print("GITHUB_EVENT_PATH, GITHUB_TOKEN and GITHUB_REPOSITORY are required", file=sys.stderr)
        return 2
    with open(event_path, encoding="utf-8") as handle:
        event = json.load(handle)
    print(triage(Api(token, repo), event, supported_locales()))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
