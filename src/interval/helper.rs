#[macro_export]
macro_rules! intervals {
    ($( $quality:ident $size:ident ),*) => {
        [
            $(
                crate::Interval::from_quality_and_size(
                    crate::interval::quality::IntervalQuality::$quality,
                    crate::interval::size::IntervalSize::$size
                ).expect("interval to be valid")
            ),*
        ]
    };
    (vec <- $( $quality:ident $size:ident ),*) => {
        ivls_slice!($( $quality $size ),*).to_vec()
    };
}

#[macro_export]
macro_rules! intervals_vec {
    ($( $quality:ident $size:ident ),*) => {
        vec![
            $(
                crate::interval::Interval::from_quality_and_size(
                    crate::interval::quality::IntervalQuality::$quality,
                    crate::interval::size::IntervalSize::$size
                ).expect("interval to be valid")
            ),*
        ]
    };
}

#[macro_export]
macro_rules! interval {
    ($quality:ident $size:ident) => {
        crate::interval::Interval::from_quality_and_size(
            crate::interval::quality::IntervalQuality::$quality,
            crate::interval::size::IntervalSize::$size
        ).expect("interval to be valid")
    };
}