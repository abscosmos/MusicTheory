macro_rules! define_scale {
    (
        name = $name:ident,
        size = $size:expr,
        intervals = $intervals:expr
        $(, mode = [$first_var:ident $(, $rest_var:ident)* $(,)?])?
        // $(, alias = $alias: ident)?
        // $(, mode_aliases = [$($alias_mode: ident => $alias_mode_num: ident),* $(,)?])?
        $(,)?
    ) => {
        use $crate::scales::numeral::*;
        
        use paste::paste;
        
        paste! {
            define_scale!(@try_custom_mode [<$name Mode>], $size $(, [$first_var $(, $rest_var)*])?);
            
            define_scale!(@definition [<$name ScaleDef>], $size, [<$name Mode>], $intervals);
        }
        
        // 
        // $(define_scale!(@scale_alias $name, $alias);)?
        // 
        // $(define_scale!(@mode_aliases $name, $($alias_mode => $alias_mode_num),*);)?
    };

    (@try_custom_mode $name:ident, $size:expr) => {
        paste! {
            use [<Numeral $size>] as $name;
        }
    };
    
    (@try_custom_mode $name: ident, $size:expr, [$first_var:ident $(, $rest_var:ident)*]) => {
        define_scale!(@custom_mode $name, [$first_var $(, $rest_var)*]);
        
        paste!{
            define_scale!(@scale_mode $name, [<Numeral $size>], $size);
        }
        
    };

    (@custom_mode $name: ident, [$first_var:ident $(, $rest_var:ident)*]) => {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, strum_macros::FromRepr)]
        pub enum $name {
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
        #[derive(Debug)]
        pub struct $def_name;
    
        impl $crate::scales::ScaleDefinition<$size> for $def_name {
            type Mode = $mode_name;
            const INTERVALS: [Interval; $size] = $intervals;
        }
    };
    
    // (@scale_alias $name: ident, $alias: ident) => {
    //     pub use $name as $alias;
    // };
    //
    // (@mode_aliases $name: ident, $($alias_mode:ident => $alias_mode_num:ident),*) => {
    //     impl $name {
    //         $(
    //             pub const $alias_mode : Self = Self:: $alias_mode_num ;
    //         )*
    //     }
    // };
}

use paste::paste;
pub(in crate::scales) use define_scale;