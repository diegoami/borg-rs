set pagination off
set can-use-hw-watchpoints 1
break obj-knowledge.c:1137 if obj == (struct object *)0x555555e12420
commands
silent
printf "=== ALLOC known, turn=%d curses=%p\n", turn, obj->curses
watch -l ((struct object *)0x555555e12420)->curses
commands
silent
printf "=== WRITE target->curses now=%p turn=%d\n", ((struct object *)0x555555e12420)->curses, turn
bt 10
continue
end
continue
end
run
bt 4
