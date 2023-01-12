// Copyright (c) 2022 Swift Navigation
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
