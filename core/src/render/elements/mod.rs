pub mod border;
pub mod relative;

pub fn is_point_in(point: (i32, i32), area: (i32, i32, i32, i32)) -> bool {
    let (x, y) = point;
    let (a_x, a_y, a_x1, a_y1) = area;
    !(x < a_x || x > a_x1 || y < a_y || y > a_y1)
}

// Target: (x,y,x1,y1), areas &[(x,y,x1,y1)]
pub fn is_area_cross(target: (i32, i32, i32, i32), areas: &[(i32, i32, i32, i32)]) -> bool {
    for (f_x, f_y, f_x1, f_y1) in areas.iter() {
        if is_point_in((*f_x, *f_y), target)
            || is_point_in((*f_x, *f_y1), target)
            || is_point_in((*f_x1, *f_y), target)
            || is_point_in((*f_x1, *f_y1), target)
        {
            return true;
        }
    }
    false
}
