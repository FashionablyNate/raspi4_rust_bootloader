use core::arch::asm;

pub struct Timer {
    period: u64,
    ticks_per_us: u64,
    last_trigger: u64,
}

impl Timer {
    pub fn new(period_ms: u64) -> Self {
        Self {
            period: period_ms * 1_000,
            last_trigger: Self::read_cntpct_el0(),
            ticks_per_us: Self::read_cntfrq_el0() / 1_000_000,
        }
    }

    pub fn elapsed(&mut self) -> bool {
        let current_tick = Self::read_cntpct_el0();
        if current_tick - self.last_trigger > self.ticks_per_us * self.period {
            self.last_trigger = current_tick;
            true
        } else {
            false
        }
    }

    /// Reads the system counter frequency in Hz (ticks per second)
    fn read_cntfrq_el0() -> u64 {
        let value: u64;
        // SAFETY: Reading from cntfrq_el0 is side-effect free and always safe in EL0
        unsafe {
            asm!("mrs {}, cntfrq_el0", out(reg) value);
        }
        value
    }

    /// Reads the current physical counter value (monotonic timer ticks)
    fn read_cntpct_el0() -> u64 {
        let value: u64;
        // SAFETY: Reading from cntpct_el0 is side-effect free and always safe in EL0
        unsafe {
            asm!("mrs {}, cntpct_el0", out(reg) value);
        }
        value
    }
}
