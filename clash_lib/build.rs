use prost_build::Config;
use std::io::Result;

fn main() -> Result<()> {
    Config::new()
        .out_dir(
            "src/proxy/vless/proto",
        )
        .compile_protos(&["src/proxy/vless/proto/vless.proto"], &["src"])?;
    Ok(())
}
