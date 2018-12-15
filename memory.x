MEMORY
{
    ROM (rx)        : ORIGIN = 0x00000000, LENGTH = 0x002000 /* 8 KB */
    RAM (xrw)       : ORIGIN = 0x10000000, LENGTH = 0x002000 /* 8 KB */
}

PROVIDE(_stack_size = 0x100);
