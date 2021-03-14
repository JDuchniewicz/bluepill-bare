target remote :3333
set backtrace limit 32
load
monitor reset
break reset_handler
continue
