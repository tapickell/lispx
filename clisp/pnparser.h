#ifndef PNPARSER_HEADER
#define PNPARSER_HEADER
#include "mpc.h"

int main(int argc, char** argv);
long eval_op(long x, char* op, long y);
long eval(mpc_ast_t* t);

#endif 
