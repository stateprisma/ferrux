use super::cpu::{asm_inb, asm_outb};

pub const COM1: u16 = 0x3f8;

pub struct Uart {
    port: u16,
}

impl Uart {
    pub fn init_uart(port: u16) -> Self {
        asm_outb(port + 1, 0x00); // Disable interrupts
        asm_outb(port + 3, 0x80); // Enable baud rate divisor
        asm_outb(port, 0x01); // Divisor hi-byte
        asm_outb(port + 1, 0x00); //         lo-byte
        asm_outb(port + 3, 0x03);
        asm_outb(port + 2, 0xC7);
        asm_outb(port + 4, 0x0B);
        asm_outb(port + 4, 0x1E);

        let this = Self { port };

        // quickly test the chip
        this.putchar(0xAE);
        if this.getchar() != 0xAE {
            panic!();
        }
        // All works
        asm_outb(port + 4, 0x0F);
        this
    }
    pub fn putchar(&self, c: u8) {
        asm_outb(self.port, c);
    }

    pub fn getchar(&self) -> u8 {
        asm_inb(self.port)
    }

    pub fn putstr(&self, buf: &[u8]) {
        for byte in buf {
            self.putchar(*byte);
        }
    }
}
