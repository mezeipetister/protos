fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().out_dir("src/proto/").compile(
        &["src/proto/hello.proto", "src/proto/hello2.proto"],
        &["src/proto"],
    )?;
    Ok(())
}
