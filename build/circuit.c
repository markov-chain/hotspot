#include <stdlib.h>
#include <string.h>
#include "circuit.h"

#define WHITESPACE " \t"

size_t parse_params(str_pair *table, size_t max, const char *params) {
	char first = 1;
	size_t count = 0;

	char *p = strtok((char *)params, WHITESPACE);
	while (p != NULL && count < max) {
		if (first) strcpy(table[count].value, p);
		else strcpy(table[count++].name, p);
		first = !first;
	}

	return count;
}

Circuit *new_circuit(const char *floorplan, const char *config, const char *params) {
	thermal_config_t thermal_config = default_thermal_config();

	str_pair *table = (str_pair *)malloc(sizeof(str_pair) * MAX_ENTRIES);
	if (!table) goto err_malloc_str_pair;

	if (config && strlen(config) > 0) {
		size_t count = read_str_pairs(table, MAX_ENTRIES, (char *)config);
		thermal_config_add_from_strs(&thermal_config, table, count);
	}

	if (params && strlen(params) > 0) {
		size_t count = parse_params(table, MAX_ENTRIES, params);
		thermal_config_add_from_strs(&thermal_config, table, count);
	}

	free(table);

	flp_t *flp = read_flp((char *)floorplan, FALSE);
	if (!flp) goto err_read_flp;

	RC_model_t *RC_model = alloc_RC_model(&thermal_config, flp);
	if (!RC_model) goto err_alloc_RC_model;

	populate_R_model(RC_model, flp);
	populate_C_model(RC_model, flp);

	Circuit *circuit = (Circuit *)malloc(sizeof(Circuit));
	if (!circuit) goto err_malloc_HotSpot;

	circuit->cores = flp->n_units;
	circuit->nodes = RC_model->block->n_nodes;

	size_t i, j, nodes = circuit->nodes;

	circuit->capacitance = (double *)malloc(nodes * sizeof(double));
	if (!circuit->capacitance) goto err_malloc_capacitance;

	for (i = 0; i < nodes; i++)
		circuit->capacitance[i] = RC_model->block->a[i];

	circuit->conductance = (double *)malloc(nodes * nodes * sizeof(double));
	if (!circuit->conductance) goto err_malloc_conductance;

	for (i = 0; i < nodes; i++)
		for (j = 0; j < nodes; j++)
			circuit->conductance[i * nodes + j] = RC_model->block->b[i][j];

	delete_RC_model(RC_model);
	free_flp(flp, FALSE);

	return circuit;

err_malloc_conductance:
	free(circuit->capacitance);

err_malloc_capacitance:
	free(circuit);

err_malloc_HotSpot:
	delete_RC_model(RC_model);

err_alloc_RC_model:
	free_flp(flp, FALSE);

err_read_flp:
err_malloc_str_pair:

	return NULL;
}

void free_circuit(Circuit *circuit) {
	free(circuit->capacitance);
	free(circuit->conductance);
	free(circuit);
}
