#include <curl/curl.h>
#include <curl/easy.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>
#include "cJSON.h"
#include <stdlib.h>
#include "options.h"
#include "api.h"

#ifdef DEBUG
#define DPRINTF(format,...) \
do {printf("%s::%s "format, __FILE__,__FUNCTION__,##__VA_ARGS__);} while(0)
#else
#define DPRINTF
#endif



#define STREQ(x, y) (strcmp(x, y) == 0)
extern struct Options options;
static size_t write_callback(void *contents, size_t size, size_t nmemb, void *stream);
static int LoginByQrCode();

static void PrintLoginStatus();
/**
 * API to login
 *
 **/

int Login(){
    switch (options.loginMethod) {
        case QRCODE:
            LoginByQrCode();
            break;
        case SMSCODE:
            DPRINTF("login by SMScode\n");
            break;
        case PASSWORD:
            DPRINTF("login by password\n");
            break;
        default:
            break;
    }
    PrintLoginStatus(); 
    return 0;
}

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

static int LoginByQrCode(){
    CURL *handle;
    // 保存 cookie
    handle = curl_easy_init();
    curl_easy_setopt(handle, CURLOPT_COOKIEJAR, "./cookie_file.txt"); 

    struct MemoryBlock block;
    block.data = (char*)malloc(1);
    block.size = 1;

    char *generateUrl = "https://passport.bilibili.com/x/passport-login/web/qrcode/generate";
    curl_easy_setopt(handle, CURLOPT_URL, generateUrl);
    curl_easy_setopt(handle, CURLOPT_WRITEFUNCTION, write_callback); 
    curl_easy_setopt(handle, CURLOPT_WRITEDATA, &block); 
    curl_easy_perform(handle);

    cJSON *responseJson = cJSON_Parse(block.data);
    free(block.data);
    block.size = 0;
    cJSON *dataJson = cJSON_GetObjectItem(responseJson, "data");

    cJSON *dataUrlJson = cJSON_GetObjectItem(dataJson, "url");
    char *qrcodeUrl = strdup(dataUrlJson -> valuestring);

    cJSON *dataQrCodeKeyJson= cJSON_GetObjectItem(dataJson, "qrcode_key");
    char *qrcodeKey = strdup(dataQrCodeKeyJson -> valuestring);

    cJSON_Delete(responseJson);
    responseJson = NULL;

    char ch;
    printf(
        "Copy the qrcode url to any qrcode generate website, such as https://cli.im/url.\n"
        "Then scan the qrcode to login.\n"
        "QR code url: %s\n", qrcodeUrl
    );
    free(qrcodeUrl);
    qrcodeUrl = NULL;
    printf("please 'y' to continue:\n");
    do {
        ch = getchar();
    } while (ch != 'y' && ch != 'Y');

    char *pollUrl = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll"; 
    char *pollUrlWithParam = (char *)malloc(strlen(pollUrl) + strlen(qrcodeKey) + 32);
    sprintf(pollUrlWithParam, "%s?qrcode_key=%s",pollUrl, qrcodeKey); 
    free(qrcodeKey);
    qrcodeKey = NULL;
    curl_easy_setopt(handle, CURLOPT_URL, pollUrlWithParam);
    curl_easy_setopt(handle, CURLOPT_WRITEFUNCTION, write_callback); 

    block.data = malloc(1);
    block.size = 1;
    curl_easy_setopt(handle, CURLOPT_WRITEDATA, &block); 
    curl_easy_perform(handle);

    responseJson = cJSON_Parse(block.data);
    free(block.data);
    block.size = 0;

    DPRINTF("responseJson: %s\n", cJSON_Print(responseJson));
    cJSON_Delete(responseJson);
    // 保存 cookie
    curl_easy_cleanup(handle);
    return 0;
}
static void PrintLoginStatus(){
    cJSON *responseJson = Get("loginInfo", NULL);
    if (responseJson == NULL){
        printf("responseJson == NULL\n");
        return;
    }
    cJSON *dataJson = cJSON_GetObjectItem(responseJson, "data");
    
    cJSON *tmpJson = cJSON_GetObjectItem(dataJson, "isLogin");
    

    // true cJSON 自动进行转换
    if ( !(tmpJson -> valueint)){
        printf(
            "Login Status: NO\n"     
        );
        cJSON_Delete(responseJson);
        return;
    }
    tmpJson = cJSON_GetObjectItem(dataJson, "uname");
    printf("用户昵称:   %s\n",cJSON_Print(tmpJson));


    tmpJson = cJSON_GetObjectItem(dataJson,"vipStatus");

    if(tmpJson -> valueint == 1){
        tmpJson = cJSON_GetObjectItem(dataJson,"vipType");
        switch (tmpJson -> valueint) {
            case 0:
                printf("vip类型:    无\n");
                break;
            case 1:
                printf("vip类型:    月度大会员\n");
                break;
            case 2:
                printf("vip类型:    年度大会员\n");
                break;
        }
    }
    else{
        printf("vip类型:    无\n");
    }
    cJSON_Delete(responseJson);

}

