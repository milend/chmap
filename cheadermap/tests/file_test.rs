// Copyright (c) 2022 Milen Dzhumerov

use core::panic;
use std::fs;

use cheadermap::{self, binary::print_headermap};
use serde_json::{Map, Value};

use cheadermap::binary::{parse_headermap, Entry};

mod test_data;

fn assert_contains_entry(json_entry: &Map<String, Value>, headermap_entry: &Entry) {
    match json_entry.get(headermap_entry.key) {
        Some(value) => {
            let prefix_suffix_obj = value.as_object().unwrap();
            let prefix = prefix_suffix_obj.get("prefix").unwrap().as_str().unwrap();
            let suffix = prefix_suffix_obj.get("suffix").unwrap().as_str().unwrap();
            assert!(prefix.eq(headermap_entry.prefix));
            assert!(suffix.eq(headermap_entry.suffix));
        }
        None => {
            panic!("Did not find entry for bucket: {:#?}", headermap_entry);
        }
    }
}

#[test]
fn test_sdwebimage_hmap() {
    let binary_hmap_path = test_data::get_sdwebimage_binary_hmap_path();

    let bytes = fs::read(binary_hmap_path).unwrap();
    let reference_json_hmap_path = test_data::get_sdwebimage_binary_reference_json_output();
    let reference_json_bytes = fs::read(reference_json_hmap_path).unwrap();
    let reference_json_value: Value = serde_json::from_slice(&reference_json_bytes).unwrap();

    match reference_json_value {
        Value::Object(json_entries_map) => {
            let headermap_entries = parse_headermap(&bytes, true).unwrap();
            assert_eq!(
                json_entries_map.len(),
                headermap_entries.len(),
                "Expecting the same number of entries"
            );
            for entry in &headermap_entries {
                assert_contains_entry(&json_entries_map, entry);
            }
        }
        _ => {
            panic!("Expected top-level JSON element to be a map");
        }
    }
}

#[test]
fn test_sdwebimage_reference_text_print() {
    let binary_hmap_path = test_data::get_sdwebimage_binary_hmap_path();

    let mut output_buffer = Vec::new();
    print_headermap(&mut output_buffer, binary_hmap_path).unwrap();

    let expected_print_output_path = test_data::get_sdwebimage_binary_reference_text_output();
    let expected_output = fs::read(expected_print_output_path).unwrap();

    assert_eq!(output_buffer, expected_output);
}

#[test]
fn test_malformed_sdwebimage_hmaps() {
    let malformed_hmap_paths = test_data::get_sdwebimage_malformed_binary_hmap_paths();

    for hmap_path in malformed_hmap_paths {
        let bytes = fs::read(hmap_path).unwrap();
        let entries_result = parse_headermap(&bytes, true);
        assert!(entries_result.is_err());
    }
}

#[test]
fn test_single_entry_enumeration() {
    let binary_hmap_path = test_data::get_sdwebimage_binary_hmap_path();
    let bytes = fs::read(binary_hmap_path).unwrap();

    let mut item_count = 0;
    cheadermap::binary::headermap_enumerate_entries(&bytes, true, |_| {
        item_count += 1;
        false
    })
    .expect("Expecetd valid SDWebImage headermap");

    assert_eq!(item_count, 1);
}
