#include <stdio.h>

int main() {
    extern int fn_main();
    printf("%d\n", fn_main());
    return 0;
}
