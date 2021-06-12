// build.rs

fn main() {
    compile_c_src();
}

fn compile_c_src() {
    println!("cargo:rerun-if-changed=src/c/base.h");
    println!("cargo:rerun-if-changed=src/c/mem_chr.h");
    println!("cargo:rerun-if-changed=src/c/mem_rchr.h");
    //
    println!("cargo:rerun-if-changed=src/c/mem/mem_cpy.c");
    println!("cargo:rerun-if-changed=src/c/mem/mem_set.c");
    println!("cargo:rerun-if-changed=src/c/mem/mem_chr.c");
    println!("cargo:rerun-if-changed=src/c/mem/mem_rchr.c");
    println!("cargo:rerun-if-changed=src/c/mem/mem_cmp.c");
    println!("cargo:rerun-if-changed=src/c/mem/mem_eq.c");
    println!("cargo:rerun-if-changed=src/c/mem/mem_mem.c");
    println!("cargo:rerun-if-changed=src/c/mem/mem_rmem.c");
    println!("cargo:rerun-if-changed=src/c/arch/x86/x86_chr.c");
    println!("cargo:rerun-if-changed=src/c/arch/x86/x86_rchr.c");
    //
    cc::Build::new()
        .file("src/c/mem/mem_cpy.c")
        .file("src/c/mem/mem_set.c")
        .flag_if_supported("-fno-tree-loop-distribute-patterns")
        .flag_if_supported("-Wno-unused-function")
        //.shared_flag(true)
        .static_flag(true)
        .compile("libmemx_c_1.a");
    //
    cc::Build::new()
        .file("src/c/mem/mem_cmp.c")
        .file("src/c/mem/mem_eq.c")
        .file("src/c/mem/mem_chr.c")
        .file("src/c/mem/mem_rchr.c")
        .file("src/c/mem/mem_mem.c")
        .file("src/c/mem/mem_rmem.c")
        //.flag_if_supported("-fno-tree-loop-distribute-patterns")
        .flag_if_supported("-Wno-unused-function")
        //.shared_flag(true)
        .static_flag(true)
        .compile("libmemx_c_2.a");
    //
    cc::Build::new()
        .file("src/c/arch/x86/x86_chr.c")
        .file("src/c/arch/x86/x86_rchr.c")
        //.flag_if_supported("-fno-tree-loop-distribute-patterns")
        .flag_if_supported("-Wno-unused-function")
        //.shared_flag(true)
        .static_flag(true)
        .compile("libmemx_c_simd.a");
}
