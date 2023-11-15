pub mod border;
pub mod relative;

pub fn is_point_in(point: &(u32, u32), area: &(u32, u32, u32, u32)) -> bool {
    let (x, y) = point;
    let (a_x, a_y, a_x1, a_y1) = area;
    !(x < a_x || x > a_x1 || y < a_y || y > a_y1)
}

// Target: (x,y,x1,y1), areas &[(x,y,x1,y1)]
pub fn is_area_cross(target: &(u32, u32, u32, u32), area: &(u32, u32, u32, u32)) -> bool {
    let (ax, ay, ax1, ay1) = area;
    if is_point_in(&(*ax, *ay), target)
        || is_point_in(&(*ax, *ay1), target)
        || is_point_in(&(*ax1, *ay), target)
        || is_point_in(&(*ax1, *ay1), target)
    {
        return true;
    }
    let (ax, ay, ax1, ay1) = target;
    if is_point_in(&(*ax, *ay), area)
        || is_point_in(&(*ax, *ay1), area)
        || is_point_in(&(*ax1, *ay), area)
        || is_point_in(&(*ax1, *ay1), area)
    {
        return true;
    }
    false
}
