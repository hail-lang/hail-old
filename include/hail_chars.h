// An iterator of characters for the hail compiler.

#pragma once

#include <hail_str.h>
#include <stdint.h>

/**
 * @brief An iterator of characters in a C string.
 * 
 * Fully supports Unicode.
 */
typedef struct HailChars {
    /**
     * @brief The current index of the iterator.
     */
    uintptr_t idx;

    /**
     * @brief The string to iterate through.
     */
    HailStr* str;
} HailChars;

/**
 * @brief Creates a character iterator from a string.
 * 
 * @param str the string to create an iterator from.
 * @return HailChars the created iterator.
 */
HailChars hail_str_chars(HailStr* str);

/**
 * @brief Gets the next character in the iterator, if any.
 * 
 * Returns `null` (0) if there is no character left in the iterator.
 * 
 * @param chars the iterator.
 * @return wchar_t the character found in the iterator, if any.
 */
uint32_t hail_chars_next(HailChars* chars);