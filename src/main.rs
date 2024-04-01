



use std::path::Path;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

const _MASK: [[i32; 3]; 3] = 
        [ 
          [-1, -1, -1], 
          [-1, 9, -1], 
          [-1, -1, -1]
        ];

// const _MASK: [[i8; 3]; 3] = 
// [ 
//     [-1, 0, -1], 
//     [5, -5, 5], 
//     [0, 0, 0]
// ];



fn _color_calc(x: u32, y: u32, index: usize, img: &DynamicImage) -> u8 {
    let mut result: i64
        = ( img.get_pixel(x, y)[index] as i32 * _MASK[1][1] as i32
        + img.get_pixel(x + 1, y)[index] as i32 * _MASK[2][1] as i32
        + img.get_pixel(x, y + 1)[index] as i32 * _MASK[1][2] as i32
        + img.get_pixel(x - 1, y)[index] as i32 * _MASK[0][1] as i32
        + img.get_pixel(x, y - 1)[index] as i32 * _MASK[1][0] as i32
        + img.get_pixel(x - 1, y - 1)[index] as i32 * _MASK[0][0] as i32
        + img.get_pixel(x + 1, y + 1)[index] as i32 * _MASK[2][2] as i32
        + img.get_pixel(x - 1, y + 1)[index] as i32 * _MASK[0][2] as i32
        + img.get_pixel(x + 1, y - 1)[index] as i32 * _MASK[2][0] as i32 ) as i64;

    let wight: i128 = (_MASK[1][1] as i128 * _MASK[2][1] as i128 * _MASK[1][2] as i128 * _MASK[0][1] as i128 * _MASK[1][0] as i128 * _MASK[0][0] as i128 * _MASK[2][2] as i128 * _MASK[0][2] as i128 * _MASK[2][0] as i128 + 1);

    if result > 255 {
        result = (255 / wight) as i64;
    } else {
        if result < 0 {
            result = result * -1;
            if result > 255 {
                result = (255 / wight) as i64;
            }
        }
    }

    result as u8
}


fn main() {


    let path = Path::new("C:\\Users\\amomomogugus\\Desktop\\laba5-images-with-mask\\image_mask\\src\\1.jpg");
    
    let mut _img = ImageReader::open(&path).expect("Failed to open image").decode().expect("Failed to decode image");

    let _width = _img.width();
    let _height = _img.height();
    
    let ref _img_clone = _img.clone();

    for x in 1..(_width-1) {
        for y in 1..(_height-1) {
            let _pixel = Rgba([_color_calc(x, y, 0, _img_clone), _color_calc(x, y, 1, _img_clone), _color_calc(x, y, 2, _img_clone), 255]);
            _img.put_pixel(x, y, _pixel);
        }
    }

    let output_path = Path::new("C:\\Users\\amomomogugus\\Desktop\\laba5-images-with-mask\\image_mask\\src\\output.png");
    _img.save(output_path).unwrap();
        
}
