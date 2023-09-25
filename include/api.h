/**
 * return cJSON *
 *
**/
#include "cJSON.h"

cJSON * Get(char* apiname, char *paramlist);
char *AddParam(char *paramlist, char *key, char *value);
