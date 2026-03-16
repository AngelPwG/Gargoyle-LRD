#ifndef GLRD_EVENT_H
#define GLRD_EVENT_H
#include "vmlinux.h"

struct glrd_event
{
    u32 pid;
    char nombre_proceso[255];
    char ruta_ejecutable[512];
    char usuario_so[64];
    char ruta_ejemplo[512];
    u32 archivos_afectados;
    u32 entropia_x10000;
    u64 timestamp_ns;
};
#endif
