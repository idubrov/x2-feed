_page_size = 1K;
_eeprom_pages = 20;

SECTIONS
{
    . = ORIGIN(FLASH) + LENGTH(FLASH) - (_eeprom_pages * _page_size);
    _eeprom_start = .;
    .eeprom :
    {
        . = ORIGIN(FLASH) + LENGTH(FLASH);
    } > FLASH
    _eeprom_end = .;
}

ASSERT(_sidata + (_edata - _sdata) <= _eeprom_start, "
The '.data' overlaps with EEPROM area. Reduce the amount of FLASH pages used for
EEPROM or reduce the amount of code stored on the FLASH");