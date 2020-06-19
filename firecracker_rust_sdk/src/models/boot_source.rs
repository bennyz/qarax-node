/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 0.21.0
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// BootSource : Boot source descriptor.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BootSource {
    /// Host level path to the kernel image used to boot the guest
    #[serde(rename = "kernel_image_path")]
    pub kernel_image_path: String,
    /// Kernel boot arguments
    #[serde(rename = "boot_args")]
    pub boot_args: String,
    /// Host level path to the initrd image used to boot the guest
    #[serde(rename = "initrd_path", skip_serializing_if = "Option::is_none")]
    pub initrd_path: Option<String>,
}

impl BootSource {
    /// Boot source descriptor.
    pub fn new(kernel_image_path: String, boot_args: String) -> BootSource {
        BootSource {
            boot_args,
            kernel_image_path,
            initrd_path: None,
        }
    }
}


