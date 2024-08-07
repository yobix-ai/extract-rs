use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::io;

fn main() {
    // Main build output directory
    let out_dir = env::var("OUT_DIR").map(PathBuf::from).unwrap();
    let jdk_install_dir = out_dir.join("graalvm-jdk"); // jdk install dir if no JAVA_HOME is set
    let libs_out_dir = out_dir.join("libs"); // Directory to copy the built shared library to

    // Set tika_native source directory and python bindings directory
    let root_dir = env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    let tika_native_dir = root_dir.join("tika-native");
    // canonicalize does not work on Windows because it returns UNC paths
    //let python_bind_dir = fs::canonicalize("../bindings/python/python/extractrs").unwrap();
    let python_bind_dir = root_dir.join("../bindings/python/python/extractrs");
    if !python_bind_dir.is_dir() { panic!("{} does not exist", python_bind_dir.display()) };

    // Use JAVA_HOME to find or install GraalVM JDK
    let graalvm_home = get_graalvm_home(&jdk_install_dir);

    // Just for debugging
    // let graal_home = env::var("GRAALVM_HOME");
    // let java_home = env::var("JAVA_HOME");
    // println!("cargo:warning=GRAALVM_HOME: {:?}", graal_home);
    // println!("cargo:warning=JAVA_HOME: {:?}", java_home);
    //println!("cargo:warning=dist_dir: {}", dist_dir.display());
    // println!("cargo:warning=out_dir: {}", out_dir.display());
    //println!("cargo:warning=tika_native_dir: {:?}", tika_native_dir);


    gradle_build(&graalvm_home, &tika_native_dir, &libs_out_dir, &python_bind_dir);
    println!("Successfully built libs 🚀");


    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libs_out_dir.display());

    // Tell cargo to tell rustc to link the `tika_native` shared library.
    println!("cargo:rustc-link-lib=dylib=tika_native");
}

// Run the gradle build command to build tika-native
fn gradle_build(graalvm_home: &Path, tika_native_dir: &Path,
                libs_out_dir: &PathBuf, dist_dir: &Path
) {
    println!("Using GraalVM JDK found at {}", graalvm_home.display());
    println!("Building tika_native libs this might take a while ... Please be patient!!");

    let gradlew = if cfg!(target_os = "windows") {
        tika_native_dir.join("gradlew.bat")
    } else {
        tika_native_dir.join("gradlew")
    };

    Command::new(gradlew)
        .current_dir(tika_native_dir)
        .arg("nativeCompile")
        .env("JAVA_HOME", graalvm_home)
        .status()
        .expect("Failed to build tika-native");

    let build_path = tika_native_dir.join("build/native/nativeCompile");

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;

    for dir in [libs_out_dir, dist_dir].iter() {
        fs_extra::dir::copy(&build_path, dir, &options)
            .expect("Failed to copy build artifacts to OUTPUT_DIR");

        fs::remove_file(dir.join("graal_isolate_dynamic.h")).unwrap();
        fs::remove_file(dir.join("graal_isolate.h")).unwrap();
        fs::remove_file(dir.join("libtika_native_dynamic.h")).unwrap();
        fs::remove_file(dir.join("libtika_native.h")).unwrap();
    }

}

// Firsts check JAVA_HOME
pub fn get_graalvm_home(install_dir: &PathBuf) -> PathBuf {
    let graalvm_home_env = env::var("GRAALVM_HOME");
    match graalvm_home_env {
        Ok(graalvm_home_val) => {
            // Check that native-image is in GRAALVM_HOME/bin
            let graalvm_home = PathBuf::from(graalvm_home_val);
            check_graalvm(&graalvm_home, true);
            graalvm_home
        }
        Err(_) => {

            let java_home_env = env::var("JAVA_HOME");
            match java_home_env {
                Ok(java_home_val) => {
                    // Check that native-image is in JAVA_HOME/bin if not install GraalVM CE
                    let mut graalvm_home = PathBuf::from(java_home_val);
                    if !check_graalvm(&graalvm_home, false) {
                        graalvm_home = install_graalvm_ce(&install_dir);
                        check_graalvm(&graalvm_home, true);
                    }
                    graalvm_home
                }
                Err(_) => {
                    // If no JAVA_HOME is set, try to download and install GraalVM CE
                    let graalvm_home = install_graalvm_ce(&install_dir);
                    check_graalvm(&graalvm_home, true);
                    graalvm_home
                }
            }
        }
    }
}

// checks if GraalVM JDK is installed and pointed to by JAVA_HOME or panics if it can't be found
pub fn check_graalvm(graalvm_home: &Path, panic: bool) -> bool {
    let native_image_exe = if cfg!(target_os = "windows") {
        "native-image.cmd"
    } else {
        "native-image"
    };

    // Check that native-image is in JAVA_HOME/bin
    let native_image = graalvm_home.join("bin").join(native_image_exe);
    let exists = native_image.exists();
    if panic && !exists {
        panic!("Your GraalVM JDK installation is pointing to: {}. Please make sure your \
                it is a valid GraalVM JDK. {}",
               graalvm_home.display(),
               graalvm_install_help_msg());
    }
    exists
}

fn graalvm_install_help_msg() -> String {
    let sdkman_graalvm_version = if cfg!(target_os = "macos") {
        "24.0.1.r22-nik" // Bellsoft Liberika NIK 24.0.1.r22 -> jdk 22
    } else {
        "22.0.1-graalce"
    };

    format!("\nWe recommend using sdkman to install and \
                manage different JDKs. See https://sdkman.io/usage for more information.\n\
                You can install graalvm using:\n  \
                sdk install java {} \n  \
                sdk use java {}", sdkman_graalvm_version, sdkman_graalvm_version)
}

pub fn install_graalvm_ce(install_dir: &PathBuf) -> PathBuf {
    let (base_url, archive_ext, main_dir) = if cfg!(target_os = "windows") {
        let url = if cfg!(target_arch = "x86_64") {
            "https://github.com/graalvm/graalvm-ce-builds/releases/download/jdk-22.0.1/graalvm-community-jdk-22.0.1_windows-x64_bin.zip"
        } else {
            panic!("Unsupported architecture: {}", cfg!(target_arch));
        };
        (url, "zip", "graalvm-community-openjdk-22.0.1+8.1")

    } else if cfg!(target_os = "macos") {
        let url = if cfg!(target_arch = "x86_64") {
            //"https://github.com/graalvm/graalvm-ce-builds/releases/download/jdk-22.0.1/graalvm-community-jdk-22.0.1_macos-x64_bin.tar.gz"
            "https://github.com/bell-sw/LibericaNIK/releases/download/24.0.1+1-22.0.1+10/bellsoft-liberica-vm-openjdk22.0.1+10-24.0.1+1-macos-amd64.tar.gz"
        } else if cfg!(target_arch = "aarch64") {
            //"https://github.com/graalvm/graalvm-ce-builds/releases/download/jdk-22.0.1/graalvm-community-jdk-22.0.1_macos-aarch64_bin.tar.gz"
            "https://github.com/bell-sw/LibericaNIK/releases/download/24.0.1+1-22.0.1+10/bellsoft-liberica-vm-openjdk22.0.1+10-24.0.1+1-macos-aarch64.tar.gz"
        } else {
            panic!("Unsupported architecture: {}", cfg!(target_arch));
        };
        //(url, "tar.gz", "graalvm-community-openjdk-22.0.1+8.1/Contents/Home/")
        (url, "tar.gz", "bellsoft-liberica-vm-openjdk22-24.0.1/Contents/Home")

    } else {
        let url = if cfg!(target_arch = "x86_64") {
            "https://github.com/graalvm/graalvm-ce-builds/releases/download/jdk-22.0.1/graalvm-community-jdk-22.0.1_linux-x64_bin.tar.gz"
        } else if cfg!(target_arch = "aarch64") {
            "https://github.com/graalvm/graalvm-ce-builds/releases/download/jdk-22.0.1/graalvm-community-jdk-22.0.1_linux-aarch64_bin.tar.gz"
        } else {
            panic!("Unsupported architecture: {}", cfg!(target_arch));
        };
        (url, "tar.gz", "graalvm-community-openjdk-22.0.1+8.1")
    };

    let graalvm_home = install_dir.join(main_dir);


    // Download and GraalVM CE
    if !graalvm_home.exists() {
        fs::create_dir_all(&install_dir).unwrap();
        let archive_path = install_dir.join("graalvm-ce-archive").with_extension(archive_ext);

        // Download the GraalVM archive file if it was not downloaded before
        if !archive_path.exists() {
            println!("Downloading GraalVM JDK from {}", base_url);

            let client = reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(60*5))// 5 minutes
                .build().unwrap();
            let response = client.get(base_url).send().unwrap();
            let mut out = fs::File::create(&archive_path).unwrap();
            io::copy(&mut response.bytes().unwrap().as_ref(), &mut out).unwrap();
        }

        // Extract the archive file
        if archive_path.exists() {
            println!("Extracting GraalVM JDK archive {}", archive_path.display());

            if cfg!(target_os = "windows") {
                let archive_file = fs::File::open(&archive_path).unwrap();
                let mut archive = zip::ZipArchive::new(std::io::BufReader::new(archive_file)).unwrap();

                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).unwrap();
                    let outpath = install_dir.join(file.name());

                    if file.is_dir() {
                        fs::create_dir_all(&outpath).unwrap();
                    } else {
                        if let Some(parent) = outpath.parent() {
                            if !parent.exists() {
                                fs::create_dir_all(parent).unwrap();
                            }
                        }
                        let mut outfile = fs::File::create(&outpath).unwrap();
                        io::copy(&mut file, &mut outfile).unwrap();
                    }
                }
            } else {
                let tar_gz_file = fs::File::open(&archive_path).unwrap();
                let tar = flate2::read::GzDecoder::new(tar_gz_file);
                let mut archive = tar::Archive::new(tar);
                archive.unpack(&install_dir).unwrap();
            }
        } else {
            panic!("Failed to download GraalVM JDK from {}", base_url);
        }
    }

    let graalvm_home = install_dir.join(main_dir);
    graalvm_home
}