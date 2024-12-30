use std::{error::Error, fmt::Debug};
use crate::{input::source::{SourceInputDevice, SourceOutputDevice}, udev::device::UdevDevice};
/// MultiColorChassis source device implementation
pub struct MultiColorChassis {
    device_info: UdevDevice,
}
impl MultiColorChassis {
    /// Create a new MultiColorChassis source device with the given udev
    /// device information
    pub fn new(device_info: UdevDevice) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self { device_info })
    }
}

impl Debug for MultiColorChassis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MultiColorChassis").finish()
    }
}

impl SourceInputDevice for MultiColorChassis {
    fn poll(&mut self) -> Result<Vec<crate::input::event::native::NativeEvent>, crate::input::source::InputError> {
        Ok(Vec::new())
    }

    fn get_capabilities(&self) -> Result<Vec<crate::input::capability::Capability>, crate::input::source::InputError> {
        Ok(Vec::new())
    }
}

impl SourceOutputDevice for MultiColorChassis {
    fn write_event(&mut self, event: crate::input::output_event::OutputEvent) -> Result<(), crate::input::source::OutputError> {
        //log::trace!("Received output event: {event:?}");
        let _ = event;
        Ok(())
    }

    fn upload_effect(&mut self, effect: evdev::FFEffectData) -> Result<i16, crate::input::source::OutputError> {
        //log::trace!("Received upload effect: {effect:?}");
        let _ = effect;
        Ok(-1)
    }

    fn update_effect(&mut self, effect_id: i16, effect: evdev::FFEffectData) -> Result<(), crate::input::source::OutputError> {
        //log::trace!("Received update effect: {effect_id:?} {effect:?}");
        let _ = effect;
        let _ = effect_id;
        Ok(())
    }

    fn erase_effect(&mut self, effect_id: i16) -> Result<(), crate::input::source::OutputError> {
        //log::trace!("Received erase effect: {effect_id:?}");
        let _ = effect_id;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), crate::input::source::OutputError> {
        Ok(())
    }
}
