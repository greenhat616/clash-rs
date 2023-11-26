use prost_build::Config;
use std::io::Result;

fn main() -> Result<()> {
    Config::new()
        .out_dir(
            "src/proxy/vless/vless_impl/proto",
        )
        .compile_protos(&["src/proxy/vless/vless_impl/proto/vless.proto"], &["src"])?;
    Ok(())
}
