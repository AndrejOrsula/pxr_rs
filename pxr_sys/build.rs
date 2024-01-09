/// List of libraries to link against (`-l`)
#[cfg(feature = "monolithic_lib")]
const LINK_LIBS: &[&str] = &[
    // Dependencies
    "tbb",
    "tbb_debug",
    #[cfg(feature = "alembic")]
    "Alembic",
    #[cfg(feature = "draco")]
    "draco",
    #[cfg(feature = "draco")]
    "dracodec",
    #[cfg(feature = "draco")]
    "dracoenc",
    #[cfg(feature = "materialx")]
    "MaterialXCore",
    #[cfg(feature = "materialx")]
    "MaterialXFormat",
    #[cfg(feature = "materialx")]
    "MaterialXGenGlsl",
    #[cfg(feature = "materialx")]
    "MaterialXGenMdl",
    #[cfg(feature = "materialx")]
    "MaterialXGenOsl",
    #[cfg(feature = "materialx")]
    "MaterialXGenShader",
    #[cfg(feature = "materialx")]
    "MaterialXRender",
    #[cfg(feature = "materialx")]
    "MaterialXRenderGlsl",
    #[cfg(feature = "materialx")]
    "MaterialXRenderHw",
    #[cfg(feature = "materialx")]
    "MaterialXRenderOsl",
    #[cfg(feature = "openvdb")]
    "openvdb",
    // USD library (monolithic)
    "usd_ms",
];
#[cfg(not(feature = "monolithic_lib"))]
const LINK_LIBS: &[&str] = &[
    // Dependencies
    "tbb",
    "tbb_debug",
    #[cfg(feature = "alembic")]
    "Alembic",
    #[cfg(feature = "draco")]
    "draco",
    #[cfg(feature = "draco")]
    "dracodec",
    #[cfg(feature = "draco")]
    "dracoenc",
    #[cfg(feature = "materialx")]
    "MaterialXCore",
    #[cfg(feature = "materialx")]
    "MaterialXFormat",
    #[cfg(feature = "materialx")]
    "MaterialXGenGlsl",
    #[cfg(feature = "materialx")]
    "MaterialXGenMdl",
    #[cfg(feature = "materialx")]
    "MaterialXGenOsl",
    #[cfg(feature = "materialx")]
    "MaterialXGenShader",
    #[cfg(feature = "materialx")]
    "MaterialXRender",
    #[cfg(feature = "materialx")]
    "MaterialXRenderGlsl",
    #[cfg(feature = "materialx")]
    "MaterialXRenderHw",
    #[cfg(feature = "materialx")]
    "MaterialXRenderOsl",
    // OpenUSD libraries
    "ar",
    "arch",
    "cameraUtil",
    "garch",
    "geomUtil",
    "gf",
    "glf",
    "hd",
    "hdGp",
    "hdsi",
    "hdSt",
    "hdx",
    "hf",
    "hgi",
    "hgiGL",
    "hgiInterop",
    "hio",
    "js",
    "kind",
    "ndr",
    "pcp",
    "plug",
    "pxOsd",
    "sdf",
    "sdr",
    "tf",
    "trace",
    "usd",
    "usdAppUtils",
    "usdGeom",
    "usdHydra",
    "usdImaging",
    "usdImagingGL",
    "usdLux",
    "usdMedia",
    "usdPhysics",
    "usdProc",
    "usdProcImaging",
    "usdRender",
    "usdRi",
    "usdRiImaging",
    "usdShade",
    "usdSkel",
    "usdSkelImaging",
    "usdUI",
    "usdUtils",
    "usdVol",
    "usdVolImaging",
    "vt",
    "work",
    // Extra OpenUSD libraries
    #[cfg(feature = "materialx")]
    "hdMtlx",
    #[cfg(feature = "materialx")]
    "usdBakeMtlx",
    #[cfg(feature = "materialx")]
    "usdMtlx",
    #[cfg(feature = "openvdb")]
    "hioOpenVDB",
];

/// List of libraries that are not USD libraries (not prefixed with `PXR_LIB_PREFIX`)
#[cfg(not(feature = "monolithic_lib"))]
const NON_USD_LINK_LIBS: &[&str] = &["tbb"];

/// List of link paths (`-L`), either absolute or relative to the `OPENUSD_PATH`
const LINK_PATHS: &[&str] = &["lib"];

/// List of include paths (`-I`), either absolute or relative to the `OPENUSD_PATH`
const INCLUDE_PATHS: &[&str] = &["include"];

/// Flag to enable C++11 ABI
#[cfg(not(feature = "use_cxx11_abi"))]
const USE_CXX11_ABI: &str = "0";
#[cfg(feature = "use_cxx11_abi")]
const USE_CXX11_ABI: &str = "1";

fn main() {
    // Configure rebuild triggers based on environment variables
    println!("cargo:rerun-if-env-changed=OPENUSD_PATH");

    // Locate OpenUSD (either from environment variable or vendored)
    let openusd_path = {
        if let Some(openusd_path) = std::env::var_os("OPENUSD_PATH") {
            let openusd_path = std::path::PathBuf::from(openusd_path);
            assert!(
                openusd_path.is_dir(),
                "Environment variable `OPENUSD_PATH` does not point to a valid directory: `{}`",
                openusd_path.display()
            );
            openusd_path
        } else {
            // If the `vendored` feature is not enabled, panic and instruct the user
            #[cfg(not(feature = "vendored"))]
            panic!(
                "Unable to locate OpenUSD installation. Please, specify the `OPENUSD_PATH` \
                environment variable. If you wish to automatically download and compile it \
                during the build process, enable the `vendored` feature."
            );

            // If the `vendored` feature is enabled, download and compile OpenUSD (or use the previously cached installation)
            #[cfg(feature = "vendored")]
            vendored::download_and_compile_openusd()
        }
    };
    // Verify that the OpenUSD installation contains the required subdirectories
    for path in LINK_PATHS {
        let path = std::path::PathBuf::from(path);
        if path.is_absolute() {
            continue;
        }
        let link_path = openusd_path.join(path);
        assert!(
            link_path.is_dir(),
            "The OpenUSD installation does not contain the required link path: `{}`",
            link_path.display()
        );
    }
    for path in INCLUDE_PATHS {
        let path = std::path::PathBuf::from(path);
        if path.is_absolute() {
            continue;
        }
        let include_path = openusd_path.join(path);
        assert!(
            include_path.is_dir(),
            "The OpenUSD installation does not contain the required include path: `{}`",
            include_path.display()
        );
    }

    // Configure link libraries
    #[cfg(feature = "monolithic_lib")]
    for lib in LINK_LIBS {
        println!("cargo:rustc-link-lib=dylib={lib}");
    }
    #[cfg(not(feature = "monolithic_lib"))]
    {
        // ENV: Determine the prefix of OpenUSD libraries when linking against them (analogous to the "-DPXR_LIB_PREFIX" build argument of OpenUSD)
        const DEFAULT_PXR_LIB_PREFIX: &str = "lib";
        println!("cargo:rerun-if-env-changed=PXR_LIB_PREFIX");
        let pxr_lib_prefix =
            std::env::var("PXR_LIB_PREFIX").unwrap_or_else(|_| DEFAULT_PXR_LIB_PREFIX.to_string());
        let pxr_lib_prefix = pxr_lib_prefix.trim_start_matches("lib");

        for lib in LINK_LIBS {
            if NON_USD_LINK_LIBS.contains(lib) {
                println!("cargo:rustc-link-lib=dylib={lib}");
            } else {
                println!("cargo:rustc-link-lib=dylib={pxr_lib_prefix}{lib}");
            }
        }
    }
    #[cfg(feature = "link_python")]
    {
        link_python::link_libs()
            .unwrap()
            .into_iter()
            .for_each(|lib| {
                println!("cargo:rustc-link-lib=dylib={lib}");
            });
    }

    // Cargo automatically adds link search paths to the dynamic library search path only for paths within OUT_DIR
    // Therefore, we create a symbolic link to the library within OUT_DIR instead if configuring link paths directly
    // In this way, dynamic library search paths do not need to be set manually when executing binaries through Cargo
    let openusd_path_out_dir =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_else(|| unreachable!()))
            .join("deps")
            .join("lib")
            .join(openusd_path.file_name().unwrap());
    if !openusd_path_out_dir.is_dir() {
        built_different::create_symlink(&openusd_path, &openusd_path_out_dir, true).unwrap();
    }
    for path in LINK_PATHS {
        let mut path = std::path::PathBuf::from(path);
        if path.is_relative() {
            path = openusd_path_out_dir.join(path);
        }
        println!("cargo:rustc-link-search={}", path.display());
    }
    #[cfg(feature = "link_python")]
    {
        link_python::link_paths()
            .unwrap()
            .into_iter()
            .for_each(|path| {
                println!("cargo:rustc-link-search={}", path.display());
            });
    }

    // Apply patches to the headers in a duplicate `include_patched_rs` directory
    built_different::apply_file_patches(
        "patches/include",
        openusd_path.join("include"),
        openusd_path.join("include_patched_rs"),
        true,
    )
    .unwrap();

    // Configure includes (the order matters here, i.e. local and patched headers should be included first)
    let include_paths_openusd_local = [std::path::PathBuf::from("./include")];
    let include_paths_openusd_patched = [openusd_path.join("include_patched_rs")];
    let include_paths_openusd = INCLUDE_PATHS.iter().map(|path| {
        let path = std::path::PathBuf::from(path);
        if path.is_relative() {
            openusd_path.join(path)
        } else {
            path
        }
    });
    let include_paths = include_paths_openusd_local
        .into_iter()
        .chain(include_paths_openusd_patched)
        .chain(include_paths_openusd)
        .collect::<Vec<_>>();
    #[cfg(feature = "link_python")]
    let include_paths = include_paths
        .into_iter()
        .chain(link_python::include_paths().unwrap())
        .collect::<Vec<_>>();

    // Expand macros to generate Rust code with `cpp` macros (workaround | `cpp!` macros cannot be nested)
    bindings::expand_macros_cpp();

    // Compile C++ code with `cpp`
    bindings::compile_cpp(&include_paths);

    // Generate bindings with `autocxx`
    bindings::generate_autocxx(&include_paths);
}

/// Module that contains functions for generating bindings.
mod bindings {
    pub fn generate_autocxx(include_paths: impl IntoIterator<Item = impl AsRef<std::ffi::OsStr>>) {
        autocxx_build::Builder::new("src/ffi/bindings.rs", include_paths)
            .extra_clang_args(&[
                "-x",
                "c++",
                "-std=c++17",
                "-stdlib=libstdc++",
                "-pthread",
                &format!("-D_GLIBCXX_USE_CXX11_ABI={}", super::USE_CXX11_ABI),
                "-Wno-everything",
            ])
            .build()
            .unwrap()
            .compiler("clang")
            .flag("-x")
            .flag("c++")
            .flag("-std=c++17")
            .cpp_set_stdlib(Some("stdc++"))
            .flag("-pthread")
            .define("_GLIBCXX_USE_CXX11_ABI", Some(super::USE_CXX11_ABI))
            .flag("-Wno-everything")
            .compile("openusd_sys_autocxx");
    }

    pub fn expand_macros_cpp() {
        pxr_build::codegen::codegen_vt_value(
            std::path::PathBuf::from("src")
                .join("ffi")
                .join("pxr")
                .join("vt")
                .join("value.rs"),
        );
    }

    pub fn compile_cpp(include_paths: impl IntoIterator<Item = impl AsRef<std::path::Path>>) {
        // Configure rebuild triggers based on file changes
        walkdir::WalkDir::new("src/ffi")
            .into_iter()
            .filter(|entry| {
                entry
                    .as_ref()
                    .map(|entry| {
                        entry.file_type().is_file()
                            && entry.path().extension() == Some("rs".as_ref())
                    })
                    .unwrap_or(false)
            })
            .for_each(|entry| {
                println!("cargo:rerun-if-changed={}", entry.unwrap().path().display());
            });

        let mut cpp_builder = cpp_build::Config::new();
        let mut cpp_builder = cpp_builder
            .compiler("clang")
            .flag("-x")
            .flag("c++")
            .flag("-std=c++17")
            .cpp_set_stdlib(Some("stdc++"))
            .define("_GLIBCXX_USE_CXX11_ABI", Some(super::USE_CXX11_ABI))
            .flag("-pthread")
            .flag("-Wno-everything");
        for path in include_paths {
            cpp_builder = cpp_builder.include(path);
        }
        cpp_builder.build("src/lib.rs");
    }
}

#[cfg(feature = "vendored")]
mod vendored {
    pub fn download_and_compile_openusd() -> std::path::PathBuf {
        // ENV: Determine if the download should be forced regardless of the cache validity
        println!("cargo:rerun-if-env-changed=OPENUSD_DOWNLOAD_FORCE");
        let force_download = built_different::parse_bool_env("OPENUSD_DOWNLOAD_FORCE", false);

        // ENV: Determine the version of OpenUSD to download and compile
        const DEFAULT_OPENUSD_DOWNLOAD_VERSION: &str = "22.11";
        println!("cargo:rerun-if-env-changed=OPENUSD_DOWNLOAD_VERSION");
        let openusd_version = std::env::var("OPENUSD_DOWNLOAD_VERSION")
            .unwrap_or_else(|_| DEFAULT_OPENUSD_DOWNLOAD_VERSION.to_string());

        // ENV: Determine the URL from which download OpenUSD source
        const DEFAULT_OPENUSD_DOWNLOAD_URL: &str =
        "https://github.com/PixarAnimationStudios/OpenUSD/archive/refs/tags/v${{VERSION}}.tar.gz";
        println!("cargo:rerun-if-env-changed=OPENUSD_DOWNLOAD_URL");
        let download_url = std::env::var("OPENUSD_DOWNLOAD_URL")
            .unwrap_or_else(|_| DEFAULT_OPENUSD_DOWNLOAD_URL.to_string());
        // Replace a potential version placeholder in the URL
        let download_url = download_url.replace("${{VERSION}}", &openusd_version);

        // ENV: Determine whether to symlink or move the installation to the cache
        println!("cargo:rerun-if-env-changed=OPENUSD_DOWNLOAD_SYMLINK_CACHE");
        let symlink_cache = built_different::parse_bool_env("OPENUSD_DOWNLOAD_SYMLINK_CACHE", true);

        // Determine the cache path based on the version of OpenUSD and the current build configuration
        let cache_path = determine_cache_path_openusd(&openusd_version);

        // Make OPENUSD_PATH environment variable point to the cache (used for default location of OpenUSD if OPENUSD_PATH is not set during runtime)
        println!("cargo:rustc-env=OPENUSD_PATH={}", cache_path.display());

        // Skip download if the cache is valid and download is not forced
        if !force_download && cache_path.exists() {
            return cache_path;
        }

        // Determine the path where the OpenUSD source will be extracted
        let openusd_extract_path =
            std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_else(|| unreachable!()))
                .join(format!("openusd{openusd_version}_source"));
        // Determine the installation path
        let openusd_install_path =
            std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_else(|| unreachable!()))
                .join(format!("openusd{openusd_version}"));

        // Download and uncompress
        built_different::download_and_uncompress(
            &download_url,
            &openusd_extract_path,
            force_download,
        )
        .unwrap();

        // Locate the root of the OpenUSD source
        let openusd_src_path = openusd_extract_path.join(format!("OpenUSD-{openusd_version}"));

        // Apply compile-time patches
        built_different::apply_file_patches_in_place("patches/src", &openusd_src_path, true, true)
            .unwrap();

        // Locate the build script
        let build_script_path = openusd_src_path.join("build_scripts").join("build_usd.py");

        // Configure additional build arguments
        let mut extra_args: Vec<String> = Vec::new();

        // Configure the prefix of the libraries (the default seems to be "libusd" for some reason)
        extra_args.push("--build-args".to_string());
        extra_args.push("USD,\"-DPXR_LIB_PREFIX=lib\"".to_string());

        // Build as individual shared libraries or as a single monolithic library
        #[cfg(not(feature = "monolithic_lib"))]
        extra_args.push("--build-shared".to_string());
        #[cfg(feature = "monolithic_lib")]
        extra_args.push("--build-monolithic".to_string());

        // // Select debug or release build variant
        // #[cfg(debug_assertions)]
        // {
        //     extra_args.push("--build-variant=debug".to_string()); // Alternative: --build-variant=relwithdebuginfo
        //     extra_args.push("--prefer-safety-over-speed".to_string());
        // }
        // #[cfg(not(debug_assertions))]
        {
            extra_args.push("--build-variant=release".to_string());
            extra_args.push("--prefer-speed-over-safety".to_string());
        }

        // Enable/disable C++11 ABI on Linux
        #[cfg(target_family = "unix")]
        extra_args.push(format!("--use-cxx11-abi={}", super::USE_CXX11_ABI));

        // Compile OpenUSD
        let output = std::process::Command::new("python3")
            .arg(build_script_path)
            .args([
                "--no-tests",
                "--no-examples",
                "--no-tutorials",
                "--no-tools",
                "--no-docs",
                "--no-usdview",
                #[cfg(feature = "link_python")]
                "--python",
                #[cfg(not(feature = "link_python"))]
                "--no-python",
                "--no-debug-python",
                #[cfg(feature = "usd_imaging")]
                "--usd-imaging",
                #[cfg(not(feature = "usd_imaging"))]
                "--no-usd-imaging",
                #[cfg(feature = "ptex")]
                "--ptex",
                #[cfg(not(feature = "ptex"))]
                "--no-ptex",
                #[cfg(feature = "openvdb")]
                "--openvdb",
                #[cfg(not(feature = "openvdb"))]
                "--no-openvdb",
                #[cfg(feature = "embree")]
                "--embree",
                #[cfg(not(feature = "embree"))]
                "--no-embree",
                #[cfg(feature = "prman")]
                "--prman",
                #[cfg(not(feature = "prman"))]
                "--no-prman",
                #[cfg(feature = "openimageio")]
                "--openimageio",
                #[cfg(not(feature = "openimageio"))]
                "--no-openimageio",
                #[cfg(feature = "opencolorio")]
                "--opencolorio",
                #[cfg(not(feature = "opencolorio"))]
                "--no-opencolorio",
                #[cfg(feature = "alembic")]
                "--alembic",
                #[cfg(not(feature = "alembic"))]
                "--no-alembic",
                #[cfg(feature = "hdf5")]
                "--hdf5",
                #[cfg(not(feature = "hdf5"))]
                "--no-hdf5",
                #[cfg(feature = "draco")]
                "--draco",
                #[cfg(not(feature = "draco"))]
                "--no-draco",
                #[cfg(feature = "materialx")]
                "--materialx",
                #[cfg(not(feature = "materialx"))]
                "--no-materialx",
            ])
            .args(extra_args)
            .arg(&openusd_install_path)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
            .unwrap();
        let status = output.status;
        if !status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("Failed to compile OpenUSD source!\n{status}\n{stdout}\n{stderr}");
        }

        // Remove the source now that it is no longer needed
        std::fs::remove_dir_all(&openusd_extract_path).unwrap();
        std::fs::remove_dir_all(openusd_install_path.join("src")).unwrap();

        // Move OpenUSD to the cache or create a symlink
        if symlink_cache && !cache_path.is_dir() {
            built_different::create_symlink(&openusd_install_path, &cache_path, true).unwrap();
        } else if !cache_path.is_symlink() {
            if let Some(parent) = cache_path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::rename(&openusd_install_path, &cache_path).unwrap();
        }

        cache_path
    }

    fn determine_cache_path_openusd(version: &str) -> std::path::PathBuf {
        // Determine the name of the build variant
        // #[cfg(debug_assertions)]
        // const BUILD_VARIANT: &str = "_debug";
        // #[cfg(not(debug_assertions))]
        const BUILD_VARIANT: &str = "_release";

        // Determine if the build is monolithic
        #[cfg(feature = "monolithic_lib")]
        const IS_MONOLITHIC: &str = "_monolithic";
        #[cfg(not(feature = "monolithic_lib"))]
        const IS_MONOLITHIC: &str = "";

        // Determine if Python is linked
        #[cfg(feature = "link_python")]
        const WITH_PYTHON: &str = "_with_python";
        #[cfg(not(feature = "link_python"))]
        const WITH_PYTHON: &str = "";

        // Determine if C++11 ABI is used
        #[cfg(all(target_family = "unix", feature = "use_cxx11_abi"))]
        const WITH_CXX11_ABI: &str = "_cxx11";
        #[cfg(all(target_family = "unix", not(feature = "use_cxx11_abi")))]
        const WITH_CXX11_ABI: &str = "";

        // The root of the cache path is in the system's cache directory (or /tmp if not available)
        let cache_root = built_different::cache_dir().unwrap_or(std::path::PathBuf::from("/tmp"));

        // Use different path for each OpenUSD version and build configuration
        cache_root.join("pxr_rs").join(format!(
            "openusd{version}{BUILD_VARIANT}{WITH_CXX11_ABI}{IS_MONOLITHIC}{WITH_PYTHON}"
        ))
    }
}

#[cfg(feature = "link_python")]
mod link_python {
    const LINK_LIBS_PYTHON_EXTRA: &[&str] =
        &["boost_python${{PYTHON_VERSION_MAJOR}}${{PYTHON_VERSION_MINOR}}"];

    pub fn link_libs() -> python_config::PyResult<Vec<String>> {
        let python_cfg = python_config::PythonConfig::new();
        let python_version = python_cfg.semantic_version()?;
        Ok(python_cfg
            .ldflags()
            .unwrap()
            .split_whitespace()
            .filter_map(|flag| {
                if flag.starts_with("-l") {
                    Some(flag.trim_start_matches("-l").to_string())
                } else {
                    None
                }
            })
            .chain(LINK_LIBS_PYTHON_EXTRA.iter().map(|lib| {
                lib.replace(
                    "${{PYTHON_VERSION_MAJOR}}",
                    &python_version.major.to_string(),
                )
                .replace(
                    "${{PYTHON_VERSION_MINOR}}",
                    &python_version.minor.to_string(),
                )
                .replace(
                    "${{PYTHON_VERSION_PATCH}}",
                    &python_version.patch.to_string(),
                )
            }))
            .collect())
    }

    pub fn link_paths() -> python_config::PyResult<Vec<std::path::PathBuf>> {
        let python_cfg = python_config::PythonConfig::new();
        Ok(python_cfg
            .ldflags()
            .unwrap()
            .split_whitespace()
            .filter_map(|flag| {
                if flag.starts_with("-L") {
                    Some(std::path::PathBuf::from(flag.trim_start_matches("-L")))
                } else {
                    None
                }
            })
            .collect())
    }

    pub fn include_paths() -> python_config::PyResult<Vec<std::path::PathBuf>> {
        let python_cfg = python_config::PythonConfig::new();
        python_cfg.include_paths()
    }
}
