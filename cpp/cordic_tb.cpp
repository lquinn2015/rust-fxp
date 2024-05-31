#include <iostream>
#include "cordic.cpp"
#include <fstream>
#include <backends/cxxrtl/cxxrtl_vcd.h>


using namespace std;

void print_iter() {

    cxxrtl_design::p_cordic top;


}


int main() {

    cxxrtl_design::p_cordic top;
    cxxrtl::debug_items all_debug_items;

    top.debug_info(all_debug_items);
    cxxrtl::vcd_writer vcd;
    vcd.timescale(1, "us");
    vcd.add_without_memories(all_debug_items);

    std::ofstream waves(TB_VCD_FILE);

    std::vector<uint32_t> angles = {0x18000, 0x8000, 0x20000};
    for(int k = 0; k < 3; k++) {
        auto theta = angles[k];
        top.step();
        top.p_i__theta.set<uint32_t>(theta);
        top.p_i__en.set<bool>(true);
        vcd.sample(k*500);
        for(int i = 0; i<500; i++) {
            print_iter();
            top.p_i__clk.set<bool>(true);
            top.step();
            top.p_i__en.set<bool>(false);
            vcd.sample(i*2+0 + k * 500);
            top.p_i__clk.set<bool>(false);
            top.step();
            vcd.sample(i*2+1 + k * 500);
        }
        cout << "theta: " << std::hex << theta << " :: " <<  (((double)theta)/((double)(1<<16))) << endl;

        uint32_t cosine = top.p_o__cosine.get<uint32_t>();
        cout << "cosine: " << std::hex << cosine << " :: " <<  (((double)cosine)/((double)(1<<16))) << endl;

        uint32_t sine = top.p_o__sine.get<uint32_t>();
        cout << "sine: " << std::hex << sine << " :: " << (((double)sine)/((double)(1<<16))) << endl;
    }

    waves << vcd.buffer;
    vcd.buffer.clear();

    
}
