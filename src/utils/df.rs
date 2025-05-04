use polars::prelude::*;

// // downcast_ref is important
// pub fn vec_from_column(
//     column: Column,
// ) -> Vec<DataType>{
//         let series = column.as_series().unwrap();
//         // let chunk_iterator = series.into_chunks();
//         let dtype = series.dtype();
//         let mut result_vec: Vec<DataType> = Vec::with_capacity(series.len()); // Pre-allocate

//         // manually call casting functions.
//         match dtype {
//             DataType::Float32 => {
//                 let typed_series = series.f32().unwrap();
//                 for a in typed_series.iter() {

//                 }
//             }

//         }

// }

// return a chunk array which you can iterate over via a for loop
// pub fn iterable_chunk_array_from_column(
//     column: Column
// ) -> ChunkedArray<dyn PolarsDataType> {

// }

// NOTE: this test is just 
// to record me playing around with
// the best way of iterating through a polars series.

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;

    #[test]
    fn it_works() {
        let s0 = Column::new("days".into(), [0, 1, 2,999].as_ref());
        let s1 = Column::new("temp".into(), [22.1, 19.9, 7., 999999.9].as_ref());
        let df = DataFrame::new(vec![s0, s1]).unwrap();
        
        // let v: Vec<DataType> = Vec::new();
        let column = df.column("days").unwrap();
        let series = column.as_series().unwrap().clone();
        for (idx, value) in series.iter().enumerate() {
            match value {
                AnyValue::String(_) => {
                    println!("{}", value);
                }
                AnyValue::Int32(_) | AnyValue::Int64(_) => {
                    println!("{}", value.to_string());
                }
                AnyValue::Float32(_) | AnyValue::Float64(_) => {
                    println!("{}", value.to_string());
                }
                _ => {}
            }
        }

        // let vec = vec_from_column(*column);
        // assert!(vec == vec![0, 1, 2,999])
        assert!(1==0)
    }
}