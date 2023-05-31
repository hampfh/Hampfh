# Makefile

all: check_rust prepare_env build run_migration

check_rust:
	@if ! command -v cargo &> /dev/null; then \
		echo "Rust is not installed. Please install it from https://www.rust-lang.org/tools/install"; \
		exit 1; \
	fi

prepare_env:
	@if [ -f .env.example ]; then \
		mv .env.example .env; \
	fi

build:
	@cargo build --release

run_migration:
	@diesel migration run
