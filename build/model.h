#ifndef __MODEL_H__
#define __MODEL_H__

#include <util.h>
#include <flp.h>
#include <temperature.h>
#include <temperature_block.h>

typedef struct {
	size_t cores;
	size_t nodes;
	double *capacitance;
	double *conductance;
} Model;

Model *new_model(const char *floorplan, const char *config, const char *params);
void free_model(Model *model);

#endif
