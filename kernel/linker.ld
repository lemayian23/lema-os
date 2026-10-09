ENTRY(_start)

SECTIONS
{
    . = 0xffffffff80000000;

    .text : {
        *(.limine_requests_start)
        *(.limine_requests)
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
    }
}