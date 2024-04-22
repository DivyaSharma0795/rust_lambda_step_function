# Name of your output file
OUTPUT = bootstrap

# Build rule
all: build-HelloWorldFunction

build-HelloWorldFunction:
	cargo build --release
	cp target/release/my_lambda_function $(OUTPUT)

# Clean rule
clean:
	cargo clean