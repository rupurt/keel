//! Mission transition specifications

use crate::domain::state_machine::mission::MissionStatus;

/// Which timestamp fields to update during a mission transition.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MissionTimestampUpdates {
    /// Update the updated_at field to current datetime
    pub updated_at: bool,
    /// Update the activated_at field to current datetime
    pub activated_at: bool,
    /// Update the achieved_at field to current datetime
    pub achieved_at: bool,
    /// Update the verified_at field to current datetime
    pub verified_at: bool,
}

impl MissionTimestampUpdates {
    pub const fn updated_only() -> Self {
        Self {
            updated_at: true,
            activated_at: false,
            achieved_at: false,
            verified_at: false,
        }
    }

    pub const fn with_activated() -> Self {
        Self {
            updated_at: true,
            activated_at: true,
            achieved_at: false,
            verified_at: false,
        }
    }

    pub const fn with_achieved() -> Self {
        Self {
            updated_at: true,
            activated_at: false,
            achieved_at: true,
            verified_at: false,
        }
    }

    pub const fn with_verified() -> Self {
        Self {
            updated_at: true,
            activated_at: false,
            achieved_at: false,
            verified_at: true,
        }
    }
}

/// Specification for a mission state transition.
#[derive(Debug, Clone)]
pub struct MissionTransitionSpec {
    /// Human-readable name for error messages
    pub name: &'static str,
    /// Valid source stages
    pub from: &'static [MissionStatus],
    /// Target stage
    pub to: MissionStatus,
    /// Which timestamp fields to update
    pub timestamps: MissionTimestampUpdates,
}

pub mod mission_transitions {
    use super::*;

    pub const ACTIVATE: MissionTransitionSpec = MissionTransitionSpec {
        name: "activate",
        from: &[MissionStatus::Defining],
        to: MissionStatus::Active,
        timestamps: MissionTimestampUpdates::with_activated(),
    };

    pub const ACHIEVE: MissionTransitionSpec = MissionTransitionSpec {
        name: "achieve",
        from: &[MissionStatus::Active],
        to: MissionStatus::Achieved,
        timestamps: MissionTimestampUpdates::with_achieved(),
    };

    pub const VERIFY: MissionTransitionSpec = MissionTransitionSpec {
        name: "verify",
        from: &[MissionStatus::Achieved],
        to: MissionStatus::Verified,
        timestamps: MissionTimestampUpdates::with_verified(),
    };

    pub const PAUSE: MissionTransitionSpec = MissionTransitionSpec {
        name: "pause",
        from: &[MissionStatus::Active],
        to: MissionStatus::Paused,
        timestamps: MissionTimestampUpdates::updated_only(),
    };

    pub const RESUME: MissionTransitionSpec = MissionTransitionSpec {
        name: "resume",
        from: &[MissionStatus::Paused],
        to: MissionStatus::Active,
        timestamps: MissionTimestampUpdates::updated_only(),
    };

    pub const ABANDON: MissionTransitionSpec = MissionTransitionSpec {
        name: "abandon",
        from: &[MissionStatus::Active, MissionStatus::Paused],
        to: MissionStatus::Abandoned,
        timestamps: MissionTimestampUpdates::updated_only(),
    };
}
