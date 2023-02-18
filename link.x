ENTRY(_start)

SECTIONS
{
    .text : {
        *(.text.entry);
        *(.text.*);
        *(.rodata.*);
    }

    /DISCARD/ : {
        *(.dynsym);
        *(.gnu.hash);
        *(.hash);
        *(.dynstr);
        *(.rela.dyn);
    }
}
