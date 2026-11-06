/* Demonstrate borg_what_text's byte-limit problem without uninitialised reads.
 * No Borg source is changed or linked into this libc-only diagnostic. */
#include <assert.h>
#include <locale.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <wchar.h>

int main(void)
{
    wchar_t screen[80];
    char buffer[1024];
    size_t count;
    assert(setlocale(LC_ALL, "C.UTF-8"));
    for (size_t i = 0; i < 79; ++i) screen[i] = L' ';
    screen[79] = L'\0';
    memset(buffer, '?', sizeof(buffer));
    count = wcstombs(buffer, screen, 80);
    assert(count == 79 && buffer[79] == '\0');
    screen[0] = L'\u00fb';
    memset(buffer, '?', sizeof(buffer));
    memcpy(buffer + 80, "STALE", 6);
    count = wcstombs(buffer, screen, 80);
    assert(count == 80);
    assert(memchr(buffer, '\0', 80) == NULL);
    assert(strlen(buffer) == 85);
    puts("ASCII: terminated at byte 79; UTF-8: 80 bytes, no NUL; stale tail read by strlen");
    return 0;
}
