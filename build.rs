use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let sasquatch_dir = PathBuf::from(&manifest_dir).join("sasquatch");
    let lzma_base = sasquatch_dir
        .join("squashfs-tools")
        .join("LZMA")
        .join("lzmadaptive")
        .join("C");

    let mut build = cc::Build::new();

    build
        .cpp(true)
        .flag("-O3")
        .flag("-Wall")
        .flag("-std=c++11")
        .include(&lzma_base)
        .include(lzma_base.join("7zip"))
        .include(lzma_base.join("Common"));

    let lzma_lib = lzma_base.join("7zip").join("Compress").join("LZMA_Lib");
    let lzma_src = lzma_base.join("7zip").join("Compress").join("LZMA");
    let lz_src = lzma_base.join("7zip").join("Compress").join("LZ");
    let rangecoder_src = lzma_base.join("7zip").join("Compress").join("RangeCoder");
    let archive_common = lzma_base.join("7zip").join("Common");
    let common_src = lzma_base.join("Common");

    build.file(lzma_lib.join("ZLib.cpp"));

    build.file(lzma_src.join("LZMADecoder.cpp"));
    build.file(lzma_src.join("LZMAEncoder.cpp"));

    build.file(lz_src.join("LZInWindow.cpp"));
    build.file(lz_src.join("LZOutWindow.cpp"));

    build.file(rangecoder_src.join("RangeCoderBit.cpp"));

    build.file(archive_common.join("InBuffer.cpp"));
    build.file(archive_common.join("OutBuffer.cpp"));
    build.file(archive_common.join("StreamUtils.cpp"));
    build.file(archive_common.join("FileStreams.cpp"));

    build.file(common_src.join("Alloc.cpp"));
    build.file(common_src.join("C_FileIO.cpp"));
    build.file(common_src.join("CommandLineParser.cpp"));
    build.file(common_src.join("CRC.cpp"));
    build.file(common_src.join("String.cpp"));
    build.file(common_src.join("StringConvert.cpp"));
    build.file(common_src.join("StringToInt.cpp"));
    build.file(common_src.join("Vector.cpp"));

    build.compile("lzmadaptive");

    println!("cargo:rustc-link-lib=static=lzmadaptive");
    println!("cargo:rerun-if-changed=sasquatch/");
}
