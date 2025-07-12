#pragma once

#include "Framework.h"
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Simple feature availability check - this is what's currently implemented
DLLEXPORT bool IsFeatureAvailable(const char* feature);

#ifdef __cplusplus
}
#endif
