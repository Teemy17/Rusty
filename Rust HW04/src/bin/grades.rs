fn split_grades(grades: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut grade_above_d = Vec::new();
    let mut grade_d_and_f = Vec::new();

    for grade in grades {
        if grade == "D" || grade == "F" {
            grade_d_and_f.push(grade);
        } else {
            grade_above_d.push(grade);
        }
    }

    return (grade_above_d, grade_d_and_f)
}

// This function creates grade conditions
fn grade_citeria(score: i64) -> &'static str {
    if score > 100 {
        "Invalid score"
    } else if score >= 95 && score <= 100 {
        "A+" 
    } else if score >= 81 && score < 95 {
        "A"
    } else if score >= 71 && score < 81 {
        "B"
    } else if score >= 61 && score < 71 {
        "C"
    } else if score >= 50 && score < 61 {
        "D"
    } else if score >= 0 && score < 50 {
        "F"
    } else if score < 0 {
        "Invalid score"
    } else {
        "Unknown grade"
    }
}

fn split_scores(scores: Vec<i64>) -> (Vec<(&'static str, i64)>, Vec<(&'static str, i64)>) {
    let mut grade_above_d = Vec::new();
    let mut grade_d_and_f = Vec::new();

    for &score in &scores {
        let calculated_grade = grade_citeria(score);

        if calculated_grade == "D" || calculated_grade == "F" {
            grade_d_and_f.push((calculated_grade, score));
        } else if  calculated_grade == "A+" || calculated_grade == "A" || calculated_grade == "B" || calculated_grade == "C" {
            grade_above_d.push((calculated_grade, score));
        }
    }
    return (grade_above_d, grade_d_and_f)
}

#[test]
fn test_split_grades() {
    assert_eq!(split_grades(vec!["B", "F", "A+", "D", "C"]), (vec!["B", "A+", "C"], vec!["F", "D"]));
    assert_eq!(split_grades(vec!["A", "B", "C", "D", "F", "B+", "C+", "D", "F"]), (vec!["A", "B", "C", "B+", "C+"], vec!["D", "F", "D", "F"]));
}

#[test]
fn test_split_scores() {
    assert_eq!(split_scores(vec![75, 42, 98, 54, 63]), (vec![("B", 75), ("A+", 98), ("C", 63)], vec![("F", 42), ("D", 54)]));
    assert_eq!(split_scores(vec![120, 30, -4, 46, 69]), (vec![("C", 69)], vec![("F", 30), ("F", 46)]));
}