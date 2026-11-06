#include <stdbool.h>
#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>
bool flag_has(const unsigned char *, size_t, int);
int main(void) {
    unsigned char *flags = malloc(1);
    flags[0] = 0;
    printf("%d\n", flag_has(flags, 1, 9));
    free(flags);
}
