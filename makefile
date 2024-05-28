
%.json: %.sv 	; yosys -p "plugin -i systemverilog; read_systemverilog $<; proc; write_json -compat-int out/${@F}";
%.svg: %.json 	; netlistsvg out/${<F} -o out/${@F}
%.cpp: %.sv     ; yosys -p "plugin -i systemverilog; read_systemverilog $<; proc; write_cxxrtl out/${*F}.cpp"
%.tb: %.cpp     ; clang++ -g -O3 -std=c++17 -I `yosys-config --datdir`/include -I out/ cpp/${*F}_tb.cpp -DTB_VCD_FILE="\"out/${*F}.vcd\"" -o out/${*F}_tb; out/${*F}_tb

clean: 
	rm -rf out/*
