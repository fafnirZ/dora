use polars::prelude::*;

// Helper function to convert a Polars Series to a Vec of a specific type.
// This uses a macro for code clarity and to avoid repetitive code.
macro_rules! series_to_vec {
    ($series:expr, $type:ty) => {{
        let chunk_iterator = $series.iter_chunks();
        let mut result_vec: Vec<$type> = Vec::with_capacity($series.len()); // Pre-allocate
        for chunk in chunk_iterator {
            let typed_chunk = chunk.as_any().downcast_ref::<ChunkedArray<$type>>().unwrap();
            result_vec.extend_from_slice(typed_chunk.into_vec());
        }
        result_vec
    }};
}


pub fn convert_column_to_vector<T: PolarsDataType>(
    column: Column,
) -> Vec<T::RustType>
{
    let series = column.as_series().unwrap(); // Convert Column to Series
    let data_type = series.dtype();

    // Match on the data type to determine the concrete Rust type and use the macro.
    match data_type {
        &DataType::Boolean => {
            let vec: Vec<bool> = series_to_vec!(series, BooleanType);
            vec.into_iter().map(|v| v as T::RustType).collect()
        }
        &DataType::Int8 => {
            let vec: Vec<i8> = series_to_vec!(series, Int8Type);
            vec.into_iter().map(|v| v as T::RustType).collect()
        }
        &DataType::Int16 => {
            let vec: Vec<i16> = series_to_vec!(series, Int16Type);
             vec.into_iter().map(|v| v as T::RustType).collect()
        }
        &DataType::Int32 => {
            let vec: Vec<i32> = series_to_vec!(series, Int32Type);
             vec.into_iter().map(|v| v as T::RustType).collect()
        }
        &DataType::Int64 => {
            let vec: Vec<i64> = series_to_vec!(series, Int64Type);
             vec.into_iter().map(|v| v as T::RustType).collect()
        }
        &DataType::UInt8 => {
            let vec: Vec<u8> = series_to_vec!(series, UInt8Type);
             vec.into_iter().map(|v| v as T::RustType).collect()
        }
        &DataType::UInt16 => {
            let vec: Vec<u16> = series_to_vec!(series, UInt16Type);
             vec.into_iter().map(|v| v as T::RustType).collect()
        }
        &DataType::UInt32 => {
            let vec: Vec<u32> = series_to_vec!(series, UInt32Type);
             vec.into_iter().map(|v| v as T::RustType).collect()
        }
        &DataType::UInt64 => {
            let vec: Vec<u64> = series_to_vec!(series, UInt64Type);
             vec.into_iter().map(|v| v as T::RustType).collect()
        }
        &DataType::Float32 => {
            let vec: Vec<f32> = series_to_vec!(series, Float32Type);
             vec.into_iter().map(|v| v as T::RustType).collect()
        }
        &DataType::Float64 => {
            let vec: Vec<f64> = series_to_vec!(series, Float64Type);
             vec.into_iter().map(|v| v as T::RustType).collect()
        }
        &DataType::Utf8 => {
            let vec: Vec<String> = series_to_vec!(series, Utf8Type);
             vec.into_iter().map(|v| v.as_str() as T::RustType).collect()
        }
        &DataType::Null => {
            // Handle the Null case.  Return an empty vector.
            Vec::new()
        }
        _ => {
            // Return an error for unsupported data types.
            PolarsError::ComputeError(
                format!("Unsupported data type: {:?}", data_type).into(),
            )
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s0 = Column::new("days".into(), [0, 1, 2,999].as_ref());
        let s1 = Column::new("temp".into(), [22.1, 19.9, 7., 999999.9].as_ref());
        let df = DataFrame::new(vec![s0, s1]).unwrap();
        
        let column = df.column("days").unwrap();
        let vec = iter_from_column(*column).collect();
        assert!(vec == vec![0, 1, 2,999])
    }
}