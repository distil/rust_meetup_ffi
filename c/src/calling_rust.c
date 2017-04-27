#include <stdint.h>
#include <stdio.h>

int32_t squared(int32_t x);

int main(int argc, char **argv) {
	printf("5 * 5 = %d\n", squared(5));
	return 0;
}
