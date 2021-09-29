#ifndef PNPARSER_HEADER
#define PNPARSER_HEADER
#include "mpc.h"

enum {
  LVAL_ERR,
  LVAL_NUM,
  LVAL_SEXPR,
  LVAL_SYM
};

typedef struct lisp_value {
  int type;
  long number;
  char* err;
  char* sym;
  int count;
  struct lisp_value** cell;
} lisp_value;

int main(int argc, char** argv);
lisp_value* lisp_value_number(long x);
lisp_value* lisp_value_err(char* m);
lisp_value* lisp_value_sym(char* m);
lisp_value* lisp_value_read_number(mpc_ast_t* t);
lisp_value* lisp_value_read(mpc_ast_t* t);
lisp_value* lisp_value_add(lisp_value* v, lisp_value* x);
void lisp_value_del(lisp_value* v);
lisp_value eval_op(lisp_value x, char* op, lisp_value y);
lisp_value eval(mpc_ast_t* t);
void lval_print(lisp_value* v);
void lval_println(lisp_value* v);
void lval_expr_print(lisp_value* v, char open, char close);

#endif 
