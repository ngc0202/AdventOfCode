macro_rules! use_days {
    ($n:tt) => (paste!{#[allow(dead_code)] mod [<day $n>];});
    ($n:tt *) => (paste!{mod [<day $n>];});
    ($n:tt $(* $(@$star:tt)?)?, $($nn:tt $(* $(@$stars:tt)?)?),+) => (use_days!($n $(* $($star)?)?); use_days!($($nn $(* $($stars)?)?),+););
}

macro_rules! run_days {
	($($n:tt $(* $(@$star:tt)?)?),+) => (
		pub fn main() -> Result<(), Whatever> {
			paste! {
				$($(println!(concat!("\nRunning day ", $n, ":")); whatever!([<day $n>]::run(), "Failed running Day {}", $n); $($star)?)?)+
			}
			Ok(())
		}
	)
}

macro_rules! days {
	($($n:tt)+) => (
		use crate::prelude::*;
		use paste::paste;
		use_days!($($n)+);
		run_days!($($n)+);
	);
}

macro_rules! day {
	(run $day:tt $($bnd:tt)*) => {
		pub fn run() -> Result<(), snafu::Whatever> {
			paste::paste! {
				struct [<Day $day In>];
				impl crate::y2024::InputSolution for [<Day $day In>] {
					day!($day);
					type Sln<'i> = [<Day $day>]$($bnd)*;
				}
				crate::y2024::solve::<[<Day $day In>]>()
			}
		}
	};
    ($day:expr) => {
        const DAY: crate::prelude::Day = crate::prelude::Day {
            day: $day,
            year: super::YEAR,
        };
    };
}
