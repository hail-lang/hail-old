#include <hail_win32.h>
#include <hail_chars.h>
#include <hail_io.h>
#include <hail_str.h>
#include <stdint.h>
#include <stdio.h>

#ifdef WINDOWS
#include <windows.h>
#endif

void utf32_to_utf8_string(uint32_t code, uint8_t* string) {
	if (code < 0x80) {
        string[0] = code;
        string[1] = 0;
    } else if (code < 0x800) {   // 00000yyy yyxxxxxx
		string[0] = (0b11000000 | (code >> 6));
		string[1] = (0b10000000 | (code & 0x3f));
        string[2] = 0;
	} else if (code < 0x10000) {  // zzzzyyyy yyxxxxxx
		string[0] = (0b11100000 | (code >> 12));         // 1110zzz
		string[1] = (0b10000000 | ((code >> 6) & 0x3f)); // 10yyyyy
		string[2] = (0b10000000 | (code & 0x3f));        // 10xxxxx
        string[3] = 0;
	} else if (code < 0x200000) { // 000uuuuu zzzzyyyy yyxxxxxx
		string[0] = (0b11110000 | (code >> 18));          // 11110uuu
		string[1] = (0b10000000 | ((code >> 12) & 0x3f)); // 10uuzzzz
		string[2] = (0b10000000 | ((code >> 6)  & 0x3f)); // 10yyyyyy
		string[3] = (0b10000000 | (code & 0x3f));         // 10xxxxxx
        string[4] = 0;
	}
}

void write_char(uint32_t c) {
    #ifdef WINDOWS
    // if this isn't done, this will break unicode characters (such as emoji) on windows.
    SetConsoleOutputCP(CP_UTF8);
    #endif
    uint8_t buf[5];
    utf32_to_utf8_string(c, buf);
    fprintf(stdout, buf);
}

void print_char(uint32_t c) {
    #ifdef WINDOWS
    // if this isn't done, this will break unicode characters (such as emoji) on windows.
    SetConsoleOutputCP(CP_UTF8);
    #endif

    uint8_t buf[5];
    utf32_to_utf8_string(c, buf);
    printf(buf);
}

void println(HailStr str) {
    HailChars chars = hail_str_chars(&str);
    
    // Print the string.
    uint32_t c = hail_chars_next(&chars);
    while (c != 0) {
        write_char(c);
        c = hail_chars_next(&chars);
    }

    putchar('\n');
}