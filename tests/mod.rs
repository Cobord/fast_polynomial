use fast_polynomial::{poly, rational};

// version from readme
fn horners_method(x: f64, coefficients: &[f64]) -> f64 {
    let mut sum = 0.0;
    for coeff in coefficients.iter().rev().copied() {
        sum = x * sum + coeff;
    }
    sum
}

macro_rules! assert_feq {
    ($e:expr, $a:expr, $b:expr) => {{
        let (a, b) = ($a, $b);
        assert!((a - b).abs() < $e, "{} != {}", a, b);
    }};
}

#[test]
fn test_polys() {
    #[rustfmt::skip]
    let c = [
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        1.1274404084913705, 0.6266756469558616,
    ];

    assert_feq!(1e-10, 1.09320587687, horners_method(0.2, &c));
    assert_feq!(1e-10, 1.09320587687, poly(0.2, &c));

    assert_feq!(1e-10, 1.76658610593, horners_method(0.5, &c));
    assert_feq!(1e-10, 1.76658610593, poly(0.5, &c));

    assert_feq!(1e-10, 0.662519601199, horners_method(-0.4, &c[..4]));
    assert_feq!(1e-10, 0.662519601199, poly(-0.4, &c[..4]));

    for x in [-0.5, 0.1, 0.5, 0.9, 1.1, 1.5] {
        for i in 0..=c.len() {
            assert_feq!(1e-4, horners_method(x, &c[..i]), poly(x, &c[..i]));

            if i < (c.len() - 10) {
                let mut numerator = &c[i..i + 10];
                let mut denominator = &c[(i + 1)..(i + i % 10 + 2)];

                if i % 2 == 0 {
                    std::mem::swap(&mut numerator, &mut denominator);
                }

                //println!("{}: {}/{}", x, numerator.len(), denominator.len());

                assert_feq!(
                    1e-3,
                    horners_method(x, numerator) / horners_method(x, denominator),
                    rational(x, numerator, denominator)
                );
            }
        }
    }
}
