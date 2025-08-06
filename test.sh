#!/bin/bash
# Project-specific test script that uses nextest by default
# Usage: ./test.sh [nextest arguments]

set -e

echo "🚀 Running tests with nextest (blazing-fast, zero-allocation testing)"
echo "Arguments: $*"

# Run with nextest using default profile
exec cargo nextest run "$@"