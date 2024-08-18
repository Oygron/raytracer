pub struct Point {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}
pub struct Vector {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn access_pt_coord() {
        let pt = Point{x:1.0, y:2.0, z:3.0};
        assert_eq!(pt.x, 1.0);
        assert_eq!(pt.y, 2.0);
        assert_eq!(pt.z, 3.0);
    }

    #[test]
    fn access_vect_coord() {
        let v = Vector{x:1.0, y:2.0, z:3.0};
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }
}
