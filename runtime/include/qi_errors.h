#ifndef QI_ERRORS_H
#define QI_ERRORS_H

#include <stdarg.h>

// Error codes
typedef enum {
    QI_ERROR_NONE = 0,
    QI_ERROR_OUT_OF_MEMORY,
    QI_ERROR_INVALID_ARGUMENT,
    QI_ERROR_DIVISION_BY_ZERO,
    QI_ERROR_INDEX_OUT_OF_BOUNDS,
    QI_ERROR_STACK_OVERFLOW,
    QI_ERROR_UNDEFINED,
} qi_error_code_t;

// Error handling
void qi_set_error(qi_error_code_t code, const char* message);
qi_error_code_t qi_get_last_error(void);
const char* qi_get_error_message(void);

// Panic handling
void qi_panic(const char* message) __attribute__((noreturn));

#endif // QI_ERRORS_H
