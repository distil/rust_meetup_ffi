#include <stdint.h>
#include <stdio.h>
#include <string.h>

char *stringify(int32_t x);
void release_string(char *string);

int main(int argc, char **argv) {
	char *string = stringify(42);
	printf("stringify: %s\n", string);
	release_string(string);

	return 0;
}
