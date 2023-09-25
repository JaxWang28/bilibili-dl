#include "cJSON.h"
#include <stdio.h>
#include <string.h>
#include "api.h"
extern cJSON* urlArr;

cJSON *bvidArr;
int ParseUrlArr(){
    printf("%s\n", cJSON_Print(urlArr));
    int array_size = cJSON_GetArraySize(urlArr);
    
    // bvid 为 12 个字符
    char bvid[13];
    char *baseurl = "https://www.bilibili.com/video/";
    int baseurlLen = strlen(baseurl);
    for (int i = 0; i < array_size; i++){
        cJSON *url = cJSON_GetArrayItem(urlArr, i);
        if (strncmp(baseurl, url->valuestring, baseurlLen) != 0){
            printf("URL ERROR: %s\n", url -> valuestring);
            continue;
        }
        strncpy(bvid, url -> valuestring + baseurlLen ,12);
        bvid[12] = 0;
        cJSON *bvidJson = cJSON_CreateString(bvid);
        cJSON_AddItemToArray(bvidArr, bvidJson);
    }
    printf("%s\n", cJSON_Print(bvidArr));
    return 0;
}


// get cid at the same time
int PrintVideoInfo(char *bvid){
    char *paramlist = NULL;
    // realloc()
    paramlist = AddParam(paramlist, "bvid", bvid);
    printf("%s\n", paramlist);
    cJSON *responseJson = Get("videoInfo", paramlist);
    //printf("%s\n", cJSON_Print(responseJson));
    cJSON *dataJson = cJSON_GetObjectItem(responseJson, "data");
    cJSON *titleJson = cJSON_GetObjectItem(dataJson, "title");

    printf("title: %s\n", titleJson -> valuestring);

    return 0;
};




int Download(){
    bvidArr = cJSON_CreateArray();
    ParseUrlArr();
    cJSON *bvid = cJSON_GetArrayItem(bvidArr, 0);
    PrintVideoInfo(bvid -> valuestring);
    return 0;
}



