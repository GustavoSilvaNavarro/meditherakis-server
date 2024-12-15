#!make
PORT=8080
SERVICE_NAME = meditherakis-server
CONTAINER_NAME = $(SERVICE_NAME)
DOCKER_COMPOSE_TAG = $(SERVICE_NAME)

# Sea ORM migrations
create-migration:
	cargo run -- generate '$(m)'

migration-up:
	cargo run -- up

migration-down:
	cargo run -- down

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
