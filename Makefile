RUSTCRATES           = highlight

DEBUG                ?= 1
RUSTDEBUG            = $(DEBUG)
include              rust-mk/rust.mk
