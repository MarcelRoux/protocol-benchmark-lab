SHELL := /bin/bash

COMPOSE_FILE := infra/compose.yaml
PROJECT := protocol-benchmark-lab

.PHONY: help up up-build down build rebuild ps images logs config

help:
	@echo "Available targets:"
	@echo "  up         - Start the services stack (no implicit rebuild)."
	@echo "  up-build   - Build images, then recreate containers."
	@echo "  build      - Build images only."
	@echo "  rebuild    - Build images without cache."
	@echo "  ps         - Show running compose services."
	@echo "  images     - Show image IDs used by compose services."
	@echo "  logs       - Tail logs for all services."
	@echo "  config     - Render resolved compose configuration."
	@echo "  down       - Stop the services stack."

up:
	docker compose -f $(COMPOSE_FILE) -p $(PROJECT) up -d

up-build:
	docker compose -f $(COMPOSE_FILE) -p $(PROJECT) up -d --build --force-recreate

build:
	docker compose -f $(COMPOSE_FILE) -p $(PROJECT) build --progress=plain

rebuild:
	docker compose -f $(COMPOSE_FILE) -p $(PROJECT) build --no-cache --progress=plain

ps:
	docker compose -f $(COMPOSE_FILE) -p $(PROJECT) ps

images:
	docker compose -f $(COMPOSE_FILE) -p $(PROJECT) images

logs:
	docker compose -f $(COMPOSE_FILE) -p $(PROJECT) logs -f --tail=200

config:
	docker compose -f $(COMPOSE_FILE) -p $(PROJECT) config

down:
	docker compose -f $(COMPOSE_FILE) -p $(PROJECT) down
