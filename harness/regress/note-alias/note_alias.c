/*
 * Differential check of borg_item.note aliasing (review of 4c06cec08,
 * finding 1): the C side. Links the original C borg-item.c and
 * borg-item-id.c; prints the note bytes and borg_item_note_needs_id() of
 * items after the struct copies the Borg's what-if trials do.
 * borg-rs/examples/note_alias.rs prints the same lines from the Rust port;
 * harness/regress/note-alias/run.sh compares the two outputs.
 *
 * C sets `note` only in borg_item_analyze(), on borg_items[i] or
 * borg_shops[s].ware[j], as `desc + offset`; set_note() does the same here.
 */
#include "angband.h"
#include "borg/borg-item.h"
#include "borg/borg-item-id.h"
#include <stdio.h>

static borg_item items[4], snap_items[4], ware[2], snap_ware[2];

static void put(borg_item *it, const char *desc, int note_off, bool ident,
                bool needs_ident)
{
    memset(it, 0, sizeof(*it));
    it->iqty = 1;
    it->aware = true;
    it->ident = ident;
    it->needs_ident = needs_ident;
    strcpy(it->desc, desc);
    it->note = it->desc + note_off;
}

static void show(const char *tag, borg_item *it)
{
    printf("%s note=[%s] needs_id=%d\n", tag, it->note ? borg_get_note(it) : "(null)",
        borg_item_note_needs_id(it) ? 1 : 0);
}

int main(void)
{
    /* S1: wear trial (borg-item-wear.c): old equipment into the pack hole,
     * candidate over the equipment slot, then borg_notice reads the hole.
     * ident=false, needs_ident=false: all runes known, effect unknown. */
    put(&items[0], "Old", 3, false, false);
    put(&items[1], "New longer {??}", 11, false, false);
    memset(&items[2], 0, sizeof(borg_item));
    memcpy(snap_items, items, sizeof items);
    memcpy(&items[2], &snap_items[0], sizeof(borg_item));
    memcpy(&items[0], &snap_items[1], sizeof(borg_item));
    show("S1 slot", &items[0]);
    show("S1 cand", &items[1]);
    show("S1 hole", &items[2]);
    /* restore, as the trial does */
    memcpy(&items[0], &snap_items[0], sizeof(borg_item));
    memcpy(&items[2], &snap_items[2], sizeof(borg_item));
    show("S1 restored slot", &items[0]);

    /* S2: the reverse: a long old description, a short candidate. */
    put(&items[0], "Long old desc {cursed}", 14, false, false);
    put(&items[1], "Short", 5, false, false);
    memset(&items[2], 0, sizeof(borg_item));
    memcpy(snap_items, items, sizeof items);
    memcpy(&items[2], &snap_items[0], sizeof(borg_item));
    memcpy(&items[0], &snap_items[1], sizeof(borg_item));
    show("S2 hole", &items[2]);

    /* S3: a store ware copied into the pack (store-buy trials), then the
     * live ware changes (the next store read). */
    put(&ware[0], "A ware {??}", 7, false, false);
    put(&ware[1], "Second", 6, false, true);
    memcpy(snap_ware, ware, sizeof ware);
    memcpy(&items[3], &ware[0], sizeof(borg_item));
    show("S3 pack copy", &items[3]);
    put(&ware[0], "Changed {magical}", 8, true, false);
    show("S3 pack copy after ware change", &items[3]);
    show("S3 safe ware", &snap_ware[0]);

    /* S4: a copy whose origin is unchanged reads its own bytes. */
    put(&items[0], "Plain {??}", 6, false, false);
    memcpy(&items[1], &items[0], sizeof(borg_item));
    show("S4 copy", &items[1]);

    /* S5: NULL note (a zeroed item) and needs_ident short cut. */
    memset(&items[0], 0, sizeof(borg_item));
    show("S5 null", &items[0]);
    put(&items[0], "x", 1, false, true);
    show("S5 needs_ident", &items[0]);
    return 0;
}
