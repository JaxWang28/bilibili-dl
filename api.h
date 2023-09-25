/**
 * return cJSON *
 *
**/
#include "cjson/cJSON.h"

cJSON * Get(char* apiname, char *paramlist);
char *AddParam(char *paramlist, char *key, char *value);
