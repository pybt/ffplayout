use regex::Regex;
use simplelog::*;

/// Apply custom filters
pub fn filter_node(filter: &str) -> (String, String) {
    let re = Regex::new(r"^;?(\[[^\[]+\])?|\[[^\[]+\]$").unwrap(); // match start/end link;
    let mut video_filter = String::new();
    let mut audio_filter = String::new();

    // match chain with audio and video filter
    if filter.contains("[c_v_out]") && filter.contains("[c_a_out]") {
        let v_pos = filter.find("[c_v_out]").unwrap();
        let a_pos = filter.find("[c_a_out]").unwrap();
        let mut delimiter = "[c_v_out]";

        // split delimiter should be first filter output link
        if v_pos > a_pos {
            delimiter = "[c_a_out]";
        }

        if let Some((f_1, f_2)) = filter.split_once(delimiter) {
            if f_2.contains("[c_a_out]") {
                video_filter = re.replace_all(f_1, "").to_string();
                audio_filter = re.replace_all(f_2, "").to_string();
            } else {
                video_filter = re.replace_all(f_2, "").to_string();
                audio_filter = re.replace_all(f_1, "").to_string();
            }
        }
    } else if filter.contains("[c_v_out]") {
        video_filter = re.replace_all(filter, "").to_string();
    } else if filter.contains("[c_a_out]") {
        audio_filter = re.replace_all(filter, "").to_string();
    } else if !filter.is_empty() && filter != "~" {
        error!("Custom filter is not well formatted, use correct out link names (\"[c_v_out]\" and/or \"[c_a_out]\"). Filter skipped!")
    }

    if filter.starts_with("[v_in]") {
        video_filter = format!("[v_in]{video_filter}");
    }

    (video_filter, audio_filter)
}
