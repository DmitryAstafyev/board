pub mod relative;

pub fn is_point_in(point: &(u32, u32), area: &(u32, u32, u32, u32)) -> bool {
    let (x, y) = point;
    let (a_x, a_y, a_x1, a_y1) = area;
    !(x < a_x || x > a_x1 || y < a_y || y > a_y1)
}

// Target: (x,y,x1,y1), areas &[(x,y,x1,y1)]
pub fn is_area_cross(target: &(u32, u32, u32, u32), area: &(u32, u32, u32, u32)) -> bool {
    let (ax, ay, ax1, ay1) = area;
    let (tx, ty, tx1, ty1) = target;
    if ty1 < ay || ty > ay1 || tx1 < ax || tx > ax1 {
        return false;
    }
    true
}
