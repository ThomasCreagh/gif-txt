use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let _file_bytes: &[u8] = &[
        // image header
        0x47, 0x49, 0x46, 0x38, 0x39, 0x61,
        // logical screen descriptor
        // 10px   x    10px
        //  _______  __________
        0x0A, 0x00, 0x0A, 0x00, 0x91, 0x00, 0x00, // Global color table
        0xFF, 0xFF, 0xFF, // first color
        0xFF, 0x00, 0x00, // second color
        0x00, 0x00, 0xFF, // third color
        0x00, 0x00, 0x00, // forth color
        // FIRST IMAGE
        // Graphics control Extension
        0x21, 0xF9, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, // Image descriptor
        0x2C, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x0A, 0x00, 0x00, // Image data
        0x02, 0x16, 0x8C, 0x2D, 0x99, 0x87, 0x2A, 0x1C, 0xDC, 0x33, 0xA0, 0x02, 0x75, 0xEC, 0x95,
        0xFA, 0xA8, 0xDE, 0x60, 0x8C, 0x04, 0x91, 0x4C, 0x01, 0x00,
        // SECOND IMAGE
        // Graphics control Extension
        0x21, 0xF9, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, // Image descriptor
        0x2C, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x0A, 0x00, 0x00,
        // Image data
        // 1. min code size, 2. number of bytes in the sub block
        // _1  _2
        0x02, 0x16, 0x8C, 0x2D, 0x99, 0x87, 0x2A, 0x1C, 0xDC, 0x33, 0xA0, 0x02, 0x75, 0xEC, 0x95,
        0xFA, 0xA8, 0xDE, 0x60, 0x8C, 0x04, 0x91, 0x4C, 0x01, 0x00, // tail (0x3B)
        0x3B,
    ];

    /*
        ASCII values have 128 so i will do
        choosing min code size = 7 (#0-127)
        that mean 7 bit (1111111) is the
        size of each color codes.
        for my first test I will make the red [(r)gb] value go from 0 - 127
    */

    let image_header: &[u8] = &[0x47, 0x49, 0x46, 0x38, 0x39, 0x61];
    // 1 110 0 110 = e6
    // let logical_screen_discriptor: &[u8] = &[0x0A, 0x00, 0x0A, 0x00, 0xe6, 0x00, 0x00];
    let logical_screen_discriptor: &[u8] = &[0x0A, 0x00, 0x0A, 0x00, 0x91, 0x00, 0x00];
    let global_color_table: &[u8] = &[0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00];
    // let global_color_table: &mut [u8; 0x180] = &mut [0; 0x180];
    //
    // for i in 0..128 {
    //     global_color_table[i * 3] = i as u8;
    //     global_color_table[(i * 3) + 1] = 0;
    //     global_color_table[(i * 3) + 2] = 0;
    // }

    // let graphical_control_extention: &[u8] = &[0x21, 0xF9, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
    let graphical_control_extention: &[u8] = &[0x21, 0xF9, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
    // let image_descriptor: &[u8] = &[0x2C, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x0A, 0x00, 0x00];
    let image_descriptor: &[u8] = &[0x2C, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x0A, 0x00, 0x00];

    let _ascii_data: &[u8] = &[
        0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20, 0x49, 0x20,
        0x77, 0x72, 0x6F, 0x74, 0x65, 0x20, 0x74, 0x6F, 0x20, 0x73, 0x65, 0x65, 0x20, 0x69, 0x66,
        0x20, 0x6D, 0x79, 0x20, 0x74, 0x78, 0x74, 0x20, 0x74, 0x6F, 0x20, 0x67, 0x69, 0x66, 0x20,
        0x63, 0x6F, 0x6E, 0x76, 0x65, 0x72, 0x74, 0x65, 0x72, 0x20, 0x77, 0x6F, 0x72, 0x6B, 0x73,
        0x2E, 0x20, 0x54, 0x68, 0x65, 0x20, 0x67, 0x69, 0x66, 0x20, 0x69, 0x73, 0x20, 0x63, 0x75,
        0x72, 0x72, 0x65, 0x6E, 0x74, 0x6C, 0x79, 0x20, 0x61, 0x20, 0x31, 0x30, 0x20, 0x78, 0x20,
        0x31, 0x30, 0x20, 0x67, 0x69, 0x66, 0x20, 0x3A, 0x29, 0x2E,
    ];
    let ascii_data: &[u8] = &[
        1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
        1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 
        1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
        1, 1, 1, 0, 0, 0, 0, 2, 2, 2, 
        1, 1, 1, 0, 0, 0, 0, 2, 2, 2, 
        2, 2, 2, 0, 0, 0, 0, 1, 1, 1,
        2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 
        2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 
        2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
        2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
    ];

    let mut code_table: Vec<Vec<u8>> = Vec::new();
    let mut image_data: Vec<u8> = vec![0x02, 0x16];
    let mut code_stream: Vec<u8> = Vec::new();
    // let testin: &mut [u8] = &mut code_stream as &mut [u8];
    // for i in 0..130 {
    //     code_table.push(vec![i as u8]);
    // }
    for i in 0..=5 {
        code_table.push(vec![i as u8]);
    }

    // let next_byte: &mut u8 = &mut 128;
    let next_byte: &mut u8 = &mut 4;
    let insert_bits: &mut u16 = &mut 0;
    let bit_size: &mut f32 = &mut 3.0;
    // let bits_in_next_byte: &mut u8 = &mut 7;
    let bits_in_next_byte: &mut u8 = &mut 3;
    let mut color_code = 0;
    let mut index: usize = 0;

    // decode the ascii_data into the image_data
    for k in 1..ascii_data.len() {
        if !code_table.contains(&ascii_data[index..=k].to_vec()) {
            color_code = code_table
                .iter()
                .position(|r| r == &ascii_data[index..k])
                .unwrap();
            code_table.push(ascii_data[index..=k].to_vec());

            println!("{}", ((code_table.len() as f32).log2().floor() + 1.0));
            println!("{}", *bit_size);

            *insert_bits = ((color_code) as u16) << *bits_in_next_byte as u16;
            *insert_bits = *insert_bits & 0xFF;
            *next_byte = *next_byte | (*insert_bits as u8);

            if (*bits_in_next_byte + *bit_size as u8) >= 8 {
                code_stream.push(*next_byte);
                if *next_byte == 0xDE {
                    println!("break")
                }
                *bits_in_next_byte = (*bits_in_next_byte + *bit_size as u8) % 8;
                *next_byte = color_code as u8 >> (*bit_size as u8 - *bits_in_next_byte);
            } else {
                *bits_in_next_byte = *bits_in_next_byte + *bit_size as u8;
                if ((code_table.len() as f32).log2().floor() + 1.0) > *bit_size {
                    *bit_size = (code_table.len() as f32).log2().floor() + 1.0;
                }
            }
            index = k; 
        }
    }
    *next_byte = *next_byte << (8 - *bits_in_next_byte);
    code_stream.push(*next_byte);
    println!("{:#0x?}", code_stream);

    let total_write_bytes: &[u8] = &[
        image_header,
        logical_screen_discriptor,
        global_color_table, // global_color_table
        graphical_control_extention,
        image_descriptor,
        &image_data,
        &code_stream,
        &[0x00, 0x3bu8],
    ]
    .concat();
    // print!("{:#0x?}", total_write_bytes);
    let mut write_file: File = File::create("out_text.gif")?;
    //
    // println!("The bytes: {:#0x?}", &file_bytes);
    write_file.write_all(&total_write_bytes)?;
    Ok(())
}
