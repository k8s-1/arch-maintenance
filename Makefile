BINARY_NAME=./target/release/rust-maintenance

all: build run

build:
	cargo build --release

run: build
	./${BINARY_NAME}

clean:
	rm ${BINARY_NAME}
