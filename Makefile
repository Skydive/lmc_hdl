CC=g++
VV=verilator
CFLAGS = $(shell pkg-config verilator --cflags) -Iobj_dir
CPPFILES = /usr/share/verilator/include/verilated{,_vcd_c}.cpp

test: lmc.v test.cpp out.hex
	$(VV) -cc lmc.v -Wall --trace
	pushd obj_dir && make -f Vlmc.mk && popd
	$(CC) $(CFLAGS) test.cpp obj_dir/Vlmc__ALL.a $(CPPFILES) -o test

all: test
clean:
	rm -Rf obj_dir
	rm -f test


