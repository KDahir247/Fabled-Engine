mod output_config;

pub use output_config::*;


#[cfg(test)]
mod test {


    #[test]
    fn test() {
        let a = std::mem::size_of::<cpal::Device>();
        println!("{}", a);
    }
}
