
module Cordic(i_clk, o_cosine, o_sine, i_theta);

    parameter width = 16;
    
    //inputs
    input i_clk;
    input signed [31:0] i_theta;

    output signed [width:0] o_cosine;
    output signed [width:0] o_sine;

    wire signed [31:0] cordic_angles [0:15];
    assign cordic_angles[00] = 'h0000c90f;
    assign cordic_angles[01] = 'h000076b1;
    assign cordic_angles[02] = 'h00003eb6;
    assign cordic_angles[03] = 'h00001fd5;
    assign cordic_angles[04] = 'h00000ffa;
    assign cordic_angles[05] = 'h000007ff;
    assign cordic_angles[06] = 'h000003ff;
    assign cordic_angles[07] = 'h000001ff;
    assign cordic_angles[08] = 'h000000ff;
    assign cordic_angles[09] = 'h0000007f;
    assign cordic_angles[10] = 'h0000003f;
    assign cordic_angles[11] = 'h0000001f;
    assign cordic_angles[12] = 'h00000010;
    assign cordic_angles[13] = 'h00000008;
    assign cordic_angles[14] = 'h00000004;
    assign cordic_angles[15] = 'h00000002;
    wire signed [31:0] cos_prod2;
    assign cos_prod2 = 'd24166;

    reg signed [width:0] x   [0:2*(width-1)];
    reg signed [width:0] y   [0:2*(width-1)];
    reg signed    [31:0] phi [0:2*(width-1)];

    genvar i,k;
    generate 
    for (i = 0; i < width-1; i=i+1)
    begin: outer
        for(k = 0; k < 2; k=k+1)
        begin: corder_iter
            wire phi_cmp;
            wire signed [width:0] x_shr, y_shr;

            assign x_shr = x[2*i + k] >>> i;
            assign y_shr = y[2*i + k] >>> i;

            assign phi_cmp = phi[i] < i_theta ? 1 : 0;

            always @(posedge i_clk)
            begin 
                  x[2*i + k]  <= phi_cmp ?   x[2*i+k] + y_shr : x[2*i+k] - y_shr;
                  y[2*i + k]  <= phi_cmp ?   y[2*i+k] - x_shr : y[2*i+k] + x_shr;
                phi[2*i + k]  <= phi_cmp ? phi[2*i+k] + cordic_angles[i] : phi[2*i+k] - cordic_angles[i];
            end
        end
    end
    endgenerate


    assign o_cosine = x[2*(width-1)] * cos_prod2;
    assign o_sine =   y[2*(width-1)] * cos_prod2;

endmodule
