#include <iostream>
#include "cordic.cpp"
#include <fstream>
#include <backends/cxxrtl/cxxrtl_vcd.h>


using namespace std;

int main() {

    cxxrtl_design::p_cordic top;
    cxxrtl::debug_items all_debug_items;

    top.debug_info(all_debug_items);
    cxxrtl::vcd_writer vcd;
    vcd.timescale(1, "us");
    vcd.add_without_memories(all_debug_items);

    std::ofstream waves(TB_VCD_FILE);

    top.step();
    top.p_i__theta.set<uint32_t>(0x18000);
    vcd.sample(0);
    for(int i = 0; i<200; i++) {
        top.p_i__clk.set<bool>(true);
        top.step();
        vcd.sample(i*2+0);
        top.p_i__clk.set<bool>(false);
        top.step();
        vcd.sample(i*2+1);
    }
    uint32_t cosine = top.p_o__cosine.get<uint32_t>();
    cout << "cosine: " <<  cosine << endl;

    uint32_t theta = top.p_i__theta.get<uint32_t>();
    cout << "theta: " <<  theta << endl;

    waves << vcd.buffer;
    vcd.buffer.clear();

    
}
