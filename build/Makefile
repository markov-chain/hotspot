root := $(CARGO_MANIFEST_DIR)
output := $(OUT_DIR)

flags := -fPIC -O3
build := $(root)/build
source := $(build)/source

$(output)/libcircuit.a: $(output)/libhotspot.a $(output)/circuit.o
	cp $(output)/libhotspot.a $@
	ar -q $@ $(output)/circuit.o

$(output)/libhotspot.a:
	$(MAKE) -C $(source) -j$(NUM_JOBS) EXTRAFLAGS='$(flags)'
	cp $(source)/libhotspot.a $@
	$(MAKE) -C $(source) clean

$(output)/circuit.o: $(build)/circuit.c
	$(CC) $(flags) -I$(source) -I$(build) -c $< -o $@
