//! Sovereign telemetry constants and harmonic metadata helpers.

use crate::models::kompress::{BONOBO_HARMONIC_PQ, TECHNO_ZEN_BONOBO_PQ};

/// Custom HTTP response header exposing Rung 0 Bonobo harmonic (`"3/2"`).
pub const BONOBO_HARMONIC_HEADER: &str = "x-bonobo-harmonic-pq";

/// Custom HTTP response header exposing Techno-Zen-Bonobo compound harmonic (`"9/8"`).
pub const TECHNO_ZEN_HEADER: &str = "x-techno-zen-bonobo-pq";

/// Return canonical header key-value pairs for sovereign telemetry verification.
pub const fn harmonic_headers() -> [(&'static str, &'static str); 2] {
    [(BONOBO_HARMONIC_HEADER, BONOBO_HARMONIC_PQ), (TECHNO_ZEN_HEADER, TECHNO_ZEN_BONOBO_PQ)]
}
