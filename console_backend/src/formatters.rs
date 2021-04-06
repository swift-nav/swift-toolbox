use paste::paste;

/// Float Formatter for file output.
/// Taken from ICBINS/src/formatters.rs.
macro_rules! make_float_formatter {
    ($deci_count:literal) => {
        paste! {
            pub mod [<float_formatter_ $deci_count>] {
                use serde::{self, Serializer};

                pub fn serialize<S>(
                    f_value: &Option<f64>,
                    serializer: S,
                ) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    if let Some(f_value) = f_value {
                        let s = format!(std::concat!("{:.0", $deci_count, "}"), f_value);
                        serializer.serialize_str(&s)
                    } else {
                        serializer.serialize_none()
                    }
                }
            }
        }
    };
}

make_float_formatter!(0);
make_float_formatter!(1);
make_float_formatter!(2);
make_float_formatter!(3);
make_float_formatter!(4);
make_float_formatter!(6);
make_float_formatter!(10);
