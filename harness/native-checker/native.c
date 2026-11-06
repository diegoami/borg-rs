/* Compile the real C calculator, renamed beside the unchanged Rust module.
 * Only its explicit environment boundary (value lookup and diagnostics) is stubbed. */
#include <stdarg.h>
#include <strings.h>
#define parse_calculation_line native_parse_calculation_line
#define borg_calculate_dynamic native_borg_calculate_dynamic
#define calculations_free native_calculations_free
#define calculations native_calculations
#include "borg/borg-formulas-calc.c"

static char *output;
static size_t capacity, used;
static void append(const char *fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    int n = vsnprintf(output + used, capacity - used, fmt, args);
    va_end(args);
    if (n < 0 || (size_t)n >= capacity - used) abort();
    used += (size_t)n;
}
void borg_note(const char *text) { append("N %s\n", text); }
void borg_formula_error(const char *section, const char *full, const char *label, const char *error)
{ append("E [%s][%s][%s][%s]\n", section, full, label, error ? error : ""); }
struct value_sec *parse_value(char *line, const char *full)
{
    (void)full;
    if (strncasecmp(line, "value(", 6)) return NULL;
    char *end;
    long i = strtol(line + 6, &end, 10);
    if (*end != ')' || end[1] || !((i >= 0 && i < 10) || i == 99)) return NULL;
    struct value_sec *v = mem_zalloc(sizeof(*v));
    v->type = VT_RANGE_INDEX;
    v->index = (int32_t)i;
    return v;
}
int32_t calculate_from_value(struct value_sec *v, int range)
{ return v->index == 99 ? range : v->index * 7 - 10; }
void native_clear(void) { native_calculations_free(); }
int native_snapshot(const char *expression, int range, char *buffer, size_t size)
{
    output = buffer; capacity = size; used = 0;
    char *line = string_make(expression);
    int index = native_parse_calculation_line(line, expression);
    int result = index < 0 ? 0 : native_borg_calculate_dynamic(index, range);
    append("R %d %d\nS %d\n", index, result, native_calculations.count);
    for (int j = 0; j < native_calculations.count; ++j) {
        struct borg_calculation *f = native_calculations.items[j];
        append("F %d %d\n", f->max_depth, f->token_array->count);
        for (int i = 0; i < f->token_array->count; ++i) {
            struct token *t = f->token_array->items[i];
            int a = 0, b = 0;
            if (t->type == TOK_NUMBER) a = *(int32_t *)t->token;
            if (t->type == TOK_VALUE) { a = ((struct value_sec *)t->token)->type; b = ((struct value_sec *)t->token)->index; }
            append("T %d %d %d %d %d\n", t->type, t->not, t->pdepth, a, b);
        }
    }
    string_free(line);
    return (int)used;
}
