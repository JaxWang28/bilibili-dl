#include "cJSON.h"
#include <stdio.h>
#include <string.h>
#include "api.h"
#include <stdlib.h>
#include <curl/curl.h>
#include "fetchstream.h"

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

static int GetVideoStream(char *bvid,cJSON *page);





int Download(){
    //bvidArr = cJSON_CreateArray();
    ParseSubmissionArr();
    //cJSON *bvid = cJSON_GetArrayItem(bvidArr, 0);
    //PrintVideoInfo(bvid -> valuestring);
    int submissionArrSize = cJSON_GetArraySize(submissionArr);

    for(int i = 0; i < submissionArrSize; i++){
        cJSON *submission = cJSON_GetArrayItem(submissionArr, i);
        cJSON *pages = cJSON_GetObjectItem(submission, "pages");
        cJSON *bvid = cJSON_GetObjectItem(submission, "bvid");
        int pagesSize = cJSON_GetArraySize(pages);
        for(int j = 0; j < pagesSize; j++){
            cJSON *page = cJSON_GetArrayItem(pages, j);
            GetVideoStream(bvid -> valuestring, page);
        }


    }

    return 0;
}







static int GetVideoStream(char *bvid,cJSON *page){
    printf("%s\n", cJSON_Print(page));
    cJSON *cid = cJSON_GetObjectItem(page,"cid");
    char *paramlist = NULL;
    printf("%d\n", cid -> valueint);
    char cidStr[10];
    sprintf(cidStr, "%d", cid ->valueint);
    paramlist = AddParam(paramlist, "fnval", "16");
    paramlist = AddParam(paramlist, "cid", cidStr);
    paramlist = AddParam(paramlist, "bvid", bvid);
    printf("%s\n", paramlist);    
    cJSON *responseJson = Get("videoStream", paramlist);

    cJSON *dataJson = cJSON_GetObjectItem(responseJson, "data");
    cJSON *dash = cJSON_GetObjectItem(dataJson, "dash");


    cJSON *video = cJSON_GetObjectItem(dash, "video");



    printf("%s\n", cJSON_Print(video));
    cJSON *video1 = cJSON_GetArrayItem(video, 0);
    //printf("%s\n", cJSON_Print(video1));
    cJSON *baseUrl = cJSON_GetObjectItem(video1, "baseUrl");
    
    FetchStream("test1.mp4", baseUrl->valuestring);



    cJSON_Delete(responseJson);
    free(paramlist);
    return 0;
}




