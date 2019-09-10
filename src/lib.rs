#![no_std]

extern crate cc2538;

const FLASH_CCA_BOOTLDR_CFG_ENABLE: u32 = 0xF0FF_FFFF;
const FLASH_CCA_BOOTLDR_CFG_PORT_A_PIN_S: u32 = 24;
const FLASH_CCA_IMAGE_VALID: u32 = 0x0000_0000;
const FLASH_CCA_CONF_BOOTLDR_BACKDOOR_PORT_A_PIN: u32 = 3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flash_cca_lock_page {
    pub ui32_bootldr_cfg: u32,
    pub ui32_image_valid: u32,
    pub ui32_image_vector_addr: u32,
    pub ui8_lock: [u8; 32usize],
}

#[link_section = ".flashcca"]
#[no_mangle]
pub static FLASH_CCA_LOCK_PAGE: flash_cca_lock_page = flash_cca_lock_page {
    ui32_bootldr_cfg: FLASH_CCA_BOOTLDR_CFG_ENABLE
        | (FLASH_CCA_CONF_BOOTLDR_BACKDOOR_PORT_A_PIN << FLASH_CCA_BOOTLDR_CFG_PORT_A_PIN_S),
    ui32_image_valid: FLASH_CCA_IMAGE_VALID,
    ui32_image_vector_addr: 0x0020_0000,
    ui8_lock: [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ],
};
