fn vflip(img: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for line in img.iter().rev() {
        result.push(line.to_string());
    }

    result
}

fn hflip(img: &[String]) -> Vec<String> {
    let max_len = img
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);
    let mut result = Vec::new();

    for line in img {
        let reversed_line = format!(
            "{:>width$}",
            line.chars().rev().collect::<String>(),
            width = max_len
        );
        result.push(reversed_line);
    }

    result
}

#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp), [""; 0]);
    assert_eq!(hflip(&emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(vflip(&data), ["<==", "#####", "<--"]);
    assert_eq!(hflip(&data), ["  --<", "#####", "  ==<"]);
}

fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for line in img1 {
        result.push(line.to_string());
    }
    for line in img2 {
        result.push(line.to_string());
    }

    result
}

fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let max_len = img1.len().max(img2.len());
    let mut result = Vec::with_capacity(max_len);

    for i in 0..max_len {
        let mut row = String::new();

        if i < img1.len() {
            row += &img1[i];
        }

        if i < img2.len() {
            let spaces =
                img1
                    .iter()
                    .map(|s| s.len())
                    .max()
                    .unwrap_or(0) - row.len();
            row += &" ".repeat(spaces);
            row += &img2[i];
        }

        result.push(row);
    }

    result
}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);

    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);

    assert_eq!(vcat(&data, &data), ["<--", "#####", "<==", "<--", "#####", "<=="]);

    assert_eq!(hcat(&data, &data[..2]), ["<--  <--", "##########", "<=="]);
    assert_eq!(hcat(&data[..2], &data), ["<--  <--", "##########", "     <=="]);
}
