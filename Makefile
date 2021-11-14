all: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test

clean:
	@cargo clean
	@rm -f z.*
