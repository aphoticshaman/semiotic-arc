# Git Instructions for Initial Push
## Step-by-step for Ryan

---

## Prerequisites

1. GitHub account (you have this)
2. Git installed (you have this via Git Bash)
3. GitHub CLI or personal access token (for authentication)

---

## Step 1: Create the GitHub Repository

Go to: https://github.com/new

Fill in:
- **Repository name**: `semiotic-arc`
- **Description**: Semiotic reasoning for abstract visual tasks
- **Public** (asymmetric warfare = open source)
- **NO** README (we have one)
- **NO** .gitignore (we'll add one)
- **NO** license (we have MIT)

Click "Create repository"

---

## Step 2: Initialize Local Git Repo

Open Git Bash in the semiotic-arc folder:

```bash
cd "/c/Users/ryanj/OneDrive/7. Ryan/Ryan's Stuff/Desktop/Crystalline Labs/Research/semiotic-arc"

# Initialize git
git init

# Add all files
git add .

# Check what's staged
git status

# Make the first commit
git commit -m "Initial release: Semiotic reasoning for ARC-AGI

Core contribution:
- Dual-channel model: evolved patterns + conditioned symbols
- Reality model inference: puzzles as compressed simulations
- 13-layer cognitive architecture
- Annotation schema for crowdsourced dataset

This is asymmetric warfare. The insight is free."
```

---

## Step 3: Connect to GitHub

```bash
# Add the remote (replace YOUR_USERNAME if not using org)
git remote add origin https://github.com/crystalline-labs/semiotic-arc.git

# OR if personal account:
# git remote add origin https://github.com/YOUR_USERNAME/semiotic-arc.git

# Verify remote
git remote -v
```

---

## Step 4: Push

```bash
# Push to main branch
git branch -M main
git push -u origin main
```

If it asks for credentials:
- Username: your GitHub username
- Password: your GitHub personal access token (NOT your password)

To create a token: https://github.com/settings/tokens
- Generate new token (classic)
- Select scopes: `repo` (full control)
- Copy and save it somewhere safe

---

## Step 5: Verify

Go to: https://github.com/crystalline-labs/semiotic-arc

You should see:
- README.md rendered
- All files present
- Green checkmark

---

## Optional: Add .gitignore

Create `.gitignore` in the repo root:

```
# Rust
target/
Cargo.lock
*.rs.bk

# Python
__pycache__/
*.py[cod]
.env
venv/

# IDE
.idea/
.vscode/
*.swp

# OS
.DS_Store
Thumbs.db

# Build artifacts
*.wasm
pkg/
```

Then:
```bash
git add .gitignore
git commit -m "Add .gitignore"
git push
```

---

## Future Updates (Dev Workflow)

1. Make changes in `agi3/` (dev repo)
2. Clean and copy to `semiotic-arc/` (publish repo)
3. In semiotic-arc:
   ```bash
   git add .
   git commit -m "Description of changes"
   git push
   ```

---

## Troubleshooting

**"Permission denied"**
- Make sure you're using a personal access token, not your password
- Check that the token has `repo` scope

**"Remote already exists"**
```bash
git remote remove origin
git remote add origin https://github.com/...
```

**"Branch 'main' not found"**
```bash
git branch -M main
```

**"Updates were rejected"**
- Someone else pushed first (shouldn't happen on new repo)
- Or you need to pull first: `git pull origin main --rebase`

---

## The Twitter Thread

After push succeeds, go post the thread from ACADEMIC_REPO_PLAN.md

Shoot. Move. Communicate.

*We don't win alone.*
