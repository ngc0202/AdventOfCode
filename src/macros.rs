
macro_rules! use_days {
    ($n:tt) => (paste!{#[allow(dead_code)] mod [<day $n>];});
    ($n:tt *) => (paste!{mod [<day $n>];});
    ($n:tt $(* $(@$star:tt)?)?, $($nn:tt $(* $(@$stars:tt)?)?),+) => (use_days!($n $(* $($star)?)?); use_days!($($nn $(* $($stars)?)?),+););
}

macro_rules! run_days {
	($($n:tt $(* $(@$star:tt)?)?),+) => (
		fn main() -> GenResult {
			paste! {
				$($(println!(concat!("\nRunning day ", $n, ":")); [<day $n>]::run()?; $($star)?)?)+
			}
			Ok(())
		}
	)
}

macro_rules! days {
	($($n:tt)+) => (
		use_days!($($n)+);
		run_days!($($n)+);
	);
}
