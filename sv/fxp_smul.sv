
module fxp_smul
#( parameter QINT=8,
    parameter QFRAC=16)
(out, a, b);
    localparam BIT_WIDTH = QINT + QFRAC; 
    output signed [BIT_WIDTH-1:0] out;
    input signed [BIT_WIDTH-1:0] a;
    input signed [BIT_WIDTH-1:0] b;

    wire [BIT_WIDTH*2-1:0] mul_out;
    assign mul_out = a * b;
    //           saa.bbbbb
    //          saa.bbbbb 
    //    [s]aaa[aa.bbbbb]bbbbb
    //    |     |        | 
    //    |     |      QFRAC      -- Bottom of the number starts at QFRAC 
    //    |  BIT_WIDTH*2-QINT -2  -- top of Integer part  is the BIT_WIDTH-1  - QINT   -1 bit because we steal signed from top
    //    BIT_WIDTH-1             -- TOP of number is signed bit
    assign out = {mul_out[BIT_WIDTH-1],mul_out[BIT_WIDTH*2-QINT-2:QFRAC]};
endmodule


