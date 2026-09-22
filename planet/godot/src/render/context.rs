use ash::vk::Handle;
use godot::classes::rendering_device::DriverResource;
// render/context.rs
use godot::prelude::*;
use godot::classes::{RenderingServer, RenderingDevice};
use ash::vk;

#[derive(Debug)]
pub enum VulkanError {
    RenderingDeviceUnavailable,
    EntryLoadFailed,
    CommandPoolCreationFailed
}

impl std::fmt::Display for VulkanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VulkanError::RenderingDeviceUnavailable => {
                write!(f, "RenderingDevice unavailable (no GPU / headless?)")
            }

            VulkanError::EntryLoadFailed => {
                write!(f, "Failed to load ash entry")
            }

            VulkanError::CommandPoolCreationFailed => {
                write!(f, "Failed to create the compute command pool")
            }
        }
    }
}

impl std::error::Error for VulkanError {}


struct RawDriverHandles {
    instance: u64,
    physical_device: u64,
    device: u64,
    queue_family: u32,
    queue: u64,
}

fn fetch_driver_handles(rd: &Gd<RenderingDevice>) -> RawDriverHandles {
    RawDriverHandles {
        instance: rd.get_driver_resource(DriverResource::VULKAN_INSTANCE, Rid::Invalid, 0) as u64,
        physical_device: rd.get_driver_resource(DriverResource::VULKAN_PHYSICAL_DEVICE, Rid::Invalid, 0) as u64,
        device: rd.get_driver_resource(DriverResource::VULKAN_DEVICE, Rid::Invalid, 0) as u64,
        queue_family: rd.get_driver_resource(DriverResource::VULKAN_QUEUE_FAMILY_INDEX, Rid::Invalid, 0) as u32,
        queue: rd.get_driver_resource(DriverResource::VULKAN_QUEUE, Rid::Invalid, 0) as u64,
    }
}

struct AdoptedHandles {
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue: vk::Queue,
}

unsafe fn adopt_driver_handles(
    entry: &ash::Entry,
    raw: &RawDriverHandles,
) -> AdoptedHandles {
    let instance_handle = vk::Instance::from_raw(raw.instance);
    let instance = unsafe {ash::Instance::load(entry.static_fn(), instance_handle)};

    let physical_device = vk::PhysicalDevice::from_raw(raw.physical_device);

    let device_handle = vk::Device::from_raw(raw.device);
    let device = unsafe {ash::Device::load(instance.fp_v1_0(), device_handle)};

    let queue = vk::Queue::from_raw(raw.queue);

    AdoptedHandles { instance, physical_device, device, queue }
}

fn create_command_pool(
    device: &ash::Device,
    queue_family: u32,
) -> Result<vk::CommandPool, VulkanError> {
    let info = vk::CommandPoolCreateInfo::default()
        .queue_family_index(queue_family)
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

    unsafe { device.create_command_pool(&info, None) }
        .map_err(|_| VulkanError::CommandPoolCreationFailed)
}

pub struct VulkanContext {
    pub instance: ash::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub queue_family: u32,
    pub queue: std::sync::Mutex<vk::Queue>,
    
    pub command_pool: vk::CommandPool,
}

impl VulkanContext {

    pub fn new() -> Result<VulkanContext, VulkanError> {
        let rd = RenderingServer::singleton()
            .get_rendering_device()
            .ok_or(VulkanError::RenderingDeviceUnavailable)?;

        let entry = unsafe { ash::Entry::load() }
            .map_err(|_| VulkanError::EntryLoadFailed)?;

        let raw_handles = fetch_driver_handles(&rd);
        let adopted_handles = unsafe {adopt_driver_handles(&entry, &raw_handles)};

        let command_pool = create_command_pool(&adopted_handles.device, raw_handles.queue_family)
            .map_err(|_| VulkanError::CommandPoolCreationFailed)?;

        Ok(VulkanContext {
            instance: adopted_handles.instance,
            physical_device: adopted_handles.physical_device,
            device: adopted_handles.device,
            queue_family: raw_handles.queue_family,
            queue: std::sync::Mutex::new(adopted_handles.queue),
            command_pool: command_pool
        })
    }

    pub fn shutdown(&mut self) {
        godot_print!("Shuting down vulkan context...");

        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);
        };
    }
}