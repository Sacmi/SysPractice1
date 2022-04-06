#include <stdio.h>

unsigned long long factorial(unsigned long long n) {
    if (n == 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

int main() {
    for (size_t i = 0; i < 1000000; i++)
    {
        factorial(20);
    }
}
