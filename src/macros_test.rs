use std::{
    io::Write,
    sync::{Arc, RwLock},
};

use crate as cargo_build;

#[test]
fn change_output_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rerun_if_changed!();
    cargo_build::rerun_if_changed!("LICENSE.md"; "README.md" ; );

    {
        let out = vec_out.0.read().expect("Unable to aquire Read lock");
        let out: &str = str::from_utf8(&out).unwrap();

        assert_eq!(
            out,
            "\
                cargo::rerun-if-changed=LICENSE.md\n\
                cargo::rerun-if-changed=README.md\n"
        );
    }

    // Reset CARGO_BUILD_OUT and try again
    vec_out
        .0
        .write()
        .expect("Unable to aquire Write lock")
        .clear();

    cargo_build::build_out::reset();

    cargo_build::rerun_if_changed(["LICENSE.md"]);

    let out: &[u8] = &vec_out.0.read().expect("Unable to aquire Read lock");

    assert_eq!(out, b"");
}

#[test]
fn rerun_if_changed_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rerun_if_changed(["LICENSE.md", "README.md"]);

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rerun-if-changed=LICENSE.md\n\
                cargo::rerun-if-changed=README.md\n"
    );
}

#[test]
fn rerun_if_env_changed_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    // cargo_build::rerun_if_env_changed!("VAR1", "VAR2");

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rerun-if-env-changed=VAR1\n\
                cargo::rerun-if-env-changed=VAR2\n"
    );
}

#[test]
fn rustc_link_arg_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_link_arg(["-mlongcalls", "-ffunction-sections", "-Wl,--cref"]);

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rustc-link-arg=-mlongcalls\n\
                cargo::rustc-link-arg=-ffunction-sections\n\
                cargo::rustc-link-arg=-Wl,--cref\n"
    );
}

#[test]
fn rustc_link_arg_cdylib_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_link_arg_cdylib(["-mlongcalls", "-ffunction-sections", "-Wl,--cref"]);

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rustc-link-arg-cdylib=-mlongcalls\n\
                cargo::rustc-link-arg-cdylib=-ffunction-sections\n\
                cargo::rustc-link-arg-cdylib=-Wl,--cref\n"
    );
}

#[test]
fn rustc_link_arg_bin_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_link_arg_bin!("server": "-Wl,--cref");

    cargo_build::rustc_link_arg_bin!(
        "client":
            "-mlongcalls"; 
            "-ffunction-sections"; 
            "-Wl,--cref"
    );

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rustc-link-arg-bin=server=-Wl,--cref\n\
                cargo::rustc-link-arg-bin=client=-mlongcalls\n\
                cargo::rustc-link-arg-bin=client=-ffunction-sections\n\
                cargo::rustc-link-arg-bin=client=-Wl,--cref\n"
    );
}

#[test]
fn rustc_link_arg_bins_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_link_arg_bins(["-mlongcalls", "-ffunction-sections", "-Wl,--cref"]);

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rustc-link-arg-bins=-mlongcalls\n\
                cargo::rustc-link-arg-bins=-ffunction-sections\n\
                cargo::rustc-link-arg-bins=-Wl,--cref\n"
    );
}

#[test]
fn rustc_link_arg_tests_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_link_arg_tests(["-mlongcalls", "-ffunction-sections", "-Wl,--cref"]);

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rustc-link-arg-tests=-mlongcalls\n\
                cargo::rustc-link-arg-tests=-ffunction-sections\n\
                cargo::rustc-link-arg-tests=-Wl,--cref\n"
    );
}

#[test]
fn rustc_link_arg_examples_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_link_arg_examples(["-mlongcalls", "-ffunction-sections", "-Wl,--cref"]);

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rustc-link-arg-examples=-mlongcalls\n\
                cargo::rustc-link-arg-examples=-ffunction-sections\n\
                cargo::rustc-link-arg-examples=-Wl,--cref\n"
    );
}

#[test]
fn rustc_link_arg_benches_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_link_arg_benches(["-mlongcalls", "-ffunction-sections", "-Wl,--cref"]);

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rustc-link-arg-benches=-mlongcalls\n\
                cargo::rustc-link-arg-benches=-ffunction-sections\n\
                cargo::rustc-link-arg-benches=-Wl,--cref\n"
    );
}


#[test]
fn rustc_link_lib_test_complex() {

    let vec_out = TestWriteVecHandle::new();
    cargo_build::build_out::set(vec_out.clone());

    let rename = "renamed_lib";

    cargo_build::rustc_link_lib!(
        static: "+whole-archive", "+verbatim", "+bundle", "+bundle" = 
                    "ff:{}", rename;
                    "ff:{}", rename;
                    "ff:{}", rename;
    );
    
    cargo_build::rustc_link_lib!(
        static: "+whole-archive", "+verbatim", "+bundle" = 
                    "ff:{}", rename;
                    "ff:{}", rename;
                    "ff:{}", rename
    );
    
    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rustc-link-lib=static:+whole-archive,+verbatim,+bundle,+bundle=ff:renamed_lib\n\
                cargo::rustc-link-lib=static:+whole-archive,+verbatim,+bundle,+bundle=ff:renamed_lib\n\
                cargo::rustc-link-lib=static:+whole-archive,+verbatim,+bundle,+bundle=ff:renamed_lib\n\
                cargo::rustc-link-lib=static:+whole-archive,+verbatim,+bundle=ff:renamed_lib\n\
                cargo::rustc-link-lib=static:+whole-archive,+verbatim,+bundle=ff:renamed_lib\n\
                cargo::rustc-link-lib=static:+whole-archive,+verbatim,+bundle=ff:renamed_lib\n"
    );
}

#[test]
fn rustc_link_lib_test_all() {

    let vec_out = TestWriteVecHandle::new();
    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_link_lib!(static: "+whole-archive" = "foo:{}", "renamed_foo" );
    cargo_build::rustc_link_lib!(dylib: "+whole-archive" = "foo:{}", "renamed_foo" );
    cargo_build::rustc_link_lib!(framework: "+whole-archive" = "foo:{}", "renamed_foo" );
    
    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rustc-link-lib=static:+whole-archive=foo:renamed_foo\n\
                cargo::rustc-link-lib=dylib:+whole-archive=foo:renamed_foo\n\
                cargo::rustc-link-lib=framework:+whole-archive=foo:renamed_foo\n"
    );
}

#[test]
fn rustc_link_search_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_link_search(["common_libs"]);

    cargo_build::rustc_link_search(["native=libs", "framework=mac_os_libs"]);

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rustc-link-search=common_libs\n\
                cargo::rustc-link-search=native=libs\n\
                cargo::rustc-link-search=framework=mac_os_libs\n"
    );
}

#[test]
fn rustc_flags_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_flags(["-L libs", "-L common_libs"]);

    cargo_build::rustc_flags(["-l ffi", "-l ncursesw", "-l stdc++", "-l z"]);

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "\
                cargo::rustc-flags=-L libs\n\
                cargo::rustc-flags=-L common_libs\n\
                cargo::rustc-flags=-l ffi\n\
                cargo::rustc-flags=-l ncursesw\n\
                cargo::rustc-flags=-l stdc++\n\
                cargo::rustc-flags=-l z\n"
    );
}

#[test]
fn rustc_cfg_test_no_value() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_cfg("api_v1");

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(out, "cargo::rustc-cfg=api_v1\n");
}

#[test]
fn rustc_cfg_test_value() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_cfg(("api_version", "1"));

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(out, "cargo::rustc-cfg=api_version=\"1\"\n");
}

#[test]
fn rustc_check_cfg_test_no_values() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_check_cfg("api_version", std::iter::empty::<&str>());

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(out, "cargo::rustc-check-cfg=cfg(api_version)\n");
}


#[test]
fn rustc_check_cfg_test_single_value() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_check_cfg("api_version", ["1"]);

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "cargo::rustc-check-cfg=cfg(api_version, values(\"1\"))\n"
    );
}

#[test]
fn rustc_check_cfg_test_many_values() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_check_cfg("api_version", ["1", "2", "3"]);

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(
        out,
        "cargo::rustc-check-cfg=cfg(api_version, values(\"1\", \"2\", \"3\"))\n"
    );
}

#[test]
fn rustc_env_test() {
    let vec_out = TestWriteVecHandle::new();

    cargo_build::build_out::set(vec_out.clone());

    cargo_build::rustc_env("GIT_HASH", "1234");

    let out = vec_out.0.read().expect("Unable to aquire Read lock");
    let out: &str = str::from_utf8(&out).unwrap();

    assert_eq!(out, "cargo::rustc-env=GIT_HASH=1234\n");
}

struct TestWriteVecHandle(Arc<RwLock<Vec<u8>>>);

impl TestWriteVecHandle {
    fn new() -> Self {
        Self(Arc::new(RwLock::new(Vec::new())))
    }
}

impl Clone for TestWriteVecHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Write for TestWriteVecHandle {
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
        self.0
            .write()
            .expect("Unable to aquire Write lock")
            .write(buf)
    }

    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
}
