BACKEND = omics-backend
FRONTEND = omics-frontend

RUST_TARGET = x86_64-unknown-linux-musl
RUST_BIN = omics

WEB_DIR = web

SERVICES = \
	postgres postgres-pgadmin \
	redis redis-commander

# Build
build: server-build web-build

server-build: server-dependencies
	cargo build --release --target $(RUST_TARGET)
	docker build . -t $(RUST_BIN):latest --no-cache

web-build: web-dependencies
	$(MAKE) -C $(WEB_DIR) build

# Dependencies
dependencies: server-dependencies web-dependencies

server-dependencies:
	rustup target add $(RUST_TARGET)

web-dependencies:
	$(MAKE) -C $(WEB_DIR) dependencies

# Run
server-run:
	cargo run

web-run:
	$(MAKE) -C $(WEB_DIR) serve

# Docker
docker-up:
	docker-compose up -d $(SERVICES)

docker-down:
	docker-compose down
