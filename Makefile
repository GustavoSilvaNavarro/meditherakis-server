#!make
PORT=8080
SERVICE_NAME = meditherakis-server
CONTAINER_NAME = $(SERVICE_NAME)
DOCKER_COMPOSE_TAG = $(SERVICE_NAME)

# format code
format:
	cargo fmt --verbose

up:
	cargo run

# Docker commands / pipeline
run-external-services:
	docker compose -f ./docker-compose.inf.yml up --detach db

down-rm:
	docker compose -f ./docker-compose.inf.yml -f ./docker-compose.yml down --remove-orphans --rmi all --volumes
