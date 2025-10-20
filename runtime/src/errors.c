#include "qi_errors.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static qi_error_code_t last_error = QI_ERROR_NONE;
static char error_message[256] = {0};

void qi_set_error(qi_error_code_t code, const char* message) {
    last_error = code;
    if (message) {
        strncpy(error_message, message, sizeof(error_message) - 1);
        error_message[sizeof(error_message) - 1] = '\0';
    } else {
        error_message[0] = '\0';
    }
}

qi_error_code_t qi_get_last_error(void) {
    return last_error;
}

const char* qi_get_error_message(void) {
    return error_message;
}

void qi_panic(const char* message) {
    if (message) {
        fprintf(stderr, "Qi runtime panic: %s\n", message);
    } else {
        fprintf(stderr, "Qi runtime panic: Unknown error\n");
    }
    abort();
}
