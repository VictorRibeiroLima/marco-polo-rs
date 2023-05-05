#[allow(unused_macros)]
macro_rules! time_it {
    ($func:expr,$duration:ident) => {{
        let start = std::time::SystemTime::now();
        $func;
        let end = std::time::SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        let duration_param = stringify!($duration);
        println!(
            "The block completed in `{}`: {}",
            duration_param,
            duration.$duration()
        )
    }};
}

#[allow(unused_imports)]
pub(crate) use time_it;
pub mod fs;
