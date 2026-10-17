#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

int main(void) {
    printf(" C lang Arm\n");

    uint32_t value = 0;
    uint32_t *a = &value;

    // View the same memory through 16-bit elements.
    uint16_t *b = (uint16_t *)&value;

    a[0] = UINT32_C(0x12345678);

    printf("32-bit value: %#010" PRIx32 "\n", a[0]);
    printf("16-bit parts: %#06" PRIx16 " --> %#06" PRIx16 "\n", b[0], b[1]);

    return 0;
}
