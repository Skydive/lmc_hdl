/* verilator lint_off UNUSED */

module lmc(/*AUTOARG*/
   // Outputs
   out, o_ins, o_state,
   // Inputs
   clk, o_hlt
   );
   input clk;
   output [15:0] out;

   output [15:0] o_ins;
   assign o_ins = mem[pc[11:0]];

   output [3:0]      o_state;
   assign o_state = state;

   output            o_hlt;
   assign o_hlt = hlt;

   reg [15:0]        pc = 0;
   reg        hlt = 0;
   reg [15:0] acc = 0;
   assign out = acc;

   reg [15:0] mem [0:(1<<12)-1]; // 64KB
   initial $readmemh("./out.hex", mem);

   localparam STATE_FETCH  = 0;
   localparam STATE_EXECUTE= 1;
   reg [3:0] state = 0;

   localparam CMD_HLT = 3'b000;
   localparam CMD_ADD = 3'b001;
   localparam CMD_SUB = 3'b010;
   localparam CMD_STA = 3'b011;
   localparam CMD_BRP = 3'b100;
   localparam CMD_LDA = 3'b101;
   localparam CMD_BRA = 3'b110;
   localparam CMD_BRZ = 3'b111;
   reg [15:0] ins = {13'b0, CMD_HLT};


   // DEBUG MEMORY
   /*reg [11:0]  cnt = 0;
   always @(posedge clk) if(!hlt) begin
      $display("%d: %h", cnt, mem[cnt]);
      cnt <= cnt + 1;
   end*/

   always @(posedge clk) if(!hlt) begin
      case (state)
        STATE_FETCH: begin
           ins <= mem[pc[11:0]];
           pc <= pc + 1;
           state <= STATE_EXECUTE;
           //$display("PC: %d, INS: %b", pc, mem[pc[11:0]]);
        end
        STATE_EXECUTE: begin
           logic [2:0]  cmd = ins[2:0];
           logic [11:0] arg = ins[15:4];
           //$display("CMD: %b, ARG: %d", cmd, arg);
           case (cmd)
             CMD_HLT: hlt <= 1;
             CMD_ADD: acc <= acc + mem[arg];
             CMD_SUB: acc <= acc - mem[arg];
             CMD_STA: mem[arg] <= acc;
             // unimplemented: CMD_BRP
             CMD_LDA: acc <= mem[arg];
             CMD_BRA: pc <= {4'b0, arg};
             CMD_BRZ:
                if(acc == 0) begin
                   pc <= {4'b0, arg};
                end
             default: ; // no-op
           endcase
           state <= STATE_FETCH;
        end
      endcase
   end
endmodule
