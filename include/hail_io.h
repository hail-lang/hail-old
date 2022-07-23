// IO tools with Unicode support for the hail compiler.

#pragma once

#include <hail_str.h>
#include <stdint.h>

/**
 * @brief Prints a UTF-32 character to the terminal.
 * 
 * @param c the character to print.
 */
void print_char(uint32_t c);

/**
 * @brief Prints a line to the terminal.
 * 
 * @param str the string to print.
 */
void println(HailStr str);