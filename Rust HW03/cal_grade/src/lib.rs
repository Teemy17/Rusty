fn grade<'a>( v: &'a mut[&'a str]) -> Vec<&'a str> {
    for n in 0..v.len() {
        let x: i64 = match v[n].parse() {
            Ok(num) => num,
            Err(_) => -1,
    };
            if x > 100  {
                v[n] = "Invalid score"
            } 
            else if x >= 95 && x <= 100 {
                v[n] = "Excellent with A+"
            }
            else if x >= 81 && x < 95 {
                v[n] = "A"
            }
            else if x >= 71 && x < 81 {
                v[n] = "B"
            }
            else if x >= 61 && x < 71 {
                v[n] = "C"
            }
            else if x >= 50 && x < 61 {
                v[n] = "D"
            }
            else if x >= 0 && x < 50 {
                v[n] = "Failed with F"
            }
            else if x < 0 {
                v[n] = "Invalid score"
            }
        }
    v.to_vec()
} 

fn grade_rec<'a>( v: &'a mut[&str]) -> Vec<&'a str> {
    if !v.is_empty(){
      let x: i64 = match v[0].parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
        if x > 100  {
            v[0] = "Invalid score"
        } 
        else if x >= 95 && x <= 100 {
            v[0] = "Excellent with A+"
        }
        else if x >= 81 && x < 95 {
            v[0] = "A"
        }
        else if x >= 71 && x < 81 {
            v[0] = "B"
        }
        else if x >= 61 && x < 71 {
            v[0] = "C"
        }
        else if x >= 50 && x < 61 {
            v[0] = "D"
        }
        else if x >= 0 && x < 50 {
            v[0] = "Failed with F"
        }
        else if x < 0 {
            v[0] = "Invalid score"
        }
            grade_rec(&mut v[1..]);
    }
    v.to_vec()
  }
   

#[test]
fn test_grade() {
assert_eq!(grade(&mut["10", "20", "-5", "75", "50", "400", "60", "70", "90", "30"]), 
["Failed with F", "Failed with F", "Invalid score", "B", "D", "Invalid score", "D", "C", "A", "Failed with F"]);
}

#[test]
fn test_grade_rec() {
assert_eq!(grade_rec(&mut["10", "20", "-5", "75", "50", "400", "60", "70", "90", "30"]), 
["Failed with F", "Failed with F", "Invalid score", "B", "D", "Invalid score", "D", "C", "A", "Failed with F"]);
}