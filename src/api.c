#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <curl/curl.h>
#include "cJSON.h"
static char *GetApiUrl(char *apiname);


#ifdef TEST  
int main(){
    char *apiname = "test";
    char *url = GetApiUrl(apiname);
    if(url == NULL){
        printf("NULL\n");
        return 0;
    }
    printf("url: %s\n", url);


    

    return 0;
}
#endif


#ifdef DEBUG
#define DPRINTF(format,...) \
do {printf("%s::%s "format, __FILE__,__FUNCTION__,##__VA_ARGS__);} while(0)
#else
#define DPRINTF
#endif



extern char* apiTable;
cJSON* apiJson = NULL;

struct MemoryBlock{
    size_t size;
    char* data;
};

static size_t write_callback(void *contents, size_t size, size_t nmemb, void *userp){
    size_t totalSize = size * nmemb;
    struct MemoryBlock *mem = (struct MemoryBlock*)userp;
    mem -> data = realloc(mem->data, mem->size + totalSize);
    
    memcpy( mem->data + mem->size - 1, contents, totalSize);
    mem->size += totalSize;
    (mem->data)[mem->size - 1] = 0;
    return totalSize;
}



/**
* 构建发送 Get 请求时参数列表
**/
char *AddParam(char *paramlist, char *key, char *value){
    if(paramlist == NULL){
        paramlist = (char*)malloc(strlen(key) + strlen(value) + 1 + 1);
        sprintf(paramlist, "%s=%s", key, value);
    }
    else{
        paramlist = (char*)realloc(paramlist, (strlen(paramlist) + strlen(key) + strlen(value) + 1 + 1 + 1));
        sprintf(paramlist, "%s&%s=%s", paramlist, key, value);
    }
    return paramlist;
}



cJSON * Get(char* apiname, char *paramlist){
    CURL *handle;
    handle = curl_easy_init();
    char* url = NULL;
    url = GetApiUrl(apiname);
    if (url == NULL){
        printf("url == NULL\n");
        exit(0);
        return NULL;
    }
    if (paramlist != NULL){
        url = (char*) realloc(url, strlen(url) + strlen(paramlist) + 1 + 1);
        sprintf(url, "%s?%s", url, paramlist); 
    }
    DPRINTF("%s\n", url);
    
    struct MemoryBlock block;
    block.data = (char*)malloc(1);
    block.size = 1;
    curl_easy_setopt(handle, CURLOPT_URL, url);
    curl_easy_setopt(handle, CURLOPT_WRITEFUNCTION, write_callback);
    curl_easy_setopt(handle, CURLOPT_WRITEDATA, &block);
    curl_easy_setopt(handle, CURLOPT_COOKIEFILE, "cookie_file.txt");
    curl_easy_perform(handle);
    free(url);

    cJSON *responseJson = NULL;
    responseJson = cJSON_Parse(block.data);
    free(block.data);
    block.size = 0;
    block.data= NULL;

    return responseJson;
}




static char *GetApiUrl(char *apiname){

    if (apiJson == NULL){
        apiJson = cJSON_Parse(apiTable);
        if (apiJson == NULL){
            printf("Parse apiTable error\n");
            exit(0);
        }
    }
    cJSON *selectedApiJson = cJSON_GetObjectItem(apiJson, apiname);
    if(selectedApiJson == NULL){
        // error
        return NULL;
    }

    char *url = NULL;
    url = strdup(selectedApiJson->valuestring);
    return url;
}
