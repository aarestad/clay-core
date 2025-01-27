use crate::{Context, Push};

/// Something that could store its data on a device.
pub trait Store {
    /// The data that is stored on the device.
    type Data: Push;

    /// Creates device data.
    fn create_data(&self, context: &Context) -> crate::Result<Self::Data>;

    /// Updates device data.
    fn update_data(&self, context: &Context, data: &mut Self::Data) -> crate::Result<()>;
}
