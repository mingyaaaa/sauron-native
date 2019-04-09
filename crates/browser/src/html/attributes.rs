//! https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
//!
use vdom::builder::attr;
use vdom::builder::Attribute;
use vdom::Value;

macro_rules! declare_attributes {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<'a, V>(v: V) -> Attribute<'a>
                where V: Into<Value>
                {
                    attr(stringify!($name), v)
                }
         )*
    };
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<'a, V>(v: V) -> Attribute<'a>
                where V: Into<Value>
                {
                    attr($attribute, v)
                }
         )*
    }
}

declare_attributes! {
    accesskey;

    autocapitalize;

    class;

    contextmenu;

    draggable;

    dropzone;

    hidden;

    id;

    inputmode;

    is;

    itemid;

    itemprop;

    itemref;

    itemscope;

    itemtype;

    lang;

    slot;

    spellcheck;

    style;

    tabindex;

    title;

    translate;


}

// special case for type attribute, since type is a rust keyword
declare_attributes! {
    r#type => "type";
}

// common attributes
declare_attributes! {
    value;
    key;
    placeholder;
}

// svg attributes

declare_attributes! {
    cx;
    cy;
    r;
    xmlns;
}