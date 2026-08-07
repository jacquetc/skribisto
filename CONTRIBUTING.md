<!-- SPDX-License-Identifier: GPL-3.0-only -->
<!-- SPDX-FileCopyrightText: 2016 Cyril Jacquet -->

# Contributing to Skribisto

Thank you for your interest in contributing to Skribisto! This document provides guidelines and information for contributors.

## Code of Conduct

Please be respectful and constructive in all interactions. We aim to maintain a welcoming environment for everyone. Participation in this project is governed by the [Code of Conduct](CODE_OF_CONDUCT.md) (Contributor Covenant 2.1).

## Authorship and review

Skribisto is built under the following seven rules. They apply to your contributions too.

1. Direct human communication is written by humans. PR messages, issues, posts, replies: no AI drafting, no AI polish. Common decency.

2. Documentation may be drafted by AI; every line is reviewed by a human. API examples must compile against the current API. Claims are checked, not skimmed.

3. Code, including tests, may be written by AI; every line is reviewed by a human. "Reviewed" means the reviewer understands the change well enough to defend it without the AI in the room. Vibe coding is forbidden. Plausible-looking code is not reviewed code.

4. Architecture and public API are human. AI implements within them; it does not design them. The load-bearing surface is specified by a human: the writing-model constraint matrix (`skribisto_model`), the `.skrib` on-disk format, the Qleany entity/use-case model, the view-model architecture — anything the app's correctness or a writer's files depend on.

5. Authors and reviewers, both human, are the voluntary bottleneck. Final responsibility rests with them, not the AI. They may use any tool to help, AI included; what is missed lands on them regardless. They take their time; high-speed AI output is not a reason for high-speed work.

6. The human who signs the work owns it, AI or not. Provenance is not disclosed in commits or PR text.

7. No AI has ever been condemned by judges. Only humans and companies have. Stay sharp.

## How to Contribute

### Reporting Issues

- Check existing issues before creating a new one
- Provide a clear description of the problem
- Include steps to reproduce, expected behavior, and actual behavior
- Mention your environment (OS, Rust version, etc.)

### Suggesting Features

- Open an issue describing the feature and its use case
- Explain why this would be valuable for Skribisto users
- Be open to discussion about alternative approaches

### Submitting Code

1. Fork the repository
2. Create a feature branch from `dev` (the active development branch)
3. Make your changes
4. Ensure your code follows the project's conventions (see `.claude/CLAUDE.md` for the working guide)
5. Build **both** feature sets green (`cargo build` and `cargo build -p teksilo_ui --features mocks`) and run `cargo test`
6. Add both `en-US` and `fr-FR` strings for any user-visible text
7. Submit a pull request

## Contributor License Agreement

Skribisto is free software, published under the **GNU General Public License v3.0** (see [LICENSE](LICENSE)). It is also developed under an **open-core model**: alongside the free, GPL-licensed community edition, the maintainer may offer a separate commercial edition ("Skribisto Pro") with additional features under proprietary terms. The project is currently held by its author, Cyril Jacquet; stewardship is intended to pass to **FernTech** at a later date (see the "Maintainer" definition in [CLA.md](CLA.md)).

For that to be possible, the maintainer must hold the rights to include your contribution in **both** editions. A plain Developer Certificate of Origin cannot grant those rights, so — unlike the sibling FernTech libraries — Skribisto uses a **Contributor License Agreement (CLA)** instead.

The CLA (see [CLA.md](CLA.md)) does three things, in plain terms:

- **You keep the copyright to your work.** The CLA is a license grant, not an assignment. You can still use your own contribution however you like, elsewhere.
- **You let the maintainer ship your contribution in both editions** — the GPL community edition and any commercial edition — by granting a broad, sublicensable copyright and patent license.
- **In return, the project guarantees your contribution stays free.** The maintainer commits to always making your contribution available in the community edition under the GPL (v3 or a later/OSI-approved license no more restrictive). Your work can never be pulled *out* of open source.

The CLA also includes the same origin certification a DCO would (that the work is yours to submit), so it fully supersedes a DCO — there is no separate `DCO.md`.

### How to agree

You agree to the CLA by **signing off your commits**. Per this document and [CLA.md](CLA.md), adding a `Signed-off-by` line certifies that you have read the CLA and accept its terms for that contribution:

```bash
git commit -s -m "Your commit message"
```

This appends a `Signed-off-by: Your Name <your.email@example.com>` line, using the identity from your Git config.

### Setting up sign-off

Configure your identity for this repository:

```bash
git config user.name "Your Name"
git config user.email "your.email@example.com"
```

Then use `git commit -s` for each commit, or create a Git alias:

```bash
git config --global alias.cs "commit -s"
```

### What if I forgot to sign off?

Amend your last commit:

```bash
git commit --amend -s
```

For multiple commits, rebase with sign-off (replace `N` with the number of commits):

```bash
git rebase --signoff HEAD~N
```

### For substantial or organization-backed contributions

The sign-off method above is the lightweight default and is sufficient for ordinary contributions. As the project grows, the maintainer may add an automated CLA assistant to record per-contributor acceptance, and may ask for an explicit signature for large contributions or those made on behalf of an employer. If any of your contribution is **not your original creation** (third-party code, snippets from elsewhere), say so in the pull request and identify its source and license — see section 6 of the CLA.

## License

By contributing to Skribisto, you agree that your contribution is licensed under the [GNU General Public License v3.0](LICENSE) as part of the community edition, **and** under the additional terms of the [Contributor License Agreement](CLA.md).

## Questions?

If you have questions about contributing or the CLA, open an issue for discussion, or contact the maintainer (see the *To contact me* section of the [README](README.md)).
