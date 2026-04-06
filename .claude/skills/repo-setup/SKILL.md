---
name: repo-setup
description: Initialize a repo created from the KeplerOps template — fill in TODOs, create GC project, create dev branch
disable-model-invocation: true
---

# Repository Setup

This skill walks through first-time setup for a repo created from the KeplerOps template. Run it once after creating the repo.

## Step 1: Gather Project Info

Ask the user for:
1. **Project name** — human-readable (e.g. "Capture")
2. **Project description** — one-sentence summary
3. **GC project identifier** — lowercase with hyphens (e.g. "capture")
4. **GitHub owner/repo** — e.g. "KeplerOps/capture"

## Step 2: Create Ground Control Project

Use `gc_create_project` to create the project:
- identifier: from step 1
- name: from step 1
- description: from step 1

## Step 3: Fill In TODO Placeholders

Replace all TODO placeholders across the repo:

1. **CLAUDE.md** — set the project description and GC project identifier
2. **AGENTS.md** — no changes needed (generic)
3. **README.md** — set project name and description
4. **.mcp.json** — set `GH_REPO` to the owner/repo value
5. **CHANGELOG.md** — add initial `[0.1.0]` entry with today's date

Search for any remaining `<!-- TODO` markers and resolve them.

## Step 4: Create dev Branch

```
git checkout -b dev
git push -u origin dev
git checkout main
```

## Step 5: Configure Branch Protection

If the user wants branch protection (ask them):

```
gh api repos/{owner}/{repo}/branches/main/protection \
  --method PUT \
  -f "required_pull_request_reviews[required_approving_review_count]=0" \
  -F "required_pull_request_reviews[dismiss_stale_reviews]=true" \
  -F "enforce_admins=false" \
  -F "required_status_checks=null" \
  -F "restrictions=null"
```

## Step 6: Commit & Push

1. Stage all changed files.
2. Commit: "Initialize project from KeplerOps template"
3. Push to main.

## Step 7: Report

Confirm:
- Ground Control project created with identifier
- All TODO placeholders resolved
- dev branch created and pushed
- Branch protection configured (if requested)
- Repo ready for development
