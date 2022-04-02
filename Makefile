.PHONY: pad build

build: pad
	cargo build --release

pad:
	sh ./pad_words.sh