extern crate vulkano;

use std::sync::Arc;

use vulkano::{
    VulkanLibrary,
    instance::{Instance, InstanceCreateInfo},
    device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags, DeviceExtensions, physical::{PhysicalDevice, PhysicalDeviceType}}, swapchain::{Swapchain, SwapchainCreateInfo}, image::ImageUsage, buffer::BufferContents
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    window::WindowBuilder,
    event_loop::{
        EventLoop,
        ControlFlow
    },
    event::{
        Event,
        WindowEvent
    }
};


fn main() {
    let library: Arc<VulkanLibrary> = VulkanLibrary::new()
        .expect("No vulkan library/DLL");

    let required_extensions = vulkano_win::required_extensions(&library);

    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            ..Default::default()
        }
    )
    .expect("Failed to create instance");

    let event_loop = EventLoop::new();

    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = select_physical_device(&instance, &surface, &device_extensions);

    let (device,  mut queues) = Device::new(
        physical_device.clone(),
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo{
                queue_family_index,
                ..Default::default()
            }],
            enabled_extensions: device_extensions,
            ..Default::default()
        }
    )
    .unwrap();

    let caps = physical_device
        .surface_capabilities(&surface, Default::default())
        .expect("Failed to get surface capabilities");

    //surface.window();

    let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();

    let image_format = Some(
        physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0,
    );

    let image_extent = caps.current_extent.unwrap_or([640, 480]);

    let (swapchain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: caps.min_image_count + 1,
            image_format,
            image_extent,
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            ..Default::default()
        },
    )
    .unwrap();

    event_loop.run(|event, _, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit
            },
            _ => ()
        }
    });
}

fn select_physical_device(instance: &Arc<Instance>, surface: &Arc<vulkano::swapchain::Surface>, device_extensions: &DeviceExtensions) -> (Arc<PhysicalDevice>, u32) {
    return instance
        .enumerate_physical_devices()
        .expect("Could not enumerate devices")
        .filter(|p| {
            p.supported_extensions().contains(device_extensions)})
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.contains(QueueFlags::GRAPHICS) &&
                    p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|q| (p, q as u32))
        })
        .min_by_key(|(p, _)| {
            match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                _ => 4,
            }
        })
        .expect("No devices available");
}

#[derive(BufferContents)]
#[repr(C)]
struct BufferObject {
    foo: u32,
    bar: u32
}
