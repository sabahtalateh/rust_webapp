CONTAINER_RUNTIME=docker
DEV_COMPOSE_FILE=docker-compose.dev.yaml
TEST_COMPOSE_FILE=docker-compose.tests.yaml

.PHONY: \
    start-containers \
    stop-containers \
    list-containers \
    run-backend \
    check-backend \
    start-backend-tests-env \
	stop-backend-tests-env \
    backend-tests \
    backend-tests-full

start-containers:
	cd docker && docker-compose -f $(DEV_COMPOSE_FILE) up -d

stop-containers:
	cd docker && docker-compose -f $(DEV_COMPOSE_FILE) down

list-containers:
	cd docker && docker-compose -f $(DEV_COMPOSE_FILE) ps

run-backend: start-containers
	cargo run -p backend

check-backend:
	cargo check -p backend

start-backend-tests-env:
	cd docker && docker-compose -f $(TEST_COMPOSE_FILE) up -d

stop-backend-tests-env:
	cd docker && docker-compose -f $(TEST_COMPOSE_FILE) down

backend-tests: start-backend-tests-env
	cargo test -p backend -- --nocapture

backend-tests-full: stop-backend-tests-env start-backend-tests-env backend-tests
