#include <hail_chars.h>
#include <hail_str.h>
#include <stdint.h>
#include <stdio.h>

static const uint32_t UTF8_ONE_BYTE_MASK = 0x80;
static const uint32_t UTF8_ONE_BYTE_BITS = 0;
static const uint32_t UTF8_TWO_BYTES_MASK = 0xE0;
static const uint32_t UTF8_TWO_BYTES_BITS = 0xC0;
static const uint32_t UTF8_THREE_BYTES_MASK = 0xF0;
static const uint32_t UTF8_THREE_BYTES_BITS = 0xE0;
static const uint32_t UTF8_FOUR_BYTES_MASK = 0xF8;
static const uint32_t UTF8_FOUR_BYTES_BITS = 0xF0;
static const uint32_t UTF8_CONTINUATION_MASK = 0xC0;
static const uint32_t UTF8_CONTINUATION_BITS = 0x80;

uintptr_t utf8_codepoint_size(const uint8_t byte) {
	if ((byte & UTF8_ONE_BYTE_MASK) == UTF8_ONE_BYTE_BITS) {
		return 1;
	}

	if ((byte & UTF8_TWO_BYTES_MASK) == UTF8_TWO_BYTES_BITS) {
		return 2;
	}

	if ((byte & UTF8_THREE_BYTES_MASK) == UTF8_THREE_BYTES_BITS) {
		return 3;
	}

	if ((byte & UTF8_FOUR_BYTES_MASK) == UTF8_FOUR_BYTES_BITS) {
		return 4;
	}

	return 0;
}

HailChars hail_str_chars(HailStr* str) {
    HailChars chars;
    chars.idx = 0;
    chars.str = str;
    return chars;
}

uint32_t hail_chars_next(HailChars* chars) {
    if (chars->idx >= chars->str->len)
        return 0;

    uintptr_t cp_size = utf8_codepoint_size(chars->str->ptr[chars->idx]);
    uint32_t buffer;

	switch (cp_size) {
		case 1:
			buffer = ((uint32_t) chars->str->ptr[chars->idx] & ~UTF8_ONE_BYTE_MASK);
			break;
		case 2:
			buffer =
				((uint32_t) chars->str->ptr[chars->idx] & ~UTF8_TWO_BYTES_MASK) << 6 |
				((uint32_t) chars->str->ptr[chars->idx + 1] & ~UTF8_CONTINUATION_MASK);
			break;
		case 3:
			buffer =
				((uint32_t) chars->str->ptr[chars->idx] & ~UTF8_THREE_BYTES_MASK) << 12 |
				((uint32_t) chars->str->ptr[chars->idx + 1] & ~UTF8_CONTINUATION_MASK) << 6 |
				((uint32_t) chars->str->ptr[chars->idx + 2] & ~UTF8_CONTINUATION_MASK);

			break;
		case 4:
			buffer =
				((uint32_t) chars->str->ptr[chars->idx] & ~UTF8_FOUR_BYTES_MASK) << 18 |
				((uint32_t) chars->str->ptr[chars->idx + 1] & ~UTF8_CONTINUATION_MASK) << 12 |
				((uint32_t) chars->str->ptr[chars->idx + 2] & ~UTF8_CONTINUATION_MASK) << 6 |
				((uint32_t) chars->str->ptr[chars->idx + 3] & ~UTF8_CONTINUATION_MASK);

			break;
		default:
			// this should never happen, since we check validity of the string
			printf("utf8_to_utf32: invalid byte in UTF8 string.\n");
			return 0;
	}

	chars->idx += cp_size;
    return buffer;
}