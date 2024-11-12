use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/proto/flow.proto", "src/proto/packet.proto"], &["src/proto/"])?;
    Ok(())
}