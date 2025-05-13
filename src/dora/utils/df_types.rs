// prerequisites for using these macros:
//      use polars::prelude::*;

// macros for AnyValue
// polars AnyValue is the type
// enum of the actual value itself
// when you iterate over it
#[macro_export]
macro_rules! any_int {
    () => {
        AnyValue::Int8(_) | AnyValue::Int16(_) | AnyValue::Int32(_) | AnyValue::Int64(_)
    };
}

#[macro_export]
macro_rules! any_uint {
    () => {
        AnyValue::UInt8(_) | AnyValue::UInt16(_) | AnyValue::UInt32(_) | AnyValue::UInt64(_)
    };
}

#[macro_export]
macro_rules! any_float {
    () => {
        AnyValue::Float32(_) | AnyValue::Float64(_)
    };
}

#[macro_export]
macro_rules! any_string {
    () => {
        AnyValue::String(_) | AnyValue::StringOwned(_)
    };
}

#[macro_export]
macro_rules! any_datetime {
    () => {
        AnyValue::Date(_)
            | AnyValue::Datetime(_, _, _)
            | AnyValue::DatetimeOwned(_, _, _)
            | AnyValue::Duration(_, _)
            | AnyValue::Time(_)
    };
}

// macros for DataType
// polars DataType is the type
// enum for the series/dataframe
// a I32DataType will map to I32 in anyvalue.
