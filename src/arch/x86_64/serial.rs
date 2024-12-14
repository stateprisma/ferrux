use super::cpu::{asm_inb, asm_outb};

const PORT: u16 = 0x3f8;

struct UART;

impl UART {
    pub fn init_uart() -> Self {
        asm_outb(PORT + 1, 0x00); // Disable interrupts
        asm_outb(PORT + 3, 0x80); // Enable baud rate divisor
        asm_outb(PORT + 0, 0x01); // Divisor hi-byte
        asm_outb(PORT + 1, 0x00); //         lo-byte
        asm_outb(PORT + 3, 0x03);
        asm_outb(PORT + 2, 0xC7);
        asm_outb(PORT + 4, 0x0B);
        asm_outb(PORT + 4, 0x1E);

        // quickly test the chip
        Self::putchar(0xAE);
        if Self::getchar() != 0xAE {
            panic!();
        }

        // All works
        asm_outb(PORT + 4, 0x0F);

        Self
    }

    pub fn putchar(c: u8) {
        asm_outb(PORT, c);
    }

    pub fn getchar() -> u8 {
        asm_inb(PORT)
    }
}
