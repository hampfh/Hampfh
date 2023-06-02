# Makefile

all: check_rust prepare_env run_migration build

check_rust:
	@if ! command -v cargo &> /dev/null; then \
		echo "Rust is not installed. Please install it from https://www.rust-lang.org/tools/install"; \
		exit 1; \
	fi
	@rustup update

prepare_env:
	@if [ -f .env.example ]; then \
		mv .env.example .env; \
	fi

build:
	@cargo build --release

run_migration:
	@brew install sqlite
	@cargo install diesel_cli --no-default-features --features sqlite
	@diesel migration run

info:
	@echo "\033[32mSuccessfully set up local environment for bot creation!"
	@echo "Execute your first match here:"
	@echo "\033[36mcargo run --release match ../scripts/trivial_dodger.lua ../scripts/bfs.lua\033[0m"
	@echo "\033[32mYour output should now end up in the file match.temp.gif and match.temp.md\033[0m"