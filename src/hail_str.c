#include <hail_chars.h>
#include <hail_str.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

HailStr hail_str_from(uint8_t* src) {
    HailStr str;
    str.ptr = src;
    str.slice = false;
    str.len = strlen(src);
    return str;
}

void hail_str_free(HailStr* str) {
    if (!str->slice) {
        free(str->ptr);
    }
}