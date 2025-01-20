use std::env;
use std::path::PathBuf;

fn main() {
    // PCL configuration
    let pcl_path = env::var("PCL_DIR").unwrap_or_else(|_| "/usr/local".to_string());
    println!("cargo:rustc-link-search=native={}/lib", pcl_path);
    println!("cargo:rustc-link-lib=pcl_common");
    println!("cargo:rustc-link-lib=pcl_filters");
    
    // Open3D configuration
    let open3d_path = env::var("OPEN3D_DIR").unwrap_or_else(|_| "/usr/local".to_string());
    println!("cargo:rustc-link-search=native={}/lib", open3d_path);
    println!("cargo:rustc-link-lib=Open3D");

    // Platform-specific optimizations
    #[cfg(target_arch = "x86_64")]
    println!("cargo:rustc-cfg=simd_x86");
    
    println!("cargo:rerun-if-env-changed=PCL_DIR");
    println!("cargo:rerun-if-env-changed=OPEN3D_DIR");
}