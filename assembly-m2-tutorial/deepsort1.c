#include <stdio.h>

void move37(long *);

int main() {
  long A[3] = {3, 1, 2};
  move37(A);
  printf("%d %d %d\n", A[0], A[1], A[2]);
}
