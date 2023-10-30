pub fn to_array(s: String) -> Vec<String> {
    s.split(",")
        .map(|address| address.trim().to_string())
        .collect()
}

pub fn to_url(s: String) -> String {
    if s.starts_with("http://") {
        return s;
    }
    format!("http://{}", s)
}

pub fn partial_pick_with_index(mut n: u32, start_index: u32, arr: &Vec<String>) -> Vec<String> {
    let mut result = vec![];
    if n > arr.len() as u32 {
        n = arr.len() as u32;
    }
    for i in 0..n {
        let mut idx = i + start_index;
        if idx > arr.len() as u32 - 1 {
            idx = idx % (arr.len() as u32);
        }
        result.push(arr[idx as usize].to_string());
    }
    result
}