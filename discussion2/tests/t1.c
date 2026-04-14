#include <stdio.h>

int calc(int, int);

int main() {
    int a, b, c;
    scanf("%d%d", &a, &b);
    c = calc(a, b);
    printf("%d\n", c);
}