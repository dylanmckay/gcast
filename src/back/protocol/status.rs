use Error;

use std::fmt;

use json;

#[derive(Clone, PartialEq)]
pub struct VolumeLevel(pub f32);

/// Stores the status of a Cast receiver.
#[derive(Clone, Debug)]
pub struct Status
{
    pub volume: Volume,
}

/// The volume status of a Cast receiver.
#[derive(Clone, Debug)]
pub struct Volume
{
    /// The volume control type.
    ///
    /// Possible values:
    /// * `"attenuation"`
    pub control_type: String,
    /// The current volume level.
    pub level: VolumeLevel,
    /// Whether or not the receiver is muted.
    pub muted: bool,
    /// The amount we increase/decrease the volume in one step.
    pub step_interval: VolumeLevel,
}

impl VolumeLevel
{
    /// Gets the volume percentage.
    pub fn percentage(&self) -> f32 {
        self.0 * 100.0
    }
}

impl fmt::Debug for VolumeLevel
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:0}", self.percentage())
    }
}
