all: build web

build:
	cargo build 

server:
	cargo run

front:
	cd web
	ng serve