// Better strings for hail.

#pragma once

#include <stdbool.h>
#include <stdint.h>

/**
 * @brief A string used by the hail compiler.
 */
typedef struct HailStr {
    /**
     * @brief The C string that this string wraps.
     */
    uint8_t* ptr;

    /**
     * @brief Whether or not this string is a slice from another string.
     * 
     * If a string is a slice, the memory it wraps cannot be deallocated.
     */
    bool slice;

    /**
     * @brief The length of the string.
     */
    uintptr_t len;
} HailStr;

/**
 * @brief Creates a string from a C string.
 * 
 * @param str the C string to wrap.
 * @return HailStr the created string.
 */
HailStr hail_str_from(uint8_t* str);

/**
 * @brief Deallocates a string.
 * 
 * @param str the string to deallocate.
 */
void hail_str_free(HailStr* str);