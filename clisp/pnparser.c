#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <editline/readline.h>
#include "mpc.h"

int main(int argc, char** argv) {
  /* Parsers */
  mpc_parser_t* Number = mpc_new("number");
  mpc_parser_t* Operator = mpc_new("operator");
  mpc_parser_t* Expr = mpc_new("expr");
  mpc_parser_t* Lispy = mpc_new("lispy");
  mpca_lang(MPCA_LANG_DEFAULT,
      " \
        number : /-?[0-9]+/ ; \
        operator : '+' | '-' | '*' | '/' ; \
        expr : <number> | '(' <operator> <expr>+ ')' ; \
        lispy : /^/ <operator> <expr>+ /$/ ; \
      ",
      Number, Operator, Expr, Lispy);

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
      mpc_ast_print(r.output);
      mpc_ast_delete(r.output);
    } else {
      mpc_err_print(r.output);
      mpc_err_delete(r.output);
    }
    printf("#=> %s\n", input);
    free(input);
  }

  mpc_cleanup(4, Number, Operator, Expr, Lispy);
  return 0;
}
