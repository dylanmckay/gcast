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

impl Status
{
    /// Reads the status from the payload of a `RECEIVER_STATUS` message.
    pub fn from_json(status: &json::JsonValue) -> Result<Self, Error> {
        let volume = Volume::from_json(&status["volume"])?;

        Ok(Status {
            volume: volume,
        })
    }
}

impl Volume
{
    pub fn from_json(volume: &json::JsonValue) -> Result<Self, Error> {
        Ok(Volume {
            control_type: volume["controlType"].as_str().expect("controlType is missing or not a string").to_owned(),
            level: VolumeLevel(volume["level"].as_f32().expect("level is missing or not a float")),
            muted: volume["muted"].as_bool().expect("muted is missing or not a bool"),
            step_interval: VolumeLevel(volume["stepInterval"].as_f32().expect("stepInterval is missing or not a float")),
        })
    }
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
        write!(fmt, "{:0}%", self.percentage())
    }
}
