pub mod heptatonic;
pub mod pentatonic;
pub mod chromatic;
pub mod hexatonic;
pub mod octatonic;

// constants to help defining scales
const T: Interval = Interval::MAJOR_SECOND;
const S: Interval = Interval::MINOR_SECOND;
const TS: Interval = Interval::MINOR_THIRD;
const TT: Interval = Interval::MAJOR_THIRD;
const A2: Interval = Interval::AUGMENTED_SECOND;

macro_rules! define_scale {
    (
        name = $name:ident,
        size = $size:expr,
        intervals = $intervals:expr
        $(, mode = [$($first_var:ident)? $(, $rest_var:ident)* $(,)?])?
        $(, mode_aliases = [$($mode_alias:ident => $aliased_mode:ident),* $(,)?])?
        $(, typed = $typed:ident)?
        $(, exact_single = $exact_single:ident)?
        $(, exact = [$($var: ident => $var_name: ident),* $(,)?])?
        // $(, alias = $alias: ident)?
        $(,)?
    ) => {
        #[allow(unused_imports)]
        use $crate::scales::numeral::*;
        
        ::paste::paste! {
            define_scale!(@try_custom_mode [<$name Mode>], $size $(, [$($first_var)? $(, $rest_var)*])?);
            
            $(define_scale!(@mode_aliases [<$name Mode>], $($mode_alias => $aliased_mode),*);)?
            
            define_scale!(@definition [<$name ScaleDef>], $size, [<$name Mode>], $intervals);
            
            $(define_scale!(@typed $typed, [<$name ScaleDef>], $size);)*
            
            $(define_scale!(@define_exact [<$name Scale>], $exact_single, [<$name Mode>], [<$name ScaleDef>], $size);)?
            
            $(define_scale!(@exact [<$name Mode>], [<$name ScaleDef>], $size, [$($var => $var_name),*]);)*
        }
    };

    (@try_custom_mode $name:ident, $size:expr) => {
        ::paste::paste! {
            use [<Numeral $size>] as $name;
        }
    };
    
    // support for chromatic 1 mode
    (@try_custom_mode $name: ident, $size:expr, []) => {
        #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name;
        
        ::paste::paste! {
            impl $crate::scales::ScaleMode< $size > for $name {
                type Base = [<Numeral $size>];
        
                fn as_num(self) -> u8 {
                    1
                }
        
                fn from_num(number: u8) -> Option<Self> where Self: Sized {
                    (number == 1).then_some(Self)
                }
            }
        }
    };
    
    (@try_custom_mode $name: ident, $size:expr, [$first_var:ident $(, $rest_var:ident)*]) => {
        define_scale!(@custom_mode $name, [$first_var $(, $rest_var)*]);
        
        ::paste::paste!{
            define_scale!(@scale_mode $name, [<Numeral $size>], $size);
        }
    };

    (@custom_mode $name: ident, [$first_var:ident $(, $rest_var:ident)*]) => {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Ord, PartialOrd, strum_macros::FromRepr)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum $name {
            #[default]
            $first_var = 1,
            $($rest_var),*
        }
    };

    (@scale_mode $name: ident, $base_ty: ident, $size: expr) => {
        impl $crate::scales::ScaleMode< $size > for $name {
            type Base = $base_ty;
    
            fn as_num(self) -> u8 {
                self as _
            }
    
            fn from_num(number: u8) -> Option<Self> where Self: Sized {
                Self::from_repr(number)
            }
        }
    };
    
    (@definition $def_name:ident, $size:expr, $mode_name:ident, $intervals:expr) => {
        #[derive(Debug, Copy, Clone)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $def_name;
    
        impl $crate::scales::ScaleDefinition<$size> for $def_name {
            type Mode = $mode_name;
            const INTERVALS: [$crate::interval::Interval; $size] = $intervals;
        }
    };
    
    (@typed $typed:ident, $def: ident, $size:expr) => {
        pub type $typed = $crate::scales::typed_scale::TypedScale<$def, $size>;
    };
    
    (@exact $mode:ident, $def:ident, $size:expr, [$($var:ident => $var_name:ident),* $(,)?]) => {
        ::paste::paste! {
            $(
                define_scale!(@define_exact [<$var_name Scale>], $var, $mode, $def, $size);
            )*
        }
    };
    
    (@define_exact $name:ident, $var:ident, $mode:ident, $def:ident, $size:expr) => {
        #[derive(Default, Debug, Clone, Copy)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name;
        
        impl $crate::scales::exact_scale::ExactScale<$size> for $name {
            type Scale = $def;
            
            fn as_typed(&self) -> $crate::scales::typed_scale::TypedScale<Self::Scale, $size> {
                $crate::scales::typed_scale::TypedScale::new($mode::$var)
            }
        }
    };
    
    (@mode_aliases $name: ident, $($mode_alias:ident => $aliased_mode:ident),*) => {
        impl $name {
            $(
                pub const $mode_alias : Self = Self:: $aliased_mode ;
            )*
        }
    };
}

use define_scale;
use crate::interval::Interval;