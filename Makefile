CARGO=cargo

SRC_DIR=src
BUILD_DIR=target
DB=~/.local/share/raspi.db
OLD_DB=~/.local/share/old_raspi.db
DB_DIR=$(SRC_DIR)/database

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
		validator -F validator/derive \
		futures_util

	@touch $@

build: $(BUILD_DIR)/release/rspi-api

$(BUILD_DIR)/release/rspi-api: install
	cargo build --release

run: build
	cargo run

test: build
	cargo test

database:
	-sqlite3 ~/.local/share/raspi.db .dump | grep -v "^CREATE" | grep -v "^[[:blank:]]" | grep -v "^);" > $(DB_DIR)/dump.sql
	-rm $(OLD_DB)
	-mv $(DB) $(OLD_DB)
	sqlite3 $(DB) < $(DB_DIR)/schema.sql
	-sqlite3 $(DB) < $(DB_DIR)/dump.sql
	-rm $(DB_DIR)/dump.sql

clean:
	rm -fr $(BUILD_DIR)/
	rm Cargo.toml
	rm Cargo.lock
	cargo clean

docker: build
	docker buildx build --tag ${DOCKER_NAME} .
