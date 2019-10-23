CONTAINER_RUNTIME=docker

.PHONY: \
    run-containers \
    stop-containers \
    list-containers \
    run-backend \
    check-backend


run-containers:
	cd docker && docker-compose up -d

stop-containers:
	cd docker && docker-compose down

list-containers:
	cd docker && docker-compose ps

run-backend:
	cargo run -p backend

check-backend:
	cargo check -p backend
