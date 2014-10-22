#ifndef __CIRCUIT_H__
#define __CIRCUIT_H__

#include <util.h>
#include <flp.h>
#include <temperature.h>
#include <temperature_block.h>

typedef struct {
	size_t cores;
	size_t nodes;
	double *capacitance;
	double *conductance;
} Circuit;

Circuit *new_circuit(const char *floorplan, const char *config, const char *params);
void free_circuit(Circuit *circuit);

#endif
