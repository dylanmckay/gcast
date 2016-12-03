use {ApplicationId, Error};

use std::fmt;

use uuid::Uuid;
use json;

/// A float value in [0..1] that represents the magnitude of volume.
#[derive(Clone, PartialEq)]
pub struct VolumeLevel(pub f32);

/// Stores the status of a Cast receiver.
#[derive(Clone, Debug, PartialEq)]
pub struct Status
{
    /// The current volume of the receiver.
    pub volume: Volume,
    /// The currently running applications.
    pub applications: Vec<Application>,
}

/// The volume status of a Cast receiver.
#[derive(Clone, Debug, PartialEq)]
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

/// Information about a currently running application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Application
{
    /// The identifier of the application.
    pub id: ApplicationId,
    /// The display name.
    pub display_name: String,
    pub is_idle_screen: bool,
    /// A unique identifier for the session.
    pub session_id: Uuid,
    /// The status text.
    pub status_text: String,
}

impl Status
{
    /// Reads the status from the payload of a `RECEIVER_STATUS` message.
    pub fn from_json(status: &json::JsonValue) -> Result<Self, Error> {
        let application_data = &status["applications"];

        let volume = Volume::from_json(&status["volume"])?;

        let applications = if application_data.is_array() {
            let result: Result<Vec<_>, _> = application_data.members().map(|app_data| {
                Application::from_json(app_data)
            }).collect();

            result?
        } else {
            Vec::new()
        };

        Ok(Status {
            volume: volume,
            applications: applications,
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

impl Application
{
    pub fn from_json(application: &json::JsonValue) -> Result<Self, Error> {
        let session_id_text = application["sessionId"].as_str().expect("sessionId is missing or not a string");
        let session_id = Uuid::parse_str(session_id_text)?;

        Ok(Application {
            id: ApplicationId(application["appId"].as_str().expect("appId is missing or not a string").to_owned()),
            display_name: application["displayName"].as_str().expect("displayName is missing or not a string").to_owned(),
            is_idle_screen: application["isIdleScreen"].as_bool().expect("isIdleScreen is missing or not a bool").to_owned(),
            session_id: session_id,
            status_text: application["statusText"].as_str().expect("statusText is missing or not a string").to_owned(),
        })
    }
}

impl VolumeLevel
{
    pub fn max() -> Self { VolumeLevel(1.0) }
    pub fn min() -> Self { VolumeLevel(0.0) }

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
#[cfg(test)]
mod test
{
    use super::*;
    use ApplicationId;
    use json;
    use uuid::Uuid;

    fn parse_text(object: &str) -> Status {
        let json = json::parse(object).unwrap();
        Status::from_json(&json).expect("error while parsing status")
    }

    fn parse_json(object: json::JsonValue) -> Status {
        let text = json::stringify(object);
        self::parse_text(&text)
    }

    fn example_volume() -> json::JsonValue {
        object! {
            "controlType" => "attenuation",
            "level" => 0.85,
            "muted" => true,
            "stepInterval" => 0.125
        }
    }

    fn youtube_application() -> json::JsonValue {
        object! {
            "appId" => "YouTube",
            "displayName" => "YouTube",
            "isIdleScreen" => false,
            "sessionId" => "e32a8e92-29cd-4afb-9d2b-6314040022d8",
            "statusText" => "YouTube TV"
        }
    }

    #[test]
    fn parse_example_volume() {
        let status = parse_json(object! { "volume" => example_volume() });

        assert_eq!(status, Status {
            applications: Vec::new(),
            volume: Volume {
                control_type: "attenuation".to_owned(),
                level: VolumeLevel(0.85),
                muted: true,
                step_interval: VolumeLevel(0.125),
            },
        });
    }

    #[test]
    fn parse_simple_volume() {
        let status = parse_text("{ \"volume\": { \"controlType\": \"attenuation\", \"level\": 1, \"muted\": false, \"stepInterval\": 0.1 } }");

        assert_eq!(status, Status {
            applications: Vec::new(),
            volume: Volume {
                control_type: "attenuation".to_owned(),
                level: VolumeLevel::max(),
                muted: false,
                step_interval: VolumeLevel(0.1),
            },
        });
    }

    #[test]
    fn parse_youtube_application() {
        let status = parse_json(object! { "volume" => example_volume(),
                                          "applications" => vec![youtube_application()]});

        assert_eq!(status.applications, &[Application {
            id: ApplicationId("YouTube".to_owned()),
            display_name: "YouTube".to_owned(),
            is_idle_screen: false,
            session_id: Uuid::parse_str("e32a8e92-29cd-4afb-9d2b-6314040022d8").unwrap(),
            status_text: "YouTube TV".to_owned(),
        }]);
    }
}
