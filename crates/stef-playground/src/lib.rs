#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

mod sample {
    include!(concat!(env!("OUT_DIR"), "/sample.rs"));
}

mod schemas {
    mod alias_basic {
        include!(concat!(env!("OUT_DIR"), "/alias_basic.rs"));
    }

    mod attribute_multi {
        include!(concat!(env!("OUT_DIR"), "/attribute_multi.rs"));
    }

    mod attribute_single {
        include!(concat!(env!("OUT_DIR"), "/attribute_single.rs"));
    }

    mod attribute_unit {
        include!(concat!(env!("OUT_DIR"), "/attribute_unit.rs"));
    }

    mod attributes_min_ws {
        include!(concat!(env!("OUT_DIR"), "/attributes_min_ws.rs"));
    }

    mod attributes {
        include!(concat!(env!("OUT_DIR"), "/attributes.rs"));
    }

    mod const_basic {
        include!(concat!(env!("OUT_DIR"), "/const_basic.rs"));
    }

    mod const_string {
        include!(concat!(env!("OUT_DIR"), "/const_string.rs"));
    }

    mod enum_basic {
        include!(concat!(env!("OUT_DIR"), "/enum_basic.rs"));
    }

    mod enum_generics {
        include!(concat!(env!("OUT_DIR"), "/enum_generics.rs"));
    }

    mod enum_many_ws {
        include!(concat!(env!("OUT_DIR"), "/enum_many_ws.rs"));
    }

    mod enum_min_ws {
        include!(concat!(env!("OUT_DIR"), "/enum_min_ws.rs"));
    }

    // TODO: implement imports
    // mod import_basic {
    //     include!(concat!(env!("OUT_DIR"), "/import_basic.rs"));
    // }

    mod module_basic {
        include!(concat!(env!("OUT_DIR"), "/module_basic.rs"));
    }

    mod schema_basic {
        include!(concat!(env!("OUT_DIR"), "/schema_basic.rs"));
    }

    mod struct_basic {
        include!(concat!(env!("OUT_DIR"), "/struct_basic.rs"));
    }

    mod struct_generics {
        include!(concat!(env!("OUT_DIR"), "/struct_generics.rs"));
    }

    mod struct_many_ws {
        include!(concat!(env!("OUT_DIR"), "/struct_many_ws.rs"));
    }

    mod struct_min_ws {
        include!(concat!(env!("OUT_DIR"), "/struct_min_ws.rs"));
    }

    mod struct_tuple {
        include!(concat!(env!("OUT_DIR"), "/struct_tuple.rs"));
    }

    mod types_basic {
        include!(concat!(env!("OUT_DIR"), "/types_basic.rs"));
    }

    mod types_generic {
        include!(concat!(env!("OUT_DIR"), "/types_generic.rs"));
    }

    mod types_nested {
        include!(concat!(env!("OUT_DIR"), "/types_nested.rs"));
    }

    mod types_non_zero {
        include!(concat!(env!("OUT_DIR"), "/types_non_zero.rs"));
    }

    mod types_ref {
        include!(concat!(env!("OUT_DIR"), "/types_ref.rs"));
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use stef::{Decode, Encode};

    use super::sample;

    fn roundtrip<T: Debug + PartialEq + Decode + Encode>(value: &T) {
        let mut buf = Vec::new();
        value.encode(&mut buf);
        println!("{}: {buf:?}", std::any::type_name::<T>());

        let value2 = T::decode(&mut &*buf).unwrap();
        assert_eq!(*value, value2);
    }

    #[test]
    fn sample() {
        roundtrip(&sample::Sample {
            a: 5,
            b: true,
            c: ("Test".into(), -2),
        });
    }

    #[test]
    fn sample2_unit() {
        roundtrip(&sample::Sample2::Unit);
    }

    #[test]
    fn sample2_tuple() {
        roundtrip(&sample::Sample2::Tuple(7, 8));
    }

    #[test]
    fn sample2_fields() {
        roundtrip(&sample::Sample2::Fields {
            name: "this".into(),
            valid: true,
            dates: vec![
                (2023, 1, 1),
                (2023, 10, 5),
                (2023, 12, sample::CHRISTMAS_DAY),
            ],
        });
    }

    #[test]
    fn sample_gen() {
        roundtrip(&sample::gens::SampleGen {
            raw: vec![5, 6, 7, 8],
            array: [9_i16; 4],
            value: 9,
        });
    }

    #[test]
    fn sample_gen2() {
        roundtrip(&sample::gens::SampleGen2::Value(sample::SampleAlias {
            a: 50,
            b: false,
            c: (String::new(), -10),
        }));
    }

    #[test]
    fn specials_options_some() {
        roundtrip(&sample::specials::SomeOptions {
            maybe_int: Some(5),
            maybe_text: Some("hi".into()),
            maybe_tuple: Some((20, 30)),
            nested: Some(Some(8)),
            vec_maybe: vec![Some(true), None, Some(false)],
        });
    }

    #[test]
    fn specials_options_none() {
        roundtrip(&sample::specials::SomeOptions {
            maybe_int: None,
            maybe_text: None,
            maybe_tuple: None,
            nested: None,
            vec_maybe: vec![None, None],
        });
    }
}
