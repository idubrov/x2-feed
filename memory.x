_page_size = 1K;
_eeprom_pages = 10;

MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K - (_eeprom_pages * _page_size)
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
  EEPROM : ORIGIN = ORIGIN(FLASH) + LENGTH(FLASH), LENGTH = (_eeprom_pages * _page_size)
}

_eeprom_start = ORIGIN(EEPROM);
_eeprom_offset = LENGTH(FLASH);
_eeprom_end = ORIGIN(EEPROM) + LENGTH(EEPROM);
