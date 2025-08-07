## Configuration
## =============

.DEFAULT_GOAL := help
.DELETE_ON_ERROR: ;
.SECONDARY:
.SUFFIXES: ;
.PHONY: .FORCE

## Verbs
## =====

help:
	@mkhelp $(firstword $(MAKEFILE_LIST))

## Checks
## ------

check: check_build check_docs check_format check_generated_files check_lint check_tests

## _
check_build:
	cargo build \
		--locked \
		--workspace
.PHONY: check_build

## _
check_docs:
	RUSTDOCFLAGS="-Dwarnings" cargo doc \
		--document-private-items \
		--locked \
		--no-deps \
		--workspace
.PHONY: check_docs

## _
check_format:
	find . -type f -name '*.rs' \
	| xargs rustfmt --check \
		--config imports_granularity=Crate \
		--config group_imports=StdExternalCrate \
		--edition 2021
	cargo fmt --check
.PHONY: check_format

## _
check_generated_files: Cargo.lock
	git update-index -q --refresh
	git --no-pager diff --exit-code HEAD -- $^
.PHONY: check_generated_files

## _
check_lint:
	cargo clippy \
		--all-targets \
		--locked \
		--no-deps \
		--workspace \
		-- \
		-Dwarnings
.PHONY: check_lint

## _
check_tests:
	cargo test \
		--all-targets \
		--locked \
		--workspace
.PHONY: check_tests

## Fixes
## -----

## _
fix_format:
	find . -type f -name '*.rs' \
	| xargs rustfmt \
		--config imports_granularity=Crate \
		--config group_imports=StdExternalCrate \
		--edition 2021
	cargo fmt
.PHONY: fix_format

## _
fix_lint:
	cargo clippy --fix
.PHONY: fix_lint


## Nouns
## =====

Cargo.lock: $(wildcard crates/*/Cargo.toml)
	cargo metadata --format-version=1 > /dev/null
