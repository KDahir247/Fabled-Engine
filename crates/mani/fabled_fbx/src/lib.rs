mod container;
mod loader;

pub use container::*;
pub use loader::*;

#[cfg(test)]
mod tests {
    use crate::load;
    use fbxcel::low::v7400::{AttributeType, AttributeValue};
    use fbxcel::pull_parser::reader::SeekableSource;
    use fbxcel::pull_parser::v7400::attribute::loaders::DirectLoader;
    use fbxcel::pull_parser::v7400::{Attributes, Event};
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn print_obj() {
        load("C:/Users/kdahi/Downloads/goultard-chibi/source/GoultardtoDecimated.fbx");
    }

    #[test]
    fn it_works() {
        let file = std::fs::File::open("C:/Users/kdahi/Downloads/untitled.fbx").unwrap();

        let reader = std::io::BufReader::new(file);

        match fbxcel::pull_parser::any::from_seekable_reader(reader) {
            Ok(fbxcel::pull_parser::any::AnyParser::V7400(mut parser)) => {
                parser.set_warning_handler(|x, p| {
                    eprintln!("Warning : {} = {:?}", x, p);

                    Ok(())
                });

                loop {
                    match parser.next_event().unwrap() {
                        Event::StartNode(start) => {
                            let mut a = start.attributes();

                            while let Some(attr) = a.load_next_buffered(DirectLoader).unwrap() {
                                match attr {
                                    AttributeValue::Bool(bool) => {
                                        println!("Boolean: {}", bool);
                                    }
                                    AttributeValue::I16(i16) => {
                                        println!("i16: {}", i16);
                                    }
                                    AttributeValue::I32(i32) => {
                                        println!("i32: {}", i32);
                                    }
                                    AttributeValue::I64(i64) => {
                                        println!("i64: {}", i64);
                                    }
                                    AttributeValue::F32(f32) => {
                                        println!("f32: {}", f32);
                                    }
                                    AttributeValue::F64(f64) => {
                                        println!("f64: {}", f64);
                                    }
                                    AttributeValue::ArrBool(arr_bool) => {
                                        println!("array boolean: {:?}", arr_bool);
                                    }
                                    AttributeValue::ArrI32(arr_i32) => {
                                        println!("array i32: {:?}", arr_i32);
                                    }
                                    AttributeValue::ArrI64(arr_i64) => {
                                        println!("array i64: {:?}", arr_i64);
                                    }
                                    AttributeValue::ArrF32(arr_f32) => {
                                        println!("array f32: {:?}", arr_f32);
                                    }
                                    AttributeValue::ArrF64(array_f64) => {
                                        println!("array f64: {:?}", array_f64);
                                    }
                                    AttributeValue::String(string) => {
                                        println!("string: {:?}", string);
                                    }
                                    AttributeValue::Binary(bin) => {
                                        println!("binary: {:?}", bin);
                                    }
                                }
                            }
                        }
                        Event::EndNode => {}
                        Event::EndFbx(e) => {
                            break;
                        }
                    }
                }
            }
            _ => {
                println!("error");
            }
        }
    }
}
