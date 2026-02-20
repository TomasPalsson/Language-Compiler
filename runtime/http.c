#include <curl/curl.h>
#include <stdlib.h>
#include <string.h>

struct response_buffer {
    char *data;
    size_t size;
};

static size_t write_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    struct response_buffer *buf = (struct response_buffer *)userp;
    char *ptr = realloc(buf->data, buf->size + realsize + 1);
    if (!ptr) return 0;
    buf->data = ptr;
    memcpy(&(buf->data[buf->size]), contents, realsize);
    buf->size += realsize;
    buf->data[buf->size] = 0;
    return realsize;
}

char *bonk_http_fetch(const char *method, const char *url, const char *body) {
    CURL *curl = curl_easy_init();
    if (!curl) {
        char *err = malloc(1);
        err[0] = 0;
        return err;
    }

    struct response_buffer buf;
    buf.data = malloc(1);
    buf.data[0] = 0;
    buf.size = 0;

    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &buf);
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, method);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);

    if (body && *body) {
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, body);
    }

    curl_easy_perform(curl);
    curl_easy_cleanup(curl);

    return buf.data;
}
