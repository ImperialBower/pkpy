.PHONY: setup build test demo the-hand fmt clippy clean ayce default help

VENV := .venv
PYTHON := $(VENV)/bin/python
PIP := $(VENV)/bin/pip
MATURIN := $(VENV)/bin/maturin
PYTEST := $(VENV)/bin/pytest

# Default target
default: ayce

# Display help information
help:
	@echo "Available targets:"
	@echo "  make (default)  - Run ayce"
	@echo "  make setup      - Create venv and install dependencies"
	@echo "  make build      - Compile the Rust extension into the venv"
	@echo "  make test       - Run pytest"
	@echo "  make demo       - Run demo.py"
	@echo "  make the-hand   - Run examples/the_hand.py"
	@echo "  make fmt        - Format Rust code"
	@echo "  make clippy     - Run clippy linter"
	@echo "  make clean      - Remove build artifacts and venv"
	@echo "  make ayce       - Run fmt, build, test, and demo"
	@echo "  make help       - Display this help message"
	@echo ""

# File target: only creates the venv and installs packages when the sentinel doesn't exist
$(VENV)/bin/activate:
	python3 -m venv $(VENV)
	$(PIP) install --upgrade pip
	$(PIP) install maturin pytest

# Convenience alias
setup: $(VENV)/bin/activate

# Compile the Rust extension and install it into the venv
build: $(VENV)/bin/activate
	$(MATURIN) develop

# Run tests
test: build
	$(PYTEST)

# Run the demo
demo: build
	$(PYTHON) demo.py

# Run the The Hand example
the-hand: build
	$(PYTHON) examples/the_hand.py

# Format Rust code
fmt:
	cargo fmt

# Run clippy linter
clippy:
	cargo clippy -- -W clippy::pedantic

# Remove build artifacts and venv
clean:
	cargo clean
	rm -rf $(VENV)

# All You Can Eat - Run all checks
ayce: fmt build test demo
