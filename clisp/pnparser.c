#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <editline/readline.h>
#include "pnparser.h"

int main(int argc, char** argv) {
  /* Parsers */
  mpc_parser_t* Number = mpc_new("number");
  mpc_parser_t* Symbol = mpc_new("symbol");
  mpc_parser_t* Sexpr = mpc_new("sexpr");
  mpc_parser_t* Expr = mpc_new("expr");
  mpc_parser_t* Lispy = mpc_new("lispy");
  mpca_lang(MPCA_LANG_DEFAULT,
      " \
        number : /-?[0-9]+/ ; \
        symbol : '+' | '-' | '*' | '/' ; \
        sexpr : '(' <expr>* ')' ; \
        expr : <number> | <symbol> | <sexpr> ; \
        lispy : /^/ <expr>* /$/ ; \
      ",
      Number, Symbol, Sexpr, Expr, Expr, Lispy);

  puts("LispC Version 0.0.0.0.1");
  puts("Interactive LispC - Press Ctrl+c to exit");
  read_history(".lispc_history");

  while (1) {
    char* input = readline("lispc> ");
    if (strcmp(input, "(:exit)") == 0) {
      return 0;
    }
    add_history(input);
    write_history(".lispc_history");

    mpc_result_t r;
    if (mpc_parse("<stdin>", input, Lispy, &r)) {
      lisp_value result = eval(r.output);
      lval_println(result);
      /*mpc_ast_print(r.output);*/
      mpc_ast_delete(r.output);
    } else {
      mpc_err_print(r.output);
      mpc_err_delete(r.output);
    }
    free(input);
  }

  mpc_cleanup(5, Number, Symbol, Sexpr, Expr, Lispy);
  return 0;
}

lisp_value eval(mpc_ast_t* t) {
  if (strstr(t->tag, "number")) {
    errno = 0;
    long c = strtol(t->contents, NULL, 10);
    return errno != ERANGE ? lisp_value_number(c) : lisp_value_err(LERR_BAD_NUM);
  }

  char* op = t->children[1]->contents;

  lisp_value x = eval(t->children[2]);

  int i = 3;
  while (strstr(t->children[i]->tag, "expr")) {
    x = eval_op(x, op, eval(t->children[i]));
    i++;
  }

  return x;
}

lisp_value eval_op(lisp_value x, char* op, lisp_value y) {
  if (x.type == LVAL_ERR) { return x; }
  if (y.type == LVAL_ERR) { return y; }

  if (strcmp(op, "+") == 0) { return lisp_value_number(x.number + y.number); }
  if (strcmp(op, "-") == 0) { return lisp_value_number(x.number - y.number); }
  if (strcmp(op, "*") == 0) { return lisp_value_number(x.number * y.number); }
  if (strcmp(op, "/") == 0) {
    return y.number == 0
      ? lisp_value_err(LERR_DIV_ZERO)
      : lisp_value_number(x.number / y.number);
  }
  return lisp_value_err(LERR_BAD_OP);
}

lisp_value* lisp_value_number(long x) {
  lisp_value* v = malloc(sizeof(lisp_value));
  v->type = LVAL_NUM;
  v->number = x;
  return v;
}

lisp_value* lisp_value_err(char* m) {
  lisp_value* v = malloc(sizeof(lisp_value));
  v->type = LVAL_ERR;
  v->err = malloc(strlen(m) + 1);
  strcpy(v->err, m);
  return v;
}

lisp_value* lisp_value_sym(char* m) {
  lisp_value* v = malloc(sizeof(lisp_value));
  v->type = LVAL_SYM;
  v->sym = malloc(strlen(m) + 1);
  strcpy(v->sym, m);
  return v;
}

lisp_value* lisp_value_sexpr(void) {
  lisp_value* v = malloc(sizeof(lisp_value));
  v->type = LVAL_SYM;
  v->count = 0;
  v->cell = NULL;
  return v;
}

lisp_value* lisp_value_read_number(mpc_ast_t* t) {
  errno = 0;
  long c = strtol(t->contents, NULL, 10);
  return errno != ERANGE ? lisp_value_number(c) : lisp_value_err("invalid number");
}

void lisp_value_del(lisp_value* v) {
  switch (v->type) {
    case LVAL_NUM: break;
    case LVAL_ERR: free(v->err); break;
    case LVAL_SYM: free(v->sym); break;
    case LVAL_SEXPR:
                   for (int i = 0; i < v->count; i++) {
                     lisp_value_del(v->cell[i]);
                   }
                   free(v->cell);
                   break;
  }
  free(v);
}

/* Print an "lval" */
void lval_print(lisp_value v) {
  switch (v.type) {
    /* In the case the type is a number print it */
    /* Then 'break' out of the switch. */
    case LVAL_NUM: printf("%li", v.number); break;

    /* In the case the type is an error */
    case LVAL_ERR:
      /* Check what type of error it is and print it */
      if (v.err == LERR_DIV_ZERO) {
        printf("Error: Division By Zero!");
      }
      if (v.err == LERR_BAD_OP)   {
        printf("Error: Invalid Operator!");
      }
      if (v.err == LERR_BAD_NUM)  {
        printf("Error: Invalid Number!");
      }
    break;
  }
}

/* Print an "lval" followed by a newline */
void lval_println(lisp_value v) { lval_print(v); putchar('\n'); }
