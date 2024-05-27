
%.json: %.sv 	; yosys -p "plugin -i systemverilog; read_systemverilog $<; proc; write_json -compat-int out/${@F}";
%.svg: %.json 	; netlistsvg out/${<F} -o out/${@F}

clean: 
	rm -rf out/*
