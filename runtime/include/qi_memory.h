#ifndef QI_MEMORY_H
#define QI_MEMORY_H

#include <stddef.h>

// Memory allocation
void* qi_malloc(size_t size);
void* qi_realloc(void* ptr, size_t size);
void qi_free(void* ptr);

// Memory management utilities
size_t qi_get_allocated_memory(void);
void qi_reset_memory_stats(void);

#endif // QI_MEMORY_H
