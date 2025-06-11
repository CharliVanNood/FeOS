use crate::vec::BigVec;
use crate::window::get_int;
use crate::window::get_screen_size;
use crate::window::get_rgb;
use crate::window::set_pixel;

pub fn render_image(image_data: BigVec) {
    let window_size = get_screen_size();
    let window_offset_x = window_size.0 / 2;

    let mut image_width = get_int([image_data.get(0), image_data.get(1), image_data.get(2)]) as usize;
    let mut image_height = get_int([image_data.get(3), image_data.get(4), image_data.get(5)]) as usize;

    let window_width = window_size.0 - window_offset_x;
    let window_height = window_size.1;

    let mut image_padding_x = 0;
    if image_width > window_width {
        image_padding_x = image_width - window_width;
        image_width = window_width;
    }
    if image_height > window_height {
        image_height = window_height
    }

    let image_start_x = window_width/2 - image_width/2;
    let image_start_y = window_height/2 - image_height/2;
    let image_end_x = image_start_x + image_width;
    let image_end_y = image_start_y + image_height;
    let mut char = 6;

    for y in (0..window_size.1).rev() {
        if (y >= image_start_y) && (y < image_end_y) {
            for x in 0..window_width {
                if (x >= image_start_x) && (x < image_end_x + image_padding_x) {
                    if x < image_end_x {
                        let red = get_int([image_data.get(char),image_data.get(char+1),image_data.get(char+2)]);
                        let green = get_int([image_data.get(char+3),image_data.get(char+4),image_data.get(char+5)]);
                        let blue = get_int([image_data.get(char+6),image_data.get(char+7),image_data.get(char+8)]);
                        char += 9;

                        let color = get_rgb(red, green, blue);
                        set_pixel(x, y, color);
                    } else {
                        char += 9;
                    }
                }
            }
        }
    }

    image_data.remove();
}