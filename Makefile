##
# Repo
#
# @file
# @version 0.1

default:
	cargo build
	cargo install --path .
	@echo "Done."

build:
	cargo build --release

# end
