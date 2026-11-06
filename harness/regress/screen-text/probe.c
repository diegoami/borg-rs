/* Link the real borg_what_text; stub only the terminal surface. */
#include "borg/borg-io.h"
#include "ui-term.h"
#include <assert.h>
#include <locale.h>
#include <string.h>
term *Term;
errr Term_get_size(int *w, int *h) { *w = 80; *h = 1; return 0; }
int main(void) {
    int attrs[80], *arows[] = {attrs};
    wchar_t chars[80], *crows[] = {chars};
    term_win window = {0}; term terminal = {0};
    char out[81]; uint8_t attr = 0;
    assert(setlocale(LC_ALL, "C.UTF-8"));
    for (int i = 0; i < 80; i++) { attrs[i] = 1; chars[i] = L' '; }
    window.a = arows; window.c = crows; terminal.scr = &window; Term = &terminal;
    memset(out, '?', sizeof(out));
    assert(borg_what_text(0, 0, 79, &attr, out, sizeof(out)) == 0);
    assert(memchr(out, 0, 80) != NULL && out[80] == '?');
    chars[0] = (wchar_t)0xfb;
    memset(out, '?', sizeof(out));
    assert(borg_what_text(0, 0, 79, &attr, out, sizeof(out)) == 0);
    assert(strlen(out) == 80 && out[80] == 0);
    assert((unsigned char)out[0] == 0xc3 && (unsigned char)out[1] == 0xbb);
    /* Do not leave a partial multibyte encoding at the output limit. */
    chars[0] = L' '; chars[78] = (wchar_t)0xfb;
    memset(out, '?', sizeof(out));
    assert(borg_what_text(0, 0, 79, &attr, out, 80) == 0);
    assert(out[78] == 0 && out[80] == '?');
    /* Capacity includes NUL, including zero/one-byte output buffers. */
    memset(out, '?', sizeof(out));
    assert(borg_what_text(0, 0, 79, &attr, out, 0) != 0 && out[0] == '?');
    assert(borg_what_text(0, 0, 79, &attr, out, 1) == 0);
    assert(out[0] == 0 && out[1] == '?');
    /* A column count is not a UTF-8 byte capacity: retain the complete
     * monster name/message when the caller provides enough output space. */
    const wchar_t *message = L"Sm\u00e9agol hits you.";
    for (int i = 0; i < 80; i++) chars[i] = L' ';
    for (size_t i = 0; i < wcslen(message); i++) chars[i] = message[i];
    memset(out, '?', sizeof(out));
    assert(borg_what_text(0, 0, (int)wcslen(message), &attr, out, sizeof(out)) == 0);
    assert(strcmp(out, "Sm\xc3\xa9" "agol hits you.") == 0);
    chars[0] = (wchar_t)0xd800;
    memset(out, '?', sizeof(out));
    assert(borg_what_text(0, 0, 79, &attr, out, sizeof(out)) != 0 && out[0] == 0);
    puts("actual borg_what_text: ASCII, full UTF-8 message, capacity, boundary and invalid encoding passed");
}
