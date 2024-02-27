rm build/out.c
cargo run $1
rm build/out
gcc build/out.c -I src/ -o build/out
./build/out
