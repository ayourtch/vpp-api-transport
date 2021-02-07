#include <stddef.h>

typedef unsigned char u8;

typedef void (*vac_callback_t)(const unsigned char * data, int len);
typedef void (*vac_error_callback_t)(const void *, const unsigned char *, int);
int vac_connect(const char * name, const char * chroot_prefix, vac_callback_t cb,
    int rx_qlen);
int vac_disconnect(void);
int vac_read(u8 **data, int *l, unsigned short timeout);
int vac_write(const u8 *data, int len);
void vac_free(void * msg);

int vac_get_msg_index(const unsigned char * name);
int vac_msg_table_size(void);
int vac_msg_table_max_index(void);

void vac_rx_suspend (void);
void vac_rx_resume (void);
void vac_set_error_handler(vac_error_callback_t);
void vac_mem_init (size_t size);

