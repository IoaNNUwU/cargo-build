fn main() {
    cargo_build::rustc_link_lib!(static: "+bundle", "+bundle" = "clo");
}
