MAX_ITER=20000

.PHONY: all entire filtered

all: filtered
entire: out.svg
filtered: out-filtered.svg

target/release/renderer: src/main.rs Cargo.toml Cargo.lock
	cargo build --release

out.dot: target/release/renderer
	./target/release/renderer > out.dot

out-filtered.dot: out.dot
	gvpr -c 'N{$.degree == 0}' out.dot > out-filtered.dot

out.svg: out.dot
	time fdp -Gmaxiter=$(MAXITER) -GK=0.1 -Tsvg -o out.svg out.dot

out-filtered.svg: out-filtered.dot
	time fdp -Gmaxiter=$(MAXITER) -GK=0.1 -Tsvg -o out-filtered.svg out-filtered.dot
