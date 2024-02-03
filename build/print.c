#include <stdio.h>

int main() {
    extern int _main();
    printf("%d\n", _main());
    return 0;
}
