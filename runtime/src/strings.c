#include "qi_strings.h"
#include <stdlib.h>
#include <string.h>

size_t qi_strlen(const char* str) {
    return strlen(str);
}

char* qi_strcpy(char* dest, const char* src) {
    return strcpy(dest, src);
}

char* qi_strdup(const char* str) {
    return strdup(str);
}

int qi_strcmp(const char* str1, const char* str2) {
    return strcmp(str1, str2);
}

size_t qi_utf8_strlen(const char* str) {
    // Simplified UTF-8 length calculation
    // A real implementation would handle multi-byte sequences properly
    return strlen(str);
}

int qi_utf8_validate(const char* str) {
    // Simplified UTF-8 validation
    // A real implementation would check for valid UTF-8 sequences
    return str != NULL;
}
