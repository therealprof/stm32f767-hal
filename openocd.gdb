target extended-remote :3333

# Print demangled symbols.
set print asm-demangle on

# Set backtrace limit to not have infinite backtrace loops.
set backtrace limit 32

# Detect unhandled exceptions, hard faults and panics.
break DefaultHandler
break HardFault
break rust_begin_unwind

# *Try* to stop at the user entry point (it might be gone due to inlining).
break main

# Enable semi-hosting.
monitor arm semihosting enable

load

# Start the process but immediately halt the processor.
stepi
