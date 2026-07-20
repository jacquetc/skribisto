# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""One rule, in one place: a live-app probe never opens a checked-in fixture.

Driving the real app means Ctrl+S, autosave, and format migration all happen for
real. Any of them rewrites whatever file was passed on the command line. Three
separate incidents came from forgetting that:

  * a probe saved into `resources/test/skribisto_test_project.skrib`, so the next
    run's toggle silently undid the previous run's;
  * a run migrated the same fixture from format v2 to v3, which broke a test that
    had pinned the version — the test looked wrong, the probe was;
  * the same file was later reverted on the assumption a *test* had rewritten it.
    No test does. Bisecting the suite proved it, after the fact.

None of those were caught by the probe that caused them, because a probe that
mutates its own fixture still passes — it just stops testing what it claims to,
and takes the repo's copy with it.

So: `working_copy()` hands back a throwaway in the scratchpad, and probes open
*that*. Cheap enough (a few MB) that there is no reason to skip it.
"""

import os
import shutil
import tempfile

#: Session scratchpad when one is set, else the system temp dir. Never inside the repo.
SCRATCH = os.environ.get(
    "SKRIBISTO_AUTOMATION_SCRATCH",
    "/tmp/claude-1000/-home-cyril-Devel-skribisto--claude-worktrees-tags/"
    "b56517fd-f8b4-4a4f-8219-e7d54b9af7e3/scratchpad",
)


def working_copy(src, label="fixture"):
    """Copy `src` into the scratchpad and return the copy's path.

    Works for both `.skrib` shapes: a zip file is copied, an exploded folder is
    copied whole. The name carries `label` and the pid so parallel probes — and
    successive runs of one probe — never share a file.
    """
    base = SCRATCH if os.path.isdir(SCRATCH) else tempfile.gettempdir()
    os.makedirs(base, exist_ok=True)
    src = os.path.abspath(src)
    if not os.path.exists(src):
        raise FileNotFoundError(f"no fixture at {src}")

    stem = os.path.basename(src.rstrip("/")) or "project"
    dst = os.path.join(base, f"probe-{label}-{os.getpid()}-{stem}")
    # A previous run at the same pid (or a crash) can leave one behind.
    if os.path.isdir(dst):
        shutil.rmtree(dst)
    elif os.path.exists(dst):
        os.remove(dst)

    if os.path.isdir(src):
        shutil.copytree(src, dst)
    else:
        shutil.copy2(src, dst)

    # Belt and braces: if this ever returns a path inside the repo, the probe is
    # about to do the exact thing this module exists to prevent.
    repo = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    assert not os.path.abspath(dst).startswith(repo + os.sep), (
        f"working copy landed inside the repo ({dst}) — refusing to hand it back"
    )
    return dst
