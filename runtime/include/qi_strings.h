#ifndef QI_STRINGS_H
#define QI_STRINGS_H

#include <stddef.h>

// String operations
size_t qi_strlen(const char* str);
char* qi_strcpy(char* dest, const char* src);
char* qi_strdup(const char* str);
int qi_strcmp(const char* str1, const char* str2);

// Unicode string operations
size_t qi_utf8_strlen(const char* str);
int qi_utf8_validate(const char* str);

#endif // QI_STRINGS_H
