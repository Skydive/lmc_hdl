#include <stdio.h>
#include <stdlib.h>
#include <iostream>
#include <bitset>
using namespace std;

#include "Vlmc.h"
#include "verilated.h"
#include "verilated_vcd_c.h"

void tick(Vlmc* tb, int tc, VerilatedVcdC* tfp) {
  tb->eval(); if(tfp)tfp->dump(tc*10-2);
  tb->clk = 1;
  tb->eval(); if(tfp)tfp->dump(tc*10);
  tb->clk = 0;
  tb->eval();
  if(tfp) {
    tfp->dump(tc*10+5);
    tfp->flush();
  }
}

int main(int argc, char** argv) {
  Verilated::commandArgs(argc, argv);
  Verilated::traceEverOn(true);

  Vlmc *tb = new Vlmc;
  VerilatedVcdC* tfp = new VerilatedVcdC;

  //tb->trace(tfp, 99);
  //tfp->open("trace.vcd");

  unsigned tc=0;
  for(int k=0; k<(1<<21); k++) {

    //printf("STATE:\t%d\n", tb->o_state);
    //cout << "LDINS:\t" << bitset<16>(tb->o_ins) << endl;

    tick(tb, ++tc, tfp);
    printf("Acc: %d\n", tb->out);
    printf("f1: %d\n", tb->lmc__DOT__mem[14]);
    printf("f2: %d\n", tb->lmc__DOT__mem[15]);
    printf("cnt: %d\n", tb->lmc__DOT__mem[16]);
    printf("tmp: %d\n", tb->lmc__DOT__mem[17]);
    printf("N1: %d\n", tb->lmc__DOT__mem[18]);

    printf("\n");
    if(tb->o_hlt) {
      printf("Terminated execution:\n");
      printf("Cycles: %d\n", tc);
      printf("Result: %d\n", tb->out);
      break;
    }
    //while(getchar()!='\n'); 
  }
}
