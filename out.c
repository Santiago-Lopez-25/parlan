#include <stdlib.h>
#include <stdio.h>
#include <string.h>

typedef unsigned long long u64;
typedef signed long long i64;
typedef signed int i32;

#define vector Vector*
typedef struct {
    u64    len;
    u64    cap;
    u64    elem_size;
    void**  data;
} Vector;

#define vec_len(vec) (vec->len)
#define vec_cap(vec) (vec->cap)

vector new__vector(u64 elem_size) {
    vector vec = (vector)malloc(sizeof(Vector));
    vec->len = 0;
    vec->elem_size = elem_size;
    vec->cap = 15;
    vec->data = malloc(vec->cap * elem_size);
    return vec;
}

void free__vector(vector vec) {
    free(vec->data);
    free(vec);
    vec = NULL;
}

void push__vector(vector vec, void* data) {
    if (vec->len+1 == vec->cap) {
        vec->cap *= 2;
        vec->data = realloc(vec->data, vec->cap * vec->elem_size);
    }
    *(vec->data + (vec->len * vec->elem_size)) = data;
    vec->len++;
}

void* get__vector(vector vec, u64 idx) {
    if (idx >= vec->len) {
        fprintf(stderr, "error: trying to access index %llu while size is %llu", idx, vec->len);
        return NULL;
    }
    return vec->data + (idx * vec->elem_size);
}

int main() {
    vector usr_vec0 = new__vector(sizeof(i64));
    free__vector(usr_vec0);
    return 0;
}
