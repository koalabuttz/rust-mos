use crate::spec::{Arch, PanicStrategy, Target, TargetMetadata, TargetOptions};

pub(crate) fn target() -> Target {
    let llvm_args = &[
        "--force-precise-rotation-cost",
        "--jump-inst-cost=6",
        "--force-loop-cold-block",
        "--phi-node-folding-threshold=0",
        "--two-entry-phi-node-folding-threshold=0",
        "--align-large-globals=false",
        "--disable-spill-hoist",
    ];

    let llvm_args = llvm_args.iter().map(|opt| opt.to_string().into()).collect();

    Target {
        arch: Arch::Mos,
        metadata: TargetMetadata {
            description: Some("MOS 6502".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        llvm_target: "mos-unknown-none".into(),
        pointer_width: 16,
        data_layout: "e-m:e-p:16:8-p1:8:8-i16:8-i32:8-i64:8-f32:8-f64:8-a:8-Fi8-n8".into(),
        options: TargetOptions {
            c_int_width: 16,
            cpu: "mos6502".into(),
            executables: true,
            singlethread: true,
            atomic_cas: false,
            min_atomic_width: Some(8),
            max_atomic_width: Some(8),
            disable_redzone: true,
            panic_strategy: PanicStrategy::Abort,
            linker: Some("mos-clang".into()),
            no_default_libraries: false,
            requires_lto: true,
            supports_stack_protector: false,
            trap_unreachable: false,
            llvm_args,
            ..Default::default()
        },
    }
}
