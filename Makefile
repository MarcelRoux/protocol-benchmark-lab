SHELL := /bin/bash

COMPOSE_FILE := infra/compose.yaml

.PHONY: help up down

help:
	@echo "Available targets:"
	@echo "  up    - Start the services stack."
	@echo "  down  - Stop the services stack."

up:
	docker compose -f $(COMPOSE_FILE) -p protocol-benchmark-lab up -d

down:
	docker compose -f $(COMPOSE_FILE) -p protocol-benchmark-lab down