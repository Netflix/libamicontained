SRC=$(shell find . -name \*.rs | grep -v "^./target")
TEST?=$(patsubst test/%.bats,%,$(wildcard test/*.bats))

.PHONY: lib
lib:
	cargo build --release

# can run single tests via `make check TEST=cpusets`
.PHONY: check
check: example
	RUST_BACKTRACE=1 cargo test
	sudo bats -t $(patsubst %,test/%.bats,$(TEST))

%: %.c lib
	gcc -static -o $@ $< -I./include -L./target/release -lamicontained

.PHONY: lint
lint:
	rustfmt --check $(SRC)
	cargo clippy --all-targets --all-features -- -D warnings -D rust-2018-idioms -D rust-2021-compatibility -A clippy::upper-case-acronyms

.PHONY: fmt
fmt:
	rustfmt $(SRC)

.PHONY: clean
clean:
	-rm -rf target example
