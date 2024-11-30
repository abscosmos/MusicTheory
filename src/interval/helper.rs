#[macro_export]
macro_rules! intervals {
    ($( $quality:ident $size:ident ),*) => {
        [
            $(
                $crate::interval!($quality $size)
            ),*
        ]
    };
    (vec <- $( $quality:ident $size:ident ),*) => {
        $crate::intervals!($( $quality $size ),*).to_vec()
    };
}

#[macro_export]
macro_rules! intervals_vec {
    ($( $quality:ident $size:ident ),*) => {
        $crate::intervals!(vec <- $( $quality $size ),*)
    };
}

#[macro_export]
macro_rules! interval {
    ($quality:ident $size:ident) => {{
        let quality = $crate::interval::quality::IntervalQuality::$quality;
        let size = $crate::interval::size::IntervalSize::$size;
                
        $crate::Interval::from_quality_and_size(quality, size)
            .unwrap_or_else(|| panic!("{quality:?} {size:?} isn't a valid interval"))
    }};
}