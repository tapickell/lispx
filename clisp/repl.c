#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <editline/readline.h>

int main(int argc, char** argv) {
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
    printf("#=> %s\n", input);
    free(input);
  }
  return 0;
}
