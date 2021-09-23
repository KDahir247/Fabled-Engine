// Audio Death Is Just Another Path.mp3 Made by Otto Halmén
// https://opengameart.org/content/death-is-just-another-path

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();


        let mut a = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        a.push_str(cargo_dir);
        a.push_str("/Death Is Just Another Path.mp3");
        let file = std::fs::File::open(a.as_str()).unwrap();

        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).unwrap());
        sink.sleep_until_end();
    }
}
