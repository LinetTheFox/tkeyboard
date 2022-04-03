.PHONY: pad build install uninstall

NAME=tkeyboard
VERSION=v0.1.0

# installation destination
DEST=/usr/local
BINDEST=${DEST}/bin
MANDEST=${DEST}/share/man/man1
RESDEST=${HOME}/.config/tkeyboard

# Repo folders vars
SRCROOT=src
OUTROOT=target/release
RESROOT=res

build: pad
	cargo build --release

pad:
	sh ./pad_words.sh

# Needs root access
install: build
	mkdir -p $(RESDEST)
	cp $(RESROOT)/words_padded.txt $(RESDEST)
	sudo cp $(OUTROOT)/$(NAME) $(BINDEST)/$(NAME)
	sudo cp $(NAME).1 $(MANDEST)

uninstall:
	sudo rm -f $(BINDEST)/$(NAME)
	sudo rm -f $(MANDEST)/$(NAME)
	rm -rf $(RESDEST)