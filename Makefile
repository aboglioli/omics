WEB_DIR = web

SERVICES = \
	postgres postgres-pgadmin \
	redis redis-commander

build:
	cargo build --release
	$(MAKE) -C $(WEB_DIR) build

server-run:
	cargo run

web-run:
	$(MAKE) -C $(WEB_DIR) serve

docker-up:
	docker-compose up -d $(SERVICES)

docker-down:
	docker-compose down
