#pragma once

#include <stdlib.h>

// Lists

struct list_node_t {
    char empty;
    long head;
    struct list_node_t* tail;
} typedef list_node;

list_node* push(list_node* list, long value) {
    list_node* next_list = (list_node*) malloc(sizeof(list_node));
    next_list->head = value;
    next_list->tail = list;
}

long len(list_node* list) {
    long length = 0;
    while (list != NULL) {
        length += 1;
        list = list->tail;
    }
    return length;
}

typedef list_node* list;

// Lambdas

union {
    long as_i;
    list as_l;
} typedef return_t;

struct {
    char evaluated;
    union {
        struct {
            void* func;
            char* args;
        };
        return_t return_v;
    };
} typedef lambda_t;

lambda_t* lambda(void* fn, void* args) {
    lambda_t* l = (lambda_t*) malloc(sizeof(lambda_t));
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

return_t eval(lambda_t* x) {
    if (!x->evaluated) {
        void* arg_ptr = x->args;
        x->return_v = ((return_t (*)(void*))x->func)(x->args);
        x->evaluated = 1;
        free(arg_ptr);
    }
    return x->return_v;
}

typedef lambda_t* fn;
