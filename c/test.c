#include <stdio.h>
#include <stdint.h>


typedef struct S1 {
  uint32_t field1;
  uint8_t field2;
} S1;

typedef struct S2 {
  uint8_t field1;
  uint32_t field2;
} S2;


int main(int argc, char *argv[]) {
  printf("sizeof(S1) = %d\n", sizeof(S1));
  printf("sizeof(S2) = %d\n", sizeof(S2));
}
