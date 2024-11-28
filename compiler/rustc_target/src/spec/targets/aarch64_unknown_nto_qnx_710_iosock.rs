use crate::spec::{Cc, LinkerFlavor, Lld, Target, TargetOptions};

pub fn target() -> Target {
    let mut target = super::aarch64_unknown_nto_qnx_710::target();
    target.options.env = "nto71_iosock".into();
    target.options.pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        &["-Vgcc_ntoaarch64le_cxx", get_iosock_param()],
    );
    target
}

// When using `io-sock` on QNX, we must add a search path for the linker so
// that it prefers the io-sock version.
// More information:
// https://www.qnx.com/developers/docs/7.1/index.html#com.qnx.doc.neutrino.io_sock/topic/migrate_app.html
fn get_iosock_param() -> &'static str {
    let target_dir = std::env::var("QNX_TARGET").expect("Environment variable QNX_TARGET is set");
    let linker_param = format!("-L{target_dir}/aarch64le/io-sock/lib");
    
    linker_param.leak()
}
