#include <stdlib.h>
#include <stdio.h>

int operacion_0(int n) {
    int temp = n * 2;
    if (temp > 100) {
        return temp + 0;
    }
    int res_0 = operacion_0(30);
    if (res_0 < 1000) {
        res_0 = res_0 + 1;
    }
    return temp;
}
