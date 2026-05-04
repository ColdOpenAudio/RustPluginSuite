# Git Command Sheet (Windows + Cross-Platform)

## Install / Verify

```powershell
# Install Git (Windows)
.\scripts\setup\install-git-windows.ps1

# Verify
 git --version
```

## Daily Workflow

```bash
git fetch origin
git checkout -b feature/my-change
git add .
git commit -m "Describe change"
git push -u origin feature/my-change
```

## Pre-Release Safety

```bash
git status --short
git log --oneline -n 10
git diff --stat origin/main...HEAD
```

## Hotfix Workflow

```bash
git checkout main
git pull --ff-only
git checkout -b hotfix/issue-id
# apply fix
git add .
git commit -m "Hotfix: issue-id"
git push -u origin hotfix/issue-id
```
