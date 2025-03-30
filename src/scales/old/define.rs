macro_rules! define_scale {
    (
        name = $name: ident,
        size = $size: tt,
        intervals = $intervals: expr
        $(, alias = $alias: ident)?
        $(, mode_aliases = [$($alias_mode: ident => $alias_mode_num: ident),* $(,)?])?
        $(,)?
    ) => {
        define_scale!(@define $size, $name);
        
        define_scale!(@scale_modes $size, $name, $intervals);

        $(define_scale!(@scale_alias $name, $alias);)?

        $(define_scale!(@mode_aliases $name, $($alias_mode => $alias_mode_num),*);)?
    };
    
    (@define 5, $name: ident) => {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, strum_macros::FromRepr)]
        pub enum $name {
            I = 1,
            II,
            III,
            IV,
            V,
        }
    };
    
    (@define 7, $name: ident) => {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, strum_macros::FromRepr)]
        pub enum $name {
            I = 1,
            II,
            III,
            IV,
            V,
            VI,
            VII,
        }
    };
    
    (@define $size: tt, $name: ident) => {
        compile_error!(concat!("Unsupported scale size: ", stringify!($size)));
    };
    
    (@scale_modes $size: expr, $name: ident, $intervals: expr) => {
        impl $crate::scales::old::ScaleModes< $size > for $name {
            const RELATIVE_INTERVALS: [Interval; $size] = $intervals;

            fn number(&self) -> u8 {
                *self as _
            }

            fn from_number(number: u8) -> Option<Self> {
                Self::from_repr(number)
            }
        }
    };
    
    (@scale_alias $name: ident, $alias: ident) => {
        pub use $name as $alias;
    };
    
    (@mode_aliases $name: ident, $($alias_mode:ident => $alias_mode_num:ident),*) => {
        impl $name {
            $(
                pub const $alias_mode : Self = Self:: $alias_mode_num ;
            )*
        }
    };
}

pub(in crate::scales) use define_scale;