use super::bindgen;

pub unsafe fn init_srs(points_buf: &[u8], num_points: u32, g2_point_buf: &[u8]) {
    bindgen::srs_init_srs(
        points_buf.as_ptr(),
        &num_points.to_be(),
        g2_point_buf.as_ptr(),
    );
}
