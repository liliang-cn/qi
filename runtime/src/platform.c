#include "qi_runtime.h"
#include <stdio.h>

void qi_runtime_init(void) {
    // Platform-specific initialization
    printf("Qi runtime initialized\n");
}

void qi_runtime_cleanup(void) {
    // Platform-specific cleanup
    printf("Qi runtime cleaned up\n");
}
