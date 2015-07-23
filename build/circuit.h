#ifndef __CIRCUIT_H__
#define __CIRCUIT_H__

#include <util.h>
#include <flp.h>
#include <temperature.h>
#include <temperature_block.h>

typedef struct {
	size_t units;
	size_t nodes;
	double *capacitance;
	double *conductance;
} Circuit;

Circuit *new_Circuit(const char *, const char *);
void drop_Circuit(Circuit *);

#endif
