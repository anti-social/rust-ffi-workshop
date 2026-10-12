#include <stdint.h>
#include <stdio.h>

extern int64_t add(int64_t, int64_t);

int main()
{
    uint64_t a = 2;
    uint64_t b = 3;
    printf("Adding %li and %li is %li\n", a, b, add(a, b));

    return 0;
}

