use rand::Rng;

fn filter_points(v: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let mut iter = v.iter();
    let mut result = Vec::new();

    if v.is_empty() {
        return result
    }

    while let Some((x,y)) = iter.next() {
        if (x.powf(2.0) + y.powf(2.0)).sqrt() <= 1.0 {
            result.push((*x,*y))
        }
    }
        return result
}

fn gen_points<R: Rng>(rng: &mut R, n: i64) -> Vec<(f64, f64)> {
    let mut result = Vec::new();
    for i in 0..n {
        let x = rng.gen_range(-1.0 ..=1.0);
        let y = rng.gen_range(-1.0 ..=1.0);
        result.push((x,y))
    }
    return result
}

#[test]
fn test_filter_points() {
    assert_eq!(filter_points(&[]), Vec::new());
    assert_eq!(filter_points(&[(0.5, 0.5), (0.8, 0.1), (0.2, 0.9), (0.6, 0.6), (1.0, 1.0), (1.1, 0.0), (0.0, 1.1)]), vec![(0.5, 0.5), (0.8, 0.1), (0.2, 0.9), (0.6, 0.6)]);
    assert_eq!(
        filter_points(&[(-2.0, -2.0), (-1.5, -1.5), (-1.0, -1.0), (-0.5, -0.5), (0.0, 0.0), (0.5, 0.5), (1.0, 1.0), (1.5, 1.5), (2.0, 2.0)]),
        vec![(-0.5, -0.5), (0.0, 0.0), (0.5, 0.5)]
    );
}


#[test]
fn test_gen_points() {
    let mut rng = rand::thread_rng();

    let generated_empty = gen_points(&mut rng, 0);
    assert_eq!(generated_empty.len(), 0);

    let generated_positive = gen_points(&mut rng, 5);
    assert_eq!(generated_positive.len(), 5);
    for &(x, y) in &generated_positive {
        assert!(x >= -1.0 && x <= 1.0);
        assert!(y >= -1.0 && y <= 1.0);
    }

    let generated_negative = gen_points(&mut rng, -5);
    assert_eq!(generated_negative.len(), 0);
}