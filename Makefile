NAME	:=	matchstick
FOLDER	:=	debug

all: build install

build:
	cargo build

install:
	cp target/$(FOLDER)/$(NAME) .

fclean:
	rm -rf $(NAME)
	rm -rf obj/

.PHONY:	all build install fclean

