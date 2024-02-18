default:
	cargo build
	cargo run ${TARGET}
	@gcc print.c build/out.c -o build/out
	@./build/out
