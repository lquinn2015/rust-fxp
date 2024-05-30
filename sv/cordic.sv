
module cordic(i_clk, i_en, o_vld, o_cosine, o_sine, i_theta);

    localparam ITER = 16;
    localparam ITER_BITS = $clog2(ITER);
    localparam QBITS = 16;
    localparam RBITS = 8;
    localparam RQBITS = QBITS + RBITS;
    localparam RQBITS_ACC = RQBITS*2;
    localparam width = 16;
    
    //inputs
    input i_clk;
    input i_en;
    input signed [RQBITS-1:0] i_theta;
    reg [31:0] r_theta;

    output signed [RQBITS-1:0] o_cosine;
    output signed [RQBITS-1:0] o_sine;
    output reg o_vld;

    wire signed [RQBITS-1:0] cordic_angles [0:ITER-1];
    assign cordic_angles[00] = 'h00c90f;
    assign cordic_angles[01] = 'h0076b1;
    assign cordic_angles[02] = 'h003eb6;
    assign cordic_angles[03] = 'h001fd5;
    assign cordic_angles[04] = 'h000ffa;
    assign cordic_angles[05] = 'h0007ff;
    assign cordic_angles[06] = 'h0003ff;
    assign cordic_angles[07] = 'h0001ff;
    assign cordic_angles[08] = 'h0000ff;
    assign cordic_angles[09] = 'h00007f;
    assign cordic_angles[10] = 'h00003f;
    assign cordic_angles[11] = 'h00001f;
    assign cordic_angles[12] = 'h000010;
    assign cordic_angles[13] = 'h000008;
    assign cordic_angles[14] = 'h000004;
    assign cordic_angles[15] = 'h000002;
    wire signed [RQBITS-1:0] cos_prod2;
    assign cos_prod2 = 'h5E66;

    reg signed [RQBITS-1:0] x   [0:2*(ITER-1)];
    reg signed [RQBITS-1:0] y   [0:2*(ITER-1)];
    reg signed [RQBITS-1:0] phi [0:2*(ITER-1)];
    reg           [5:0]    vld;

    assign o_vld = vld[5];

    always @(posedge i_clk) 
    begin 
        if (i_en == 1) begin
            x[0] <= 'h10000;
            y[0] <= 0;
            phi[0] <= i_theta;
            vld <= 1;
        end
        else if (vld[5]) begin
            vld <= 0;
        end
        else if (vld != 0) begin 
            vld <= vld + 1;
        end 
    end

    genvar i,k;
    generate 
    for (i = 0; i < ITER; i=i+1)
    begin: outer
        for(k = 0; k < 2; k=k+1)
        begin: corder_iter
            wire phi_cmp;
            wire signed [RQBITS-1:0] x_shr, y_shr;

            assign x_shr = x[2*i + k] >>> i;
            assign y_shr = y[2*i + k] >>> i;

            assign phi_cmp = phi[2*i + k][23];

            always @(posedge i_clk)
            begin 
                  x[2*i + k + 1]  <= phi_cmp ?   x[2*i+k] + y_shr : x[2*i+k] - y_shr;
                  y[2*i + k + 1]  <= phi_cmp ?   y[2*i+k] - x_shr : y[2*i+k] + x_shr;
                phi[2*i + k + 1]  <= phi_cmp ? phi[2*i+k] + cordic_angles[i] : phi[2*i+k] - cordic_angles[i];
            end
        end
    end
    endgenerate

    reg signed [RQBITS_ACC-1:0] accX; assign accX =  x[2*(ITER-1)] * cos_prod2;
    reg signed [RQBITS_ACC-1:0] accY; assign accY =  y[2*(ITER-1)] * cos_prod2;

    assign o_cosine = accX[RQBITS_ACC-1:RBITS*2];
    assign o_sine =   accY[RQBITS_ACC-1:RBITS*2]; 
endmodule
