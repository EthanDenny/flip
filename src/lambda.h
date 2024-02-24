#include <stdlib.h>

struct {
    char evaluated;
    union {
        struct {
            void* func;
            char* args;
        };
        long return_v;
    };
} typedef lambda_t;

lambda_t* lambda(void* fn, void* args) {
    lambda_t* l = malloc(sizeof(lambda_t));
    *l = (lambda_t) {
        .evaluated = 0,
        .func = fn,
        .args = args
    };
    return l;
}

#define get_arg(args, type) \
    *((type*) ((args += sizeof(type)) - sizeof(type)));

#define add_arg(args, arg) {\
    *((typeof(arg)*) args) = arg;\
    args += sizeof(arg);\
}

long eval(lambda_t* x) {
    if (!x->evaluated) {
        void* arg_ptr = x->args;
        x->return_v = ((long (*)(void*))x->func)(x->args);
        x->evaluated = 1;
        free(arg_ptr);
    }
    return x->return_v;
}

typedef lambda_t* fn;
