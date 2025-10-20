#ifndef QI_RUNTIME_H
#define QI_RUNTIME_H

#include <stdint.h>
#include <stddef.h>

// Runtime initialization
void qi_runtime_init(void);
void qi_runtime_cleanup(void);

// Version information
#define QI_RUNTIME_VERSION_MAJOR 0
#define QI_RUNTIME_VERSION_MINOR 1
#define QI_RUNTIME_VERSION_PATCH 0

#endif // QI_RUNTIME_H
