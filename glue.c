#include <errno.h>
//#include <resolv.h>
#include <stdlib.h>
#include <sys/socket.h>

int __socket30(int domain, int type, int protocol) {
  return socket(domain, type, protocol);
}

int __sigaltstack14(int ss, int oss) { return 0; }

int *__errno_location() { return __errno(); }

int res_init() { return 0; }

int dlsym() { return 0; }
