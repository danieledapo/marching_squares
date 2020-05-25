/// Simplify a given polyline by reducing the amount of points that do not actually contribute a
/// lot of details to the overall shape.
pub fn simplify(poly: &[(f64, f64)]) -> Vec<(f64, f64)> {
    simplify_with_eps(poly, 1e-9)
}

pub fn simplify_with_eps(poly: &[(f64, f64)], eps: f64) -> Vec<(f64, f64)> {
    let mut r = vec![];
    _simplify_with_eps(&mut r, poly, eps);
    r
}

pub fn _simplify_with_eps(r: &mut Vec<(f64, f64)>, poly: &[(f64, f64)], eps: f64) {
    // Ramer Douglas Peucker doesn't work with closed paths, thus simplify the open path and then
    // close it manually
    if !poly.is_empty() && poly[0] == poly[poly.len() - 1] {
        rdp(r, &poly[..poly.len() - 1], eps);
        r.push(poly[poly.len() - 1]);
    } else {
        rdp(r, poly, eps);
    }
}

/// Implementation of the [Ramer–Douglas–Peucker algorithm] to simplify an open path.
///
/// [0]: https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm
fn rdp(r: &mut Vec<(f64, f64)>, poly: &[(f64, f64)], eps: f64) {
    if poly.len() < 3 {
        r.extend_from_slice(poly);
        return;
    }

    let sp = poly[0];
    let ep = *poly.last().unwrap();

    let mut farthest_i = 0;
    let mut max_dist = std::f64::NEG_INFINITY;
    for (i, p) in poly.iter().enumerate().take(poly.len() - 1).skip(1) {
        let d = perpendicular_dist(*p, (sp, ep));
        if d > max_dist {
            max_dist = d;
            farthest_i = i;
        }
    }

    if max_dist > eps {
        rdp(r, &poly[..=farthest_i], eps);

        // remove point with max dist, it will be added with the right vec
        r.pop();

        _simplify_with_eps(r, &poly[farthest_i..], eps);
    } else {
        r.push(sp);
        r.push(ep);
    }
}

fn perpendicular_dist(p: (f64, f64), (s, e): ((f64, f64), (f64, f64))) -> f64 {
    let num = ((e.1 - s.1) * p.0 - (e.0 - s.0) * p.1 + e.0 * s.1 - e.1 * s.0).abs();
    let den = ((e.0 - s.0).powi(2) + (e.1 - s.1).powi(2)).sqrt();

    num / den
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_small_paths() {
        assert_eq!(simplify(&[]), vec![]);
        assert_eq!(simplify(&[(1.0, 1.0)]), vec![(1.0, 1.0)]);
        assert_eq!(
            simplify(&[(1.0, 1.0), (2.0, 2.0)]),
            vec![(1.0, 1.0), (2.0, 2.0)]
        );
    }

    #[test]
    fn test_simplify_open_paths() {
        assert_eq!(
            simplify(&[(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)]),
            vec![(1.0, 1.0), (3.0, 3.0)]
        );

        assert_eq!(
            simplify(&[
                (0.0, 0.0),
                (1e-20, 0.0),
                (1e-19, 0.0),
                (1.0, 1.0),
                (1.0, 1.0 + 1e-20),
                (2.0, 2.0),
            ]),
            vec![(0.0, 0.0), (2.0, 2.0)]
        );
    }

    #[test]
    fn test_simplify_closed_paths() {
        assert_eq!(
            simplify(&[(1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (1.0, 1.0)]),
            vec![(1.0, 1.0), (3.0, 3.0), (1.0, 1.0)]
        );

        assert_eq!(
            simplify(&[
                (0.0, 0.0),
                (1e-20, 0.0),
                (1e-19, 0.0),
                (1.0, 1.0),
                (1.0, 1.0 + 1e-20),
                (2.0, 2.0),
                (0.0, 0.0),
            ]),
            vec![(0.0, 0.0), (2.0, 2.0), (0.0, 0.0)]
        );
    }
}
