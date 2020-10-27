WEB_DIR = web
SCRIPTS_DIR = scripts

# ----------
# Server
# ----------
check:
	cargo clippy

fix:
	cargo fmt
	cargo fix --allow-dirty

run:
	PORT=3000 cargo run --bin omics

production-run:
	PORT=3000 ENV=production cargo run --bin omics

dependencies:
	cargo update

build: dependencies
	cargo build --release

api-test:
	$(MAKE) -C $(SCRIPTS_DIR) test

clean-db:
	$(MAKE) -C $(SCRIPTS_DIR) clean-db

migrate:
	$(MAKE) -C $(SCRIPTS_DIR) migrate

populate:
	$(MAKE) -C $(SCRIPTS_DIR) populate

old-populate:
	$(MAKE) -C $(SCRIPTS_DIR) old-populate

test:
	cargo test --color always

update-samples:
	$(MAKE) -C $(SCRIPTS_DIR) update-samples

backup:
	$(MAKE) -C $(SCRIPTS_DIR) backup


# ----------
# Web
# ----------
web-run:
	$(MAKE) -C $(WEB_DIR) run

web-dependencies:
	$(MAKE) -C $(WEB_DIR) dependencies

web-build: web-dependencies
	$(MAKE) -C $(WEB_DIR) build

web-deploy: web-build
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

docker:
	docker build -t aboglioli/omics-server:latest .
	docker push aboglioli/omics-server:latest

heroku: docker
	docker tag aboglioli/omics-server registry.heroku.com/omics/web
	docker push registry.heroku.com/omics/web
	heroku container:release web
