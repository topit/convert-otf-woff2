fn main() {
    // WOFF1 编译（默认启用）
    cc::Build::new()
        .include("vendor/sfnt2woff/source/woff")
        .file("vendor/sfnt2woff/source/woff/woff.c")
        .compiler("emcc")  // 使用 Emscripten 编译器
        .flag("-O3")
        .flag("-sSIDE_MODULE=1")  // 生成动态库
        .flag("-sUSE_ZLIB=1")     // 启用 zlib 支持
        .static_flag(true)
        .warnings(false)
        .compile("libsfnt2woff.a");

    println!("cargo:rustc-link-lib=z");

    // WOFF2 编译（默认启用）
    cc::Build::new()
        .cpp(true)
        .compiler("emcc")
        .flag("-std=c++11")
        .flag("-O3")
        .flag("-sSIDE_MODULE=1")
        .flag("-sUSE_ZLIB=1")
        .include("vendor/woff2/source/include")
        .include("vendor/woff2/wrapper")
        .file("vendor/woff2/wrapper/woff2.cpp")
        .static_flag(true)
        .warnings(false)
        .compile("libwoff2wrapper.a");

    cc::Build::new()
        .cpp(true)
        .compiler("emcc")
        .flag("-std=c++11")
        .flag("-O3")
        .flag("-sSIDE_MODULE=1")
        .flag("-sUSE_ZLIB=1")
        .include("vendor/woff2/source/include")
        .include("vendor/brotli/source/c/include")
        .files(&[
            "vendor/woff2/source/src/font.cc",
            "vendor/woff2/source/src/glyph.cc",
            "vendor/woff2/source/src/normalize.cc",
            "vendor/woff2/source/src/table_tags.cc",
            "vendor/woff2/source/src/transform.cc",
            "vendor/woff2/source/src/variable_length.cc",
            "vendor/woff2/source/src/woff2_common.cc",
            "vendor/woff2/source/src/woff2_dec.cc",
            "vendor/woff2/source/src/woff2_enc.cc",
            "vendor/woff2/source/src/woff2_out.cc",
        ])
        .static_flag(true)
        .warnings(false)
        .compile("libwoff2.a");

    // Brotli 编译（默认启用）
    cc::Build::new()
        .compiler("emcc")
        .flag("-O3")
        .flag("-sSIDE_MODULE=1")
        .flag("-sUSE_ZLIB=1")
        .include("vendor/brotli/source/c/include")
        .files(&[
            "vendor/brotli/source/c/common/constants.c",
            "vendor/brotli/source/c/common/context.c",
            "vendor/brotli/source/c/common/dictionary.c",
            "vendor/brotli/source/c/common/platform.c",
            "vendor/brotli/source/c/common/shared_dictionary.c",
            "vendor/brotli/source/c/common/transform.c",
            "vendor/brotli/source/c/dec/bit_reader.c",
            "vendor/brotli/source/c/dec/decode.c",
            "vendor/brotli/source/c/dec/huffman.c",
            "vendor/brotli/source/c/dec/state.c",
            "vendor/brotli/source/c/enc/backward_references.c",
            "vendor/brotli/source/c/enc/backward_references_hq.c",
            "vendor/brotli/source/c/enc/bit_cost.c",
            "vendor/brotli/source/c/enc/block_splitter.c",
            "vendor/brotli/source/c/enc/brotli_bit_stream.c",
            "vendor/brotli/source/c/enc/cluster.c",
            "vendor/brotli/source/c/enc/command.c",
            "vendor/brotli/source/c/enc/compound_dictionary.c",
            "vendor/brotli/source/c/enc/compress_fragment.c",
            "vendor/brotli/source/c/enc/compress_fragment_two_pass.c",
            "vendor/brotli/source/c/enc/dictionary_hash.c",
            "vendor/brotli/source/c/enc/encode.c",
            "vendor/brotli/source/c/enc/encoder_dict.c",
            "vendor/brotli/source/c/enc/entropy_encode.c",
            "vendor/brotli/source/c/enc/fast_log.c",
            "vendor/brotli/source/c/enc/histogram.c",
            "vendor/brotli/source/c/enc/literal_cost.c",
            "vendor/brotli/source/c/enc/memory.c",
            "vendor/brotli/source/c/enc/metablock.c",
            "vendor/brotli/source/c/enc/static_dict.c",
            "vendor/brotli/source/c/enc/utf8_util.c",
        ])
        .static_flag(true)
        .warnings(false)
        .compile("libbrotli.a");
}