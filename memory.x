_page_size = 1K;
# Note : need to be in sync with hal/mod.rs
_eeprom_pages = 10;

MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K - (_eeprom_pages * _page_size)
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}
