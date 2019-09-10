/**
 *   GCC linker script for CC2538.
 */

MEMORY
{
  /* would like to use SYMBOLS (from above)here but we cannot
   * GCC version 4.9 does not support symbolic expressions here.
   * But later versions do support the feature.
   */
  FLASH (rx) :     ORIGIN = 0x00200000,     LENGTH = 0x0007c000
  FLASH_CCA (rx) : ORIGIN = 0x0027FFD4,     LENGTH = 0x2c
  RAM (rwx) :      ORIGIN = 0x20000000,     LENGTH = 32K
}

/* Example for defining a flash cca area in a custom location. */
/* Note that the section will not be zero-initialized by the runtime! */
SECTIONS {
	.flashcca : 
    {
		KEEP(*(.flashcca))
	} > FLASH_CCA
} INSERT AFTER .uninit;
