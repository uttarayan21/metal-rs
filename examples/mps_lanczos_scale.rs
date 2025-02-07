use metal::*;
use mps::*;
use objc::{rc::autoreleasepool, runtime::YES};

fn main() {
    autoreleasepool(|| foo());
}
fn foo() {
    // Create the Metal device
    let device = Device::system_default().expect("No Metal device found");

    // Verify MPS support
    assert!(
        mps_supports_device(&device),
        "MPS is not supported on this device"
    );

    // Create a command queue
    let command_queue = device.new_command_queue();

    // Create source texture (example dimensions)
    let source_width = 1024;
    let source_height = 768;
    let descriptor = TextureDescriptor::new();
    descriptor.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
    descriptor.set_width(source_width);
    descriptor.set_height(source_height);
    descriptor.set_usage(MTLTextureUsage::ShaderRead);

    let source_texture = device.new_texture(&descriptor);

    // Here you would normally load your image data into source_texture
    // For this example, we'll skip actual image loading

    // Create destination texture (scaled down size)
    let dest_width = 8192;
    let dest_height = 6144;
    let dest_descriptor = TextureDescriptor::new();
    dest_descriptor.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
    dest_descriptor.set_width(dest_width);
    dest_descriptor.set_height(dest_height);
    dest_descriptor.set_usage(MTLTextureUsage::ShaderWrite);

    let dest_texture = device.new_texture(&dest_descriptor);

    // Create the Lanczos scale filter
    let lanczos =
        ImageLanczosScale::from_device(&device).expect("Failed to create Lanczos scale filter");

    // panic!();
    // Configure the filter
    // lanczos.set_number_of_lobes(3); // Higher values = better quality but slower

    // Set up the scale transform
    let transform = MPSScaleTransform::new(
        dest_width as f32 / source_width as f32,   // scale_x
        dest_height as f32 / source_height as f32, // scale_y
        0.0,                                       // translate_x
        0.0,                                       // translate_y
    );
    lanczos.set_scale_transform(transform);

    // Create and encode the scaling operation
    let command_buffer = command_queue.new_command_buffer();
    lanczos.encode_to_command_buffer(command_buffer, &source_texture, &dest_texture);

    // Commit the command buffer
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // At this point, dest_texture contains the scaled image
    println!(
        "Image scaled from {}x{} to {}x{}",
        source_width, source_height, dest_width, dest_height
    );

    // drop(pool);
}
