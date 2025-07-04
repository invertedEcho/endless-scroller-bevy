pub fn get_y_of_ground(window_height: f32) -> f32 {
    let bottom_y = -(window_height / 2.0);

    let ten_percent_of_window_height = bottom_y / 5.0;
    let one_percent_of_window_height = ten_percent_of_window_height / 10.0;

    // This is at 91.5 percent of the top of the image, but center is at 0x0, so we dont multiply
    // by 91.5, but need to substract 50.0
    let y_coordinate_of_ground = one_percent_of_window_height * 41.5;
    return y_coordinate_of_ground;
}

// TODO: Maybe num_tiles should be stored in our state / as resource?
pub fn get_num_tiles(window_width: f32, scaled_image_width: f32) -> usize {
    (window_width / scaled_image_width).ceil() as usize + 1
}

pub fn get_left_edge_of_window(window_width: f32) -> f32 {
    -(window_width / 2.0)
}

pub fn get_asset_path_formatted_for_bevy(path: String) -> String {
    if !path.starts_with("assets/") {
        return path;
    }
    return path.split("/").collect::<Vec<&str>>()[1..].join("/");
}
