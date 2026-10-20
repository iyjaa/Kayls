#!/usr/bin/env bash
#
# Update ISSUES.md from GitHub Issues.
#
# Requires:
#   - gh CLI (GitHub CLI)
#   - GH_TOKEN environment variable (provided by GitHub Actions)
#
# Usage:
#   bash .github/scripts/update-issues.sh

set -euo pipefail

REPO="${GITHUB_REPOSITORY:-iyjaa/Kayls}"
OUTPUT="ISSUES.md"
DATE=$(date -u +"%Y-%m-%d")

echo "Updating $OUTPUT for repo: $REPO"

# -----------------------------------------------------------------------------
# Fetch issues
# -----------------------------------------------------------------------------

OPEN_ISSUES=$(gh issue list \
    --repo "$REPO" \
    --state open \
    --limit 100 \
    --json number,title,labels,assignees,createdAt \
    --jq '[.[] | select(.assignees | length == 0)]')

IN_PROGRESS=$(gh issue list \
    --repo "$REPO" \
    --state open \
    --limit 100 \
    --json number,title,labels,assignees,createdAt \
    --jq '[.[] | select(.assignees | length > 0)]')

CLOSED_ISSUES=$(gh issue list \
    --repo "$REPO" \
    --state closed \
    --limit 100 \
    --json number,title,labels,closedAt,url,assignees \
    --jq 'sort_by(.closedAt) | reverse')

# -----------------------------------------------------------------------------
# Helper functions
# -----------------------------------------------------------------------------

# Get issue type from labels
get_type() {
    local labels="$1"
    if echo "$labels" | grep -qi "bug"; then
        echo "Bug"
    elif echo "$labels" | grep -qi "feature"; then
        echo "Feature"
    elif echo "$labels" | grep -qi "docs"; then
        echo "Docs"
    else
        echo "Other"
    fi
}

# -----------------------------------------------------------------------------
# Generate sections
# -----------------------------------------------------------------------------

generate_open_table() {
    local data="$1"
    local count
    count=$(echo "$data" | jq 'length')

    if [ "$count" -eq 0 ]; then
        echo "| — | No open issues | — | — |"
        return
    fi

    echo "$data" | jq -r '.[] |
        "| #\(.number) | \(.title) | \(if (.labels | map(.name) | any(. == "bug")) then "Bug" elif (.labels | map(.name) | any(. == "feature")) then "Feature" elif (.labels | map(.name) | any(. == "docs")) then "Docs" else "Other" end) | \(.createdAt[0:10]) |"
    '
}

generate_in_progress_table() {
    local data="$1"
    local count
    count=$(echo "$data" | jq 'length')

    if [ "$count" -eq 0 ]; then
        echo "| — | No issues in progress | — | — |"
        return
    fi

    echo "$data" | jq -r '.[] |
        "| #\(.number) | \(.title) | \(if (.labels | map(.name) | any(. == "bug")) then "Bug" elif (.labels | map(.name) | any(. == "feature")) then "Feature" elif (.labels | map(.name) | any(. == "docs")) then "Docs" else "Other" end) | \(.assignees | map("@" + .login) | join(", ")) |"
    '
}

generate_fixed_table() {
    local data="$1"
    local count
    count=$(echo "$data" | jq 'length')

    if [ "$count" -eq 0 ]; then
        echo "| — | No issues fixed yet | — | — | — |"
        return
    fi

    echo "$data" | jq -r '.[] |
        "| #\(.number) | \(.title) | \(if (.labels | map(.name) | any(. == "bug")) then "Bug" elif (.labels | map(.name) | any(. == "feature")) then "Feature" elif (.labels | map(.name) | any(. == "docs")) then "Docs" else "Other" end) | \(.assignees // [] | map("@" + .login) | join(", ")) | \(.closedAt[0:10] // "—") |"
    '
}

# -----------------------------------------------------------------------------
# Generate ISSUES.md
# -----------------------------------------------------------------------------

{
    cat <<EOF
# Kayls Issues

> Summary of issues from [GitHub Issues](https://github.com/$REPO/issues).
> For the full list, see GitHub.

**Last updated**: $DATE

---

## Open Issues

Issues that have been reported but not yet fixed.

| ID | Title | Type | Reported |
|----|-------|------|----------|
EOF

    generate_open_table "$OPEN_ISSUES"

    cat <<EOF

---

## In Progress

Issues currently being worked on. Marked with \`[WIP]\` in the title.

| ID | Title | Type | Assignee |
|----|-------|------|----------|
EOF

    generate_in_progress_table "$IN_PROGRESS"

    cat <<EOF

---

## Fixed Issues

Issues that have been fixed and merged. Contributors are credited below.

| ID | Title | Type | Fixed by | Version |
|----|-------|------|----------|---------|
EOF

    generate_fixed_table "$CLOSED_ISSUES"

    cat <<EOF

---

## Criteria for "Fixed"

An issue is considered fixed when ALL of these are met:

1. **Build passes** — \`cargo build --workspace\` succeeds without errors
2. **Works correctly** — the fix/feature works as described
3. **Evidence provided** — screenshot, video, or log demonstrating the fix
4. **Reviewed** — maintainer reviews the PR
5. **Merged** — PR is merged to \`main\`

Only issues that meet all criteria appear in the "Fixed Issues" list.

---

## Contributors

Thanks to everyone who has contributed to Kayls:

| Contributor | Issues |
|-------------|--------|
| @iyjaa | Project maintainer |

---

## How to Report an Issue

1. Check if the issue already exists in [GitHub Issues](https://github.com/$REPO/issues)
2. If not, open a new issue with:
   - **Title** — clear and specific
   - **Type** — Bug, Feature, or Docs
   - **Description** — what happened, what you expected
   - **Steps to reproduce** (for bugs)
   - **Environment** — OS, terminal, Kayls version
3. Wait for maintainer triage

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution workflow.

---

## Labels Used

| Label | Meaning |
|-------|---------|
| \`bug\` | Something is broken |
| \`feature\` | New feature request |
| \`docs\` | Documentation improvement |
| \`good first issue\` | Suitable for first-time contributors |
| \`help wanted\` | Maintainer needs help |
| \`wip\` | Work in progress |
| \`wontfix\` | Will not be fixed |

---

## Notes

- This file is a **summary**, not the source of truth
- The source of truth is [GitHub Issues](https://github.com/$REPO/issues)
- This file is updated automatically via GitHub Actions
- Workflow: \`.github/workflows/update-issues.yml\`
EOF
} > "$OUTPUT"

echo "Done. $OUTPUT updated."