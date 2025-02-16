static TICKS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

pub fn tick_handler() {
    TICKS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
}

pub struct MilliSecondClock32;

impl embedded_timers::clock::Clock for MilliSecondClock32 {
    type Instant = embedded_timers::instant::Instant32<1000>;

    fn now(&self) -> Self::Instant {
        let ticks = TICKS.load(core::sync::atomic::Ordering::Relaxed);
        embedded_timers::instant::Instant32::<1000>::new(ticks)
    }
}
