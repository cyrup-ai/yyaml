#!/bin/bash
# Setup script to make nextest the default test runner
# Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.)

# Override cargo test with nextest
function cargo() {
    if [[ "$1" == "test" ]]; then
        shift
        command cargo nextest run "$@"
    else
        command cargo "$@"
    fi
}

# Export the function so it's available in subshells
export -f cargo

echo "✅ Nextest is now the default test runner for 'cargo test'"
echo "   Use 'cargo nextest run' directly for explicit nextest usage"
echo "   Use 'command cargo test' to access the original cargo test"