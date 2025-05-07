#!/bin/bash

# This script is designed to check if the current package version exists on crates.io
# If it doesn't, it will trigger the workflow to publish it

set -e

# Get the current version from Cargo.toml
CURRENT_VERSION=$(grep '^version =' Cargo.toml | head -n 1 | cut -d '"' -f 2)

echo "Current version in Cargo.toml: $CURRENT_VERSION"

# Check if this version exists on crates.io
RESPONSE=$(curl -s https://crates.io/api/v1/crates/node-size-analyzer/$CURRENT_VERSION)

if echo "$RESPONSE" | grep -q "\"version\":\"$CURRENT_VERSION\""; then
  echo "Version $CURRENT_VERSION already exists on crates.io. No action needed."
  exit 0
else
  echo "Version $CURRENT_VERSION does not exist on crates.io."
  
  # Check if gh CLI is available
  if command -v gh &> /dev/null; then
    echo "Triggering GitHub workflow to publish the new version..."
    gh workflow run force-publish.yml -f version=$CURRENT_VERSION
    echo "Workflow triggered. Check GitHub Actions for progress."
  else
    echo "GitHub CLI not available. Please manually trigger the 'Force Publish to Crates.io' workflow with version: $CURRENT_VERSION"
  fi
fi