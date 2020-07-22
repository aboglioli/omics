RUST_TARGET = x86_64-unknown-linux-musl
RUST_BIN = omics

WEB_DIR = web

build: server-build web-build

dependencies: server-dependencies web-dependencies

# ----------
# Server
# ----------
server-build: server-dependencies
	cargo build --release --target $(RUST_TARGET)

server-dependencies:
	cargo update
	rustup target add $(RUST_TARGET)

server-run:
	PORT=3000 cargo run

# ----------
# Web
# ----------
web-build: web-dependencies
	$(MAKE) -C $(WEB_DIR) build

web-dependencies:
	$(MAKE) -C $(WEB_DIR) dependencies

web-run:
	$(MAKE) -C $(WEB_DIR) run

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

# ----------
# Deploy
# ----------
server-deploy:
	git push heroku master

web-deploy:
	$(MAKE) -C $(WEB_DIR) deploy
