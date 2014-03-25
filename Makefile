RUSTCRATES           = highlight cli
cli_CRATE_DEPS       = highlight

DEBUG                ?= 1
RUSTDEBUG            = $(DEBUG)
include              rust-mk/rust.mk
