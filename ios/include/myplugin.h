#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct HashMap_String__FinClipCall HashMap_String__FinClipCall;

struct HashMap_String__FinClipCall *myplugin_register_apis(void);

void myplugin_release(struct HashMap_String__FinClipCall *ptr);
