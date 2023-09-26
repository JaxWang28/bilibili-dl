/***************************************************************************
*
*     Project                 ____  ____  ____  _     
*                            | __ )| __ )|  _ \| |    
*                            |  _ \|  _ \| | | | |    
*                            | |_) | |_) | |_| | |___ 
*                            |____/|____/|____/|_____|
*
*
*
***************************************************************************/
#define PROGNAME "BilibiliDownload"
#define VERSION "v0.0.0"


#include <stdio.h>
#include <string.h>
#include "login.h"
#include <stdlib.h>
#include "options.h"
#include "cJSON.h"
#include "download.h"

#ifdef DEBUG
#define DPRINTF(format,...) \
do {printf("%s::%s "format, __FILE__,__FUNCTION__,##__VA_ARGS__);} while(0)
#else
#define DPRINTF
#endif

#define STREQ(x, y) (strcmp(x, y) == 0)



// default
enum Cmd cmd = UNUSED;
struct Options options = {
    .version = false,
    .help = false,
    .loginMethod = QRCODE,
}; 

// 稿件列表
cJSON *submissionArr = NULL;

/* [{url: "", bvid:"",avid:"", title:"", pagefrom: int, pageend: int, pagelist:[], {}, {}, {}]
*/

static int ParsePageRange(char *pagerangecmd, int *pagefrom, int *pageend);
static int ParasCmdOpt(int argc, char *argv[]);
static void PrintVersion();
static void PrintHelp();
int main(int argc, char* argv[])
{
    submissionArr = cJSON_CreateArray();
    if (submissionArr == NULL){
        return -1;
    }

    if (argc == 1){
        options.help = true;
    }
    else{
        if(ParasCmdOpt(argc, argv) != 0){
            exit(1);
        };
    }
    while(1){
        switch (cmd) {
            case UNUSED:
                // if opt not --version --help  continue ->  Download
                if(options.version){
                    //DPRINTF("cmd: Unused opt: version\n");
                    PrintVersion();

                }    
                else if(options.help){
                    PrintHelp();
                    //DPRINTF("cmd: Unused opt: help\n");
                }
                else {
                    cmd = DOWNLOAD;
                    continue;
                }
                break;
            case LOGIN:
                Login();
                break;
            case DOWNLOAD:
                printf("此功能还没有实现，可以在 issue 中催催作者.\n");
                Download();
                break;
            default:
                DPRINTF("cmd: error\n");
                exit(1);
        } 
        break;
    }
    cJSON_Delete(submissionArr);
    return 0;
}

static int ParasCmdOpt(int argc, char *argv[]){
    char *templateStr = "{ \
                            \"url\": \"\"   , \
                            \"bvid\": \"\"  , \
                            \"avid\": \"\"  , \
                            \"title\": \"\" , \
                            \"pagefrom\": 0 , \
                            \"pageend\": 0  , \
                            \"pages\": []  \
                            }";

    cJSON *template = cJSON_Parse(templateStr);
    //printf("%s\n",cJSON_Print(template));
    
    // argv[0] is the program name
    argc--;
    argv++;
     
    while(argc--){
        // opt
        if( argv[0][0] == '-' ){
            // option
            // double --
            if(STREQ(argv[0], "-v") || STREQ(argv[0], "--version")){
                options.version = true;
            }
            else if(STREQ(argv[0], "-h") || STREQ(argv[0], "--help")){
                options.help = true;

            }

            else if(STREQ(argv[0], "--qrcode")){
                options.loginMethod = QRCODE;
            }
            else if(STREQ(argv[0], "--smscode")){
                options.loginMethod = SMSCODE;
            }
            else if(STREQ(argv[0], "--password")){
                options.loginMethod = PASSWORD;
            }


            else{
                // printf error
                printf("Unknow option: %s\n", argv[0]);
                printf(
                    "Try 'bbdl -h' for more informantion.\n"
                );
                cJSON_Delete(template);
                return -1;
            }
            
        }

        // cmd
        else if(STREQ(argv[0], "login") || STREQ(argv[0], "Login")){
            cmd = LOGIN;
        }

        else if(STREQ(argv[0], "download") || STREQ(argv[0], "Download")){
            cmd = DOWNLOAD;
        }

        // url
        else{
            //cJSON *url = cJSON_CreateString(argv[0]);
            cJSON *submission = cJSON_Duplicate(template, 1);
            cJSON *url = cJSON_GetObjectItem(submission, "url");
            cJSON_SetValuestring(url, argv[0]);
            
            // 是否指定 page
            if(argc > 0 && argv[1][0] == 'p'){
                int tmp_from, tmp_end;
                ParsePageRange(argv[1], &tmp_from, &tmp_end);
                cJSON *tmp = NULL;
                tmp = cJSON_GetObjectItem(submission, "pagefrom");
                cJSON_SetIntValue(tmp, tmp_from);
                tmp = cJSON_GetObjectItem(submission, "pageend");
                cJSON_SetIntValue(tmp, tmp_end);
                argc --;
                argv ++;
            }
            cJSON_AddItemToArray(submissionArr, submission);
        }

        argv++;
    }
    cJSON_Delete(template);
    return 0;
}


static int ParsePageRange(char *pagerangecmd, int *pagefrom, int *pageend){
    // 目前只支持指定一种page 如， p10
    
    // 去掉 p
    char *pagerange = pagerangecmd + 1;

    *pagefrom = *pageend = atoi(pagerange);
    // 缺少错误检查
    return 0;
}

static void PrintHelp(){
    printf(
        "Usage: " "bbdl" " [command] [options] <URL> [pxxx]\n"
        "\n"
        "options:\n"
        "  -v, --version        print program version\n"
        "  -h, --help           give this help list\n"
        "  --qrcode             login by QR Code\n"
        "  --smscode            login by SMS Code\n"
        "  --password           login by password\n"
        "\n"
        "commands:\n"
        "  login                login in account\n"
        "  download             download,default\n"
    );
}

static void PrintVersion(){
    printf(
        PROGNAME " " VERSION "\n"
        "See more informantion https://github.com/jw-jackson/bilibili-dl.\n"
    );
}

