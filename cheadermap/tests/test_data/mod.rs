// Copyright (c) 2022 Milen Dzhumerov

use std::path::PathBuf;

pub fn get_sdwebimage_binary_reference_json_output() -> PathBuf {
    get_path_for_test_resources_filename("SDWebImage-all-target-headers.reference-output.json")
}

pub fn get_sdwebimage_binary_hmap_path() -> PathBuf {
    get_path_for_test_resources_filename("SDWebImage-all-target-headers.hmap")
}

pub fn get_sdwebimage_malformed_binary_hmap_paths() -> Vec<PathBuf> {
    return vec![
        get_path_for_test_resources_filename(
            "SDWebImage-all-target-headers.malformed.header-only.hmap",
        ),
        get_path_for_test_resources_filename(
            "SDWebImage-all-target-headers.malformed.string-section-cut-off.hmap",
        ),
    ];
}

fn get_path_for_test_resources_filename(filename: &str) -> PathBuf {
    let mut path = get_tests_resources_dir();
    path.push(filename);
    path
}

fn get_tests_resources_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources");
    path.push("tests");
    path
}
