fn main() {
    build::main();
}

#[allow(deprecated)]
mod build {
    extern crate gcc;

    use std::env;

    pub fn main() {
        let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set");

        let mut config = gcc::Config::new();

        config
            .file("lame-3.99.5/libmp3lame/bitstream.c")
            .file("lame-3.99.5/libmp3lame/encoder.c")
            .file("lame-3.99.5/libmp3lame/fft.c")
            .file("lame-3.99.5/libmp3lame/gain_analysis.c")
            .file("lame-3.99.5/libmp3lame/id3tag.c")
            .file("lame-3.99.5/libmp3lame/lame.c")
            .file("lame-3.99.5/libmp3lame/mpglib_interface.c") //Perhaps remove
            .file("lame-3.99.5/libmp3lame/newmdct.c")
            .file("lame-3.99.5/libmp3lame/presets.c")
            .file("lame-3.99.5/libmp3lame/psymodel.c")
            .file("lame-3.99.5/libmp3lame/quantize_pvt.c")
            .file("lame-3.99.5/libmp3lame/quantize.c")
            .file("lame-3.99.5/libmp3lame/reservoir.c")
            .file("lame-3.99.5/libmp3lame/set_get.c")
            .file("lame-3.99.5/libmp3lame/tables.c")
            .file("lame-3.99.5/libmp3lame/takehiro.c")
            .file("lame-3.99.5/libmp3lame/util.c")
            .file("lame-3.99.5/libmp3lame/vbrquantize.c")
            .file("lame-3.99.5/libmp3lame/VbrTag.c")
            .file("lame-3.99.5/libmp3lame/version.c")
            .file("lame-3.99.5/mpglib/common.c")
            .file("lame-3.99.5/mpglib/dct64_i386.c")
            .file("lame-3.99.5/mpglib/decode_i386.c")
            .file("lame-3.99.5/mpglib/interface.c")
            .file("lame-3.99.5/mpglib/layer1.c")
            .file("lame-3.99.5/mpglib/layer2.c")
            .file("lame-3.99.5/mpglib/layer3.c")
            .file("lame-3.99.5/mpglib/tabinit.c")
            .include("lame-3.99.5/include")
            .include("lame-3.99.5/libmp3lame")
            .include("lame-3.99.5/mpglib")
            .define("HAVE_CONFIG_H", None)
            .define("PIC", None)
            .define("HAVE_MPGLIB", Some("1"));

        if target_os == "windows" {
            config
                .define("TAKEHIRO_IEEE754_HACK", None)
                .define("FLOAT8", Some("float"))
                .define("REAL_IS_FLOAT", Some("1"))
                .define("BS_FORMAT", Some("BINARY"));
        }

        let os_config_dir = match &*target_os {
            "linux" => "linux",
            "macos" => "mac",
            "windows" => "win",
            os => panic!("unsupported os {}", os),
        };

        let arch_config_dir = match &*target_arch {
            "x86" => "ia32",
            "x86_64" => "x64",
            "arm" => "arm",
            arch => panic!("unsupported arch {}", arch),
        };

        // until we know how to link this correctly on windows we will use mock for win just to compile
        // config.include(format!("lame-config/{}/{}", os_config_dir, arch_config_dir));
        // config.compile("libmp3lame.a");

        match &*target_os {
            "linux" | "macos" => {
                config.include(format!("lame-config/{}/{}", os_config_dir, arch_config_dir));
                config.compile("libmp3lame.a");
            }
            "windows" => {
                // do nothing
            }
            _ => unreachable!(),
        };

        //seems to be working fine without it, whatever is build above is probably linked by default
        //let out_dir = env::var("OUT_DIR").unwrap();
        //println!("cargo:rustc-link-search=native={}", out_dir);
        //println!("cargo:rustc-link-lib=static=mp3lame");
    }
}
