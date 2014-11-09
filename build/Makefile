root := $(CARGO_MANIFEST_DIR)
into := $(OUT_DIR)
jobs := $(NUM_JOBS)

flags := -fPIC -O3
build := $(root)/build
hotspot := $(build)/libhotspot

$(into)/libcircuit.a: $(into)/libhotspot.a $(into)/circuit.o
	cp $(into)/libhotspot.a $@
	ar -q $@ $(into)/circuit.o

$(into)/libhotspot.a:
	$(MAKE) -C $(hotspot) -j$(jobs) EXTRAFLAGS='$(flags)'
	cp $(hotspot)/libhotspot.a $@
	$(MAKE) -C $(hotspot) clean

$(into)/circuit.o: $(build)/circuit.c
	$(CC) $(flags) -I$(hotspot) -I$(build) -c $< -o $@
