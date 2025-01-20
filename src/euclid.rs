pub fn extended_euclidean(a: isize, b: isize) -> Option<(usize, isize, isize)> {
    if a == 0 && b == 0 {
        return None;
    }

    if b == 0 {
        let s = if a > 0 { 1 } else { -1 };
        return Some((a.unsigned_abs(), s, 0));
    }

    if a % b == 0 {
        let t = if b > 0 { 1 } else { -1 };
        return Some((b.unsigned_abs(), 0, t));
    }

    let (mut x, mut y) = (a, b);
    let (mut s1, mut s2) = (1, 0);
    let (mut t1, mut t2) = (0, 1);
    let (mut s, mut t) = (0, 0);

    while x % y != 0 {
        let q = x.div_euclid(y);
        let r = x.rem_euclid(y);

        s = s1 - q * s2;
        t = t1 - q * t2;
        s1 = s2;
        s2 = s;
        t1 = t2;
        t2 = t;

        x = y;
        y = r;
    }

    Some((y as usize, s, t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eea_48_neg30() {
        let (gcm, s, t) = extended_euclidean(48, -30).unwrap();
        assert_eq!((gcm, s, t), (6, 2, 3));
    }
}
