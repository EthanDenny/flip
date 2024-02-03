default:
	cargo build
	cargo run ${TARGET}
	@gcc build/print.c build/out.s -o build/out
	@./build/out
