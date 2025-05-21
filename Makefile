CARGO=cargo

SRC_DIR=src
BUILD_DIR=target

DOCKER_NAME=rspi-api-docker

.PHONY: init install build run test clean docker

ifeq ("$(wildcard Cargo.lock)", "")
INIT_CMD := cargo init
else
INIT_CMD := @echo "Cargo already initialized..."
endif

init:
	$(INIT_CMD)

install: Cargo.toml

Cargo.toml: init
	cargo add actix-web \
		actix-cors \
		serde-json \
		serde -F serde/derive \
		chrono -F chrono/serde \
		env_logger \
		dotenv \
		uuid -F "uuid/serde uuid/v4" \
		rusqlite \
		r2d2 \
		r2d2_sqlite \
		jsonwebtoken \
		bcrypt \
		validator -F validator/derive

	@touch $@

build: $(BUILD_DIR)/release/rspi-api

$(BUILD_DIR)/release/rspi-api: install
	cargo build --release

run: build
	cargo run

test: build
	cargo test

clean:
	cargo clean

docker: build
	docker buildx build --tag ${DOCKER_NAME} .
