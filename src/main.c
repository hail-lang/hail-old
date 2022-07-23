#include <fcntl.h>
#include <hail_chars.h>
#include <hail_io.h>
#include <hail_str.h>
#include <io.h>
#include <locale.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

int main(void) {
    FILE* file = fopen("test.hl", "r");
    fseek(file, 0, SEEK_END);
    uintptr_t len = ftell(file) + 1;
    fseek(file, 0, SEEK_SET);

    char* buf = malloc(len);
    fgets(buf, len, file);

    println(hail_str_from(buf));

    fclose(file);

    /*
    // Create a hail string from the test file.
    HailStr str = hail_str_from(buf);
    HailChars chars = hail_str_chars(&str);
    
    // Print the string.
    uint32_t c = hail_chars_next(&chars);
    while (c != 0) {
        wchar_t str[] = {c, L'\0'};
        wprintf(str);
        c = hail_chars_next(&chars);
    }
    */

    /*
    HailStr str = hail_str_from("Hello, world! 😊 \n");
    HailChars chars = hail_str_chars(&str);

    wchar_t c = hail_chars_next(&chars);
    while (c != 0) {
        printf("%lc", c);
        c = hail_chars_next(&chars);
    }
    */
}