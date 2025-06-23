pub fn get_y_of_ground(window_height: f32) -> f32 {
    let bottom_y = -(window_height / 2.0);

    let ten_percent_of_window_height = bottom_y / 5.0;
    let one_percent_of_window_height = ten_percent_of_window_height / 10.0;

    // This is at 90.5 percent of the top of the image, but center is at 0x0, so we dont multiply
    // by 90.5, but need to substract 50.0
    let y_coordinate_of_ground = one_percent_of_window_height * 41.5;
    return y_coordinate_of_ground;
}
