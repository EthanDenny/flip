cargo run $1
gcc -I src/ src/print.c build/out.c -o build/out
./build/out
