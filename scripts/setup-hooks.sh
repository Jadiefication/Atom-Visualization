#!/bin/bash
set -e

# Make the pre-commit hook executable
chmod +x .githooks/pre-commit

# Configure git to use the hooks in .githooks
git config core.hooksPath .githooks

echo "Git hooks configured successfully!"
