RUST_TARGET = x86_64-unknown-linux-musl
RUST_BIN = omics

WEB_DIR = web

build: server-build web-build

dependencies: server-dependencies web-dependencies

deploy: server-deploy web-deploy

# ----------
# Server
# ----------
server-run:
	PORT=3000 cargo run

server-dependencies:
	cargo update
	rustup target add $(RUST_TARGET)

server-test:
	cargo test

server-build: server-dependencies
	cargo build --release --target $(RUST_TARGET)

server-deploy:
	heroku container:push web
	heroku container:release web

# ----------
# Web
# ----------
web-run:
	$(MAKE) -C $(WEB_DIR) run

web-dependencies:
	$(MAKE) -C $(WEB_DIR) dependencies

web-build: web-dependencies
	$(MAKE) -C $(WEB_DIR) build

web-deploy:
	$(MAKE) -C $(WEB_DIR) deploy

# ----------
# Docker
# ----------
SERVICES = \
	postgres postgres-pgadmin \
	redis redis-commander

docker-up:
	docker-compose up -d $(SERVICES)

docker-down:
	docker-compose down

docker-build: docker-server-build docker-web-build

docker-server-build:
	docker build -t omics-server:latest .

docker-web-build:
	docker build -t omics-web:latest ./web
