#include "qi_memory.h"
#include <stdlib.h>
#include <string.h>

static size_t total_allocated = 0;

void* qi_malloc(size_t size) {
    void* ptr = malloc(size);
    if (ptr) {
        total_allocated += size;
    }
    return ptr;
}

void* qi_realloc(void* ptr, size_t size) {
    // Note: This is a simplified implementation
    // A real implementation would track the old size
    void* new_ptr = realloc(ptr, size);
    return new_ptr;
}

void qi_free(void* ptr) {
    free(ptr);
}

size_t qi_get_allocated_memory(void) {
    return total_allocated;
}

void qi_reset_memory_stats(void) {
    total_allocated = 0;
}
