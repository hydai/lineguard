#!/bin/bash
# Example pre-commit hook for LineGuard
# Save this file as .git/hooks/pre-commit and make it executable

set -e

echo "Running LineGuard checks..."

# Check only staged files
FILES=$(git diff --cached --name-only --diff-filter=ACM)

if [ -z "$FILES" ]; then
    echo "No files to check"
    exit 0
fi

# Run LineGuard on staged files
echo "$FILES" | lineguard --stdin --quiet

# Optional: Auto-fix issues
# echo "$FILES" | lineguard --stdin --fix
# git add $FILES

echo "LineGuard checks passed!"