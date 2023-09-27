#include <curl/curl.h>
#include <string.h>
#include <stdlib.h>






struct VideoFile {
    unsigned long totalSize;
    unsigned long thisFileSize;
    unsigned long thisDownloadSize;
    FILE *file;
};
static size_t write_data(void *ptr, size_t size, size_t nmemb, struct VideoFile *vf) {

    size_t written = fwrite(ptr, size, nmemb, vf -> file);
    vf -> thisDownloadSize += written;
    printf("\r%ld", vf -> thisDownloadSize);
    return written;
}

size_t header_callback(char *buffer, size_t size, size_t nitems, void *userdata) {
  // 查找 "Content-Length" 头以获取文件大小
    unsigned long * thisFileSize = (unsigned long *) userdata;
  if(strncmp(buffer, "Content-Length:", 15) == 0) {
    *thisFileSize = strtol(buffer + 16, NULL, 10);
    printf("File size: %ld bytes\n", *thisFileSize);
  }
  return size * nitems;
}

int FetchStream(char* filename, char *streamurl){

    CURL *curl;
    //CURLcode res;
    struct VideoFile vf;
    vf.totalSize = 0;
    vf.thisDownloadSize = 0;
    vf.thisDownloadSize = 0;
    vf.file = NULL;
    const char *outfilename = filename;

    curl = curl_easy_init();

    if(curl) {
        vf.file = fopen(outfilename,"wb");
        if(vf.file == NULL) {
            perror("Error opening file");
            return 1;
        }

        // 设置 CURL 选项
        curl_easy_setopt(curl, CURLOPT_URL, streamurl);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_data);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, &vf);
        curl_easy_setopt(curl, CURLOPT_REFERER, "https://www.bilibili.com");
        curl_easy_setopt(curl, CURLOPT_COOKIEFILE, "cookie_file.txt");
        curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, header_callback);
        curl_easy_setopt(curl, CURLOPT_HEADERDATA, &(vf.thisFileSize));
        // 设置最大重试次数
        //curl_easy_setopt(curl, CURLOPT_MAXCONNECTS, 100L);


         // 设置 User-Agent
        curl_easy_setopt(curl, CURLOPT_USERAGENT, "MyUserAgent/1.0");
        // 执行 HTTP 请求
        char range[50];
        do{
            vf.thisDownloadSize = 0;
            curl_easy_perform(curl);
            vf.totalSize += vf.thisDownloadSize;
            snprintf(range, 50, "%ld-", vf.totalSize);
            curl_easy_setopt(curl, CURLOPT_RANGE, range);

        }while(vf.thisDownloadSize < vf.thisFileSize);

        // 关闭文件和释放资源
        fclose(vf.file);
        curl_easy_cleanup(curl);
    }
    return 0;
}
