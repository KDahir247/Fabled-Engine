#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let a= core_affinity::get_core_ids();

        println!("{:?}", a);
    }
}
