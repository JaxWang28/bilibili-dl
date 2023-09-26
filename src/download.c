#include "cJSON.h"
#include <stdio.h>
#include <string.h>
#include "api.h"

extern cJSON *submissionArr;
int getSubmissionInfo(cJSON *submission, char *bvid);
int ParseSubmissionArr(){
    //printf("%s\n", cJSON_Print(submissionArr));
    int array_size = cJSON_GetArraySize(submissionArr);
    
    // bvid 为 12 个字符
    char bvid[13];
    char *baseurl = "https://www.bilibili.com/video/";
    int baseurlLen = strlen(baseurl);
    for (int i = 0; i < array_size; i++){
        cJSON *submission = cJSON_GetArrayItem(submissionArr, i);
        cJSON *url = cJSON_GetObjectItem(submission, "url");

        if (strncmp(baseurl, url->valuestring, baseurlLen) != 0){
            printf("URL ERROR: %s\n", url -> valuestring);
            // 删除这一项
            // 删除同时将下标 减一
            cJSON_DeleteItemFromArray(submissionArr, i--);
            continue;
        }
        strncpy(bvid, url -> valuestring + baseurlLen ,12);
        bvid[12] = 0;
        cJSON *bvidJson = cJSON_GetObjectItem(submission, "bvid");
        cJSON_SetValuestring(bvidJson, bvid);
        
        getSubmissionInfo(submission, bvid);
    }
    printf("%s\n", cJSON_Print(submissionArr));
    return 0;
}

int getSubmissionInfo(cJSON *submission, char *bvid){
    char *paramlist = NULL;
    // realloc()
    paramlist = AddParam(paramlist, "bvid", bvid);
    //printf("%s\n", paramlist);
    cJSON *responseJson = Get("videoInfo", paramlist);

    cJSON *dataJson = cJSON_GetObjectItem(responseJson, "data");
    //printf("%s\n", cJSON_Print(dataJson));
    cJSON *responseTitle = cJSON_GetObjectItem(dataJson, "title");
    cJSON *title = cJSON_GetObjectItem(submission, "title");
    cJSON_SetValuestring(title, responseTitle -> valuestring);
    
    cJSON *pagefrom = cJSON_GetObjectItem(submission, "pagefrom");
    cJSON *pageend = cJSON_GetObjectItem(submission, "pageend");
    cJSON *pages = cJSON_GetObjectItem(submission, "pages");
    for (int i = pagefrom -> valueint - 1; i < pageend -> valueint; i++){
        cJSON *responsePages = NULL;
        responsePages = cJSON_GetObjectItem(dataJson, "pages");

        cJSON *responsePage = NULL;
        responsePage = cJSON_GetArrayItem(responsePages, i);
        if (responsePage == NULL){
            break;
        }
        cJSON *page = cJSON_Duplicate(responsePage, 1);
        cJSON_AddItemToArray(pages, page);
    }
    cJSON_Delete(responseJson);
    return 0;
}



// get cid at the same time
int PrintVideoInfo(char *bvid){
    return 0;
};




int Download(){
    //bvidArr = cJSON_CreateArray();
    ParseSubmissionArr();
    //cJSON *bvid = cJSON_GetArrayItem(bvidArr, 0);
    //PrintVideoInfo(bvid -> valuestring);
    return 0;
}



