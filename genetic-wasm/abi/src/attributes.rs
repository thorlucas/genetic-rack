use serde::Serialize; use ts_rs::TS;

macro_rules! attr_desc {
    ($attr:ident : 4 $($t:tt)*) => { attr_desc!( $($t)* ($attr "4") ); };
    ($attr:ident : 3 $($t:tt)*) => { attr_desc!( $($t)* ($attr "3") ); };
    ($attr:ident : 2 $($t:tt)*) => { attr_desc!( $($t)* ($attr "2") ); };
    ($attr:ident : 1 $($t:tt)*) => { attr_desc!( $($t)* ($attr "1") ); };
    
    ($(($attr:ident $size:literal))*) => { attr_desc!(@parse {$(($attr $size))*} {}/{}); };

    // Tail case
    (@parse {} {$($acc:tt)*}/{$($names:tt)*}) => {
        #[derive(Serialize, TS)]
        #[serde(rename_all="lowercase")]
        #[serde(tag="type")]
        pub enum AttributeDescriptor {
            $($acc)*
        }

        #[derive(TS, Serialize)]
        #[serde(rename_all="lowercase")]
        pub enum Attribute {
            $($names)*
        }
    };

    // Add to the accumulator
    (@parse {($attr:ident $size:literal) $($tail:tt)*} {$($acc:tt)*}/{$($names:tt)*}) => {
        attr_desc!(@parse {$($tail)*} {
            $($acc)*
            $attr {
                #[ts(type=$size)]
                size: usize,
            },
        }/{
            $($names)*
            $attr,
        });
    }
}

attr_desc! {
    Position: 3
    Momentum: 3
    Mass: 1
}
