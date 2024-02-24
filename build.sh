rm build/out.c
cargo run $1
rm build/out
gcc -I src/ src/print.c build/out.c -o build/out
./build/out
