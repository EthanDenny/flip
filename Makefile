default:
	cargo build
	cargo run ${TARGET}
	@gcc build/print.c build/out.c -o build/out
	@./build/out
