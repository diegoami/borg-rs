#include "angband.h"
#include "borg/borg-item.h"
#include "borg/borg-item-id.h"
#include <stdio.h>
int main(void) {
    borg_item live[3] = {0}, safe[3] = {0};
    live[0].iqty = live[1].iqty = 1;
    live[0].aware = live[1].aware = true;
    strcpy(live[0].desc, "Old");
    live[0].note = live[0].desc + 3;
    strcpy(live[1].desc, "New longer {??}");
    live[1].note = live[1].desc + 11;
    memcpy(safe, live, sizeof live);
    /* wear_trial: old equipment 0 into hole 2, candidate 1 into slot 0 */
    memcpy(&live[2], &safe[0], sizeof(borg_item));
    memcpy(&live[0], &safe[1], sizeof(borg_item));
    printf("hole note=[%s] needs_id=%d\n", borg_get_note(&live[2]), borg_item_note_needs_id(&live[2]));
}
