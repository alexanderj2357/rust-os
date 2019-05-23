#define GPIO_BASE (0x3F000000 + 0x200000)

volatile unsigned *GPIO_FSEL1 = (volatile unsigned *)(GPIO_BASE + 0x04);
volatile unsigned *GPIO_SET0 = (volatile unsigned *)(GPIO_BASE + 0x1C);
volatile unsigned *GPIO_CLR0 = (volatile unsigned *)(GPIO_BASE + 0x28);

#define TRUE 1
#define TURN_ON(n) (1 << n)

static void spin_sleep_us(unsigned int us) {
    for (unsigned int i = 0; i < us * 6; i++) {
        asm volatile("nop");
    }
}

static void spin_sleep_ms(unsigned int ms) {
    spin_sleep_us(ms * 1000);
}

int main(void) {
    // Set GPIO pin 16 as output 
    unsigned payload = TURN_ON(18);
    *GPIO_FSEL1 = payload;
    // Set and clear with GPIO_SET and GPIO_CLEAR 
    // to make the LED blink 
    while (TRUE) {
        payload = TURN_ON(16);
        *GPIO_SET0 = payload;
        spin_sleep_ms(200);
        *GPIO_CLR0 = payload;
        spin_sleep_ms(200);
    }
}
