/*
usage: bindgen external.h
*/

/* for rustfmt */
typedef unsigned long size_t;

/**
 * Defines the memory destructor handler, which is called when the reference
 * of a memory object goes down to zero
 *
 * @param data Pointer to memory object
 */
typedef void (mem_destroy_h)(void *data);

void *mem_alloc(size_t size, mem_destroy_h *dh);
void *mem_zalloc(size_t size, mem_destroy_h *dh);
void *mem_realloc(void *data, size_t size);
void *mem_deref(void *data);
void mem_debug(void);