#[cfg(feature = "cycle-tracker")]
#[inline(always)]
pub(crate) fn start(name: &str) {
    println!("cycle-tracker-report-start: {name}");
}

#[cfg(feature = "cycle-tracker")]
#[inline(always)]
pub(crate) fn end(name: &str) {
    println!("cycle-tracker-report-end: {name}");
}

#[cfg(not(feature = "cycle-tracker"))]
#[inline(always)]
pub(crate) fn start(_: &str) {}

#[cfg(not(feature = "cycle-tracker"))]
#[inline(always)]
pub(crate) fn end(_: &str) {}
