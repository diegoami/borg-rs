/* Coverage-only link shim: borgh_finish uses _exit, bypassing atexit.
 * The ordinary harness never links this object. */
#include <unistd.h>
extern int __llvm_profile_write_file(void);
extern void __real__exit(int) __attribute__((noreturn));
void __wrap__exit(int status)
{
    if (status == 0 && __llvm_profile_write_file() != 0)
        status = 4;
    __real__exit(status);
}
