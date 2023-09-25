#include "cjson/cJSON.h"
#include <curl/curl.h>
#include <curl/easy.h>
#include <stddef.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>


#ifdef DEBUG
#define DPRINTF(format,...) \
do {printf("%s::%s "format, __FILE__,__FUNCTION__,##__VA_ARGS__);} while(0)
#else
#define DPRINTF
#endif
char str[1000000] = "";


struct MemoryBlock{
    size_t size;
    char* data;
};


static size_t write_callback(void *contents, size_t size, size_t nmemb, void *stream);
/*
static size_t write_callback(void *contents, size_t size, size_t nmemb, void *stream){
    cJSON **cjson_ptr = (cJSON **)stream;
    *cjson_ptr = cJSON_Parse((char*)contents);
    DPRINTF("%s\n", (char*)contents);
    return size * nmemb;
}
*/
int ptr = 0;
static size_t write_callback(void *contents, size_t size, size_t nmemb, void *userp){
    size_t totalSize = size * nmemb;
    struct MemoryBlock *mem = (struct MemoryBlock*)userp;
    mem -> data = realloc(mem->data, mem->size + totalSize);
    
    memcpy( mem->data + mem->size - 1, contents, totalSize);
    mem->size += totalSize;
    (mem->data)[mem->size - 1] = 0;
    return totalSize;
}


int getVideoBasicInfo(char *aid){
    char *url = "https://api.bilibili.com/x/web-interface/view";
    
    char *urlWithParam = (char *)malloc(strlen(url) + strlen(aid) + 64);
    sprintf(urlWithParam, "%s?bvid=%s",url, aid);



    CURL *handle;
    handle = curl_easy_init();

    cJSON *responseJson=NULL;
    memset(str, 0, sizeof(str));
    curl_easy_setopt(handle, CURLOPT_URL, urlWithParam);
    curl_easy_setopt(handle, CURLOPT_COOKIEFILE, "cookie_file.txt");
    curl_easy_setopt(handle,CURLOPT_WRITEFUNCTION,write_callback); 


    struct MemoryBlock block;
    block.size = 1;
    block.data = (char*)malloc(1);

    curl_easy_setopt(handle, CURLOPT_WRITEDATA, &block); 
    curl_easy_perform(handle);
    //cJSON * code = cJSON_GetObjectItem(responseJson, "message");
    //DPRINTF("%s\n", str);
    responseJson = cJSON_Parse(block.data);
    DPRINTF("%s\n", cJSON_Print(responseJson));
    
    free(block.data);
    block.data = NULL;



    cJSON_Delete(responseJson);
    return 0;
}




int ConvertBvidToCid(char *bvid){
    char* pagelistUrl = "https://api.bilibili.com/x/player/pagelist";

    char* pagelistUrlWithParam = (char*)malloc(strlen(pagelistUrl) + strlen(bvid) + 32);
    sprintf(pagelistUrlWithParam, "%s?bvid=%s",pagelistUrl,bvid); 
    DPRINTF("pagelisturlwithparam: %s\n", pagelistUrlWithParam);




    CURL *handle;
    handle = curl_easy_init();
    curl_easy_setopt(handle, CURLOPT_URL,pagelistUrlWithParam); 

    struct MemoryBlock block;
    block.data = (char*)malloc(1);
    block.size = 1;
    
    curl_easy_setopt(handle, CURLOPT_WRITEFUNCTION,write_callback);
    curl_easy_setopt(handle, CURLOPT_WRITEDATA,&block);
    curl_easy_perform(handle);
    
    cJSON *responseJson = NULL;
    responseJson = cJSON_Parse(block.data);
    DPRINTF("%s\n", cJSON_Print(responseJson));



    return 0;
}






int GetStreamUrl(char *cid){
    char *playurl = "https://api.bilibili.com/x/player/playurl";
    char *playurlWithParam = (char*)malloc(strlen(playurl) + strlen(cid) + 64);
    // 必须有 bvid 的 cid
    sprintf(playurlWithParam, "%s?bvid=%s&cid=%s&fnval=16",playurl,"BV16F411B7Ek",cid); 
    //sprintf(playurlWithParam, "%s?cid=%s",playurl,cid); 
    DPRINTF("playurlWithParam: %s\n", playurlWithParam);
    
    CURL *handle;
    handle = curl_easy_init();
    curl_easy_setopt(handle, CURLOPT_URL, playurlWithParam);
    curl_easy_setopt(handle, CURLOPT_COOKIEFILE, "cookie_file.txt");


    struct MemoryBlock block;
    block.data = (char*)malloc(1);
    block.size = 1;

    
    curl_easy_setopt(handle, CURLOPT_WRITEFUNCTION,write_callback);
    curl_easy_setopt(handle, CURLOPT_WRITEDATA,&block);


    curl_easy_perform(handle);
    cJSON *responseJson;
    responseJson = cJSON_Parse(block.data);
    DPRINTF("%s\n", cJSON_Print(responseJson));
    return 0;
}




cJSON * Get(char* url, char *paramlist){
    CURL *handle;
    handle = curl_easy_init();
    char *url2 = (char*)malloc(strlen(url) + strlen(paramlist) + 1 + 1);
    sprintf(url2, "%s?%s", url, paramlist); 
    
    

    struct MemoryBlock block;
    block.data = NULL;
    block.size = 1;
    curl_easy_setopt(handle, CURLOPT_WRITEFUNCTION, write_callback);
    curl_easy_setopt(handle, CURLOPT_WRITEDATA, &block);
    curl_easy_setopt(handle, CURLOPT_COOKIEFILE, "cookie_file.txt");
    curl_easy_perform(handle);
    free(url2);

    cJSON *responseJson = NULL;
    responseJson = cJSON_Parse(block.data);
    free(block.data);
    block.size = 0;
    block.data= NULL;


    return NULL;
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
