#ifndef PNPARSER_HEADER
#define PNPARSER_HEADER
#include "mpc.h"

enum { LVAL_NUM, LVAL_ERR };
enum { LERR_DIV_ZERO, LERR_BAD_OP, LERR_BAD_NUM };

typedef struct {
  int type;
  long number;
  int err;
} lisp_value;

int main(int argc, char** argv);
lisp_value lisp_value_number(long x);
lisp_value lisp_value_err(int x);
lisp_value eval_op(lisp_value x, char* op, lisp_value y);
lisp_value eval(mpc_ast_t* t);
void lval_print(lisp_value v);
void lval_println(lisp_value v);

#endif 
