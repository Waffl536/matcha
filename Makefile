.DEFAULT_GOAL := test

run:
	cargo run ${ARGS}

test3:
	cargo run /Users/vladyslav/Documents/code/rust/matcha/test3.mt ${ARGS}

test2:
	cargo run /Users/vladyslav/Documents/code/rust/matcha/test2.mt ${ARGS}
	
test:
	cargo run /Users/vladyslav/Documents/code/rust/matcha/testfile.mt ${ARGS}

release:
	cargo build --release