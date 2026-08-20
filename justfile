# tkr CLI - Rust Ticket Management System
# Standard justfile following ADR-20260131001

_log := '
_jv_has() {
  local cat="$1"
  local v="${JUST_LOG:-0}"
  case "$v" in
    1|all) return 0 ;;
    0|"") return 1 ;;
  esac
  v="${v//startend/start,end}"
  echo ",$v," | grep -q ",$cat,"
}
log_info()   { _jv_has info   && echo "$*" || true; }
log_start()  { _jv_has start  && echo "▶ $*" || true; }
log_end()    { _jv_has end    && echo "✔ $*" || true; }
log_status() { _jv_has status && echo "$*" || true; }
log_warn()   { echo "⚠️  $*" >&2; }
log_error()  { echo "❌ $*" >&2; }
log_startend() {
  local msg="$1"; shift
  local rc
  _jv_has start && echo "▶ $msg" || true
  rc=0; "$@" || rc=$?
  _jv_has end && echo "✔ $msg complete" || true
  return $rc
}
'

# Devbox auto-detection: run impl target directly if in devbox,
# re-exec via devbox run if not, or fail with doctor diagnostic.
_devbox target *args:
    #!/usr/bin/env bash
    {{_log}}
    if [ "${DEVBOX_SHELL_ENABLED:-0}" = "1" ]; then
        exec just "{{target}}" {{args}}
    elif command -v devbox >/dev/null 2>&1; then
        exec devbox run -- just "{{target}}" {{args}}
    else
        log_error "devbox not found in PATH."
        log_warn "Running doctor to diagnose environment issues..."
        just doctor 2>/dev/null || true
        exit 1
    fi

# Normal targets - Developer interface (REQUIRED)
clean:
    @just _devbox clean_impl

dev:
    @just _devbox dev_impl

build:
    @just _devbox build_impl

test:
    @just _devbox test_impl

lint:
    @just _devbox lint_impl

typecheck:
    @just _devbox typecheck_impl

release:
    @just _devbox release_impl

# Bootstrap recipes (REQUIRED)
bootstrap:
    # Ensure devbox is available and environment is ready
    @just _devbox bootstrap_impl

# Health and diagnostics (REQUIRED)
doctor:
    @just _devbox doctor_impl

# Quality checks (OPTIONAL but RECOMMENDED)
quality:
    @just lint
    @just test
    @just typecheck

# Language-specific commands for Rust CLI
# Development setup (OPTIONAL)
setup:
    #!/usr/bin/env bash
    {{_log}}
    log_end "Rust CLI development environment ready!"

# Docker commands (maintained from original Makefile)
docker-build:
    # Build docker image
    docker build -t tkr:latest .

docker-run:
    # Run via docker
    docker run --rm -it tkr:latest

# Profile management (from original Makefile)
profile:
    #!/usr/bin/env bash
    {{_log}}
    # Install toolchain into user profile (optional)
    log_warn "Profile management handled by devbox - use 'devbox profile install' if needed"

# Help target (maintained from original Makefile)
help:
    # Show available commands
    echo "🦀 tkr CLI - Rust Ticket Management System"
    echo ""
    echo "Standard commands:"
    echo "  just bootstrap    - Initialize the development environment"
    echo "  just build        - Build the project"
    echo "  just test         - Run tests"
    echo "  just lint         - Run linting"
    echo "  just typecheck    - Run type checking"
    echo "  just dev           - Run in development mode"
    echo "  just clean         - Clean build artifacts"
    echo "  just doctor        - Check environment health"
    echo "  just quality       - Run all quality checks"
    echo ""
    echo "Rust-specific commands:"
    echo "  just debug         - Build in debug mode"
    echo "  just install       - Install binary locally"
    echo ""
    echo "Docker commands:"
    echo "  just docker-build  - Build Docker image"
    echo "  just docker-run    - Run via Docker"
    echo ""
    echo "Internal commands (for devbox scripts):"
    echo "  just *_impl        - Internal implementations"

# =============================================================================
# Implementation targets (private)
# =============================================================================

[private]
bootstrap_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    # Internal bootstrap logic called by devbox init_hook
    # Language-specific dependency installation
    just setup
    log_end "Project bootstrap complete"

[private]
doctor_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "🔍 Checking tkr CLI development environment..."
    if ! cargo --version >/dev/null 2>&1; then
        echo "❌ Error: cargo not found" >&2
        echo "💡 Suggestion: Ensure Rust toolchain is installed" >&2
        exit 1
    fi
    if ! just --version >/dev/null 2>&1; then
        echo "❌ Error: just not found" >&2
        echo "💡 Suggestion: Ensure just is installed" >&2
        exit 1
    fi
    if [ ! -f Cargo.toml ]; then
        echo "❌ Error: Cargo.toml not found (expected in project root)" >&2
        exit 1
    fi
    echo "✅ OK: Rust toolchain + just + Cargo.toml present"
    if command -v direnv >/dev/null 2>&1; then
        echo "✅ OK: direnv present"
        echo "💡 Next: direnv allow"
    else
        echo "⚠️  Warning: direnv not found"
        echo "💡 Suggestion: install direnv (https://direnv.net/)"
        echo "💡 Then run: direnv allow"
    fi
    echo "💡 Suggestion: just bootstrap"
    echo "🚀 Ready to develop tkr CLI!"

[private]
clean_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    # Clean build artifacts
    cargo clean
    log_end "Build artifacts removed"

[private]
build_impl:
    # Build the project in release mode
    cargo build --release

[private]
release_impl:
    #!/usr/bin/env bash
    set -euo pipefail
    {{_log}}
    log_start "Starting release pipeline for tkr CLI"
    just lint_impl
    just test_impl
    just typecheck_impl
    just build_impl
    log_end "Release complete! Binary available at target/release/tkr"

[private]
debug_impl:
    # Build the project in debug mode
    cargo build

[private]
install_impl:
    # Install the binary locally
    cargo install --path .

[private]
lint_impl:
    # Lint the code using clippy
    cargo clippy -- -D warnings

[private]
test_impl:
    # Run tests
    cargo test

[private]
typecheck_impl:
    # Run type checking (cargo check)
    cargo check

[private]
dev_impl:
    # Run the application in development mode
    cargo run

[private]
run_impl:
    # Run the application with arguments
    cargo run
