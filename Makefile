.PHONY: setup build test demo the-hand calc gto fmt clippy clean ayce default help version release

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
	@echo "  make calc       - Run examples/calc.py (THE HAND)"
	@echo "  make gto        - Run examples/gto.py (KK vs range)"
	@echo "  make fmt        - Format Rust code"
	@echo "  make clippy     - Run clippy linter"
	@echo "  make clean      - Remove build artifacts and venv"
	@echo "  make ayce       - Run fmt, build, test, and demo"
	@echo "  make version    - Print the crate version from Cargo.toml"
	@echo "  make release    - Tag HEAD as v<Cargo.toml version> and push (drives publish.yml)"
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

# Run the calc example (THE HAND)
calc: build
	$(PYTHON) examples/calc.py -d "6♠ 6♥ 5♦ 5♣" -b "9♣ 6♦ 5♥ 5♠"

# Run the gto example (KK vs range)
gto: build
	$(PYTHON) examples/gto.py -p "K♠ K♥" -v "66+,AJs+,KQs,AJo+,KQo"

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

# Print the crate version from Cargo.toml
version:
	@awk -F'"' '/^version/{print $$2; exit}' Cargo.toml

# Cut a release: verify, annotate-tag from Cargo.toml's version, push to origin.
# The pushed tag is what fires .github/workflows/publish.yml.
release:
	@v=$$(awk -F'"' '/^version/{print $$2; exit}' Cargo.toml); \
	if [ -z "$$v" ]; then echo "could not parse version from Cargo.toml"; exit 1; fi; \
	if ! git diff --quiet HEAD; then echo "working tree has uncommitted changes; commit or stash first"; exit 1; fi; \
	if git rev-parse "v$$v" >/dev/null 2>&1; then echo "tag v$$v already exists"; exit 1; fi; \
	if [ -f CHANGELOG.md ] && ! grep -q "^## \[$$v\]" CHANGELOG.md; then echo "CHANGELOG.md has no entry for $$v"; exit 1; fi; \
	echo "Tagging v$$v on $$(git rev-parse --short HEAD) and pushing to origin..."; \
	git tag -a "v$$v" -m "Release v$$v" && git push origin "v$$v"
