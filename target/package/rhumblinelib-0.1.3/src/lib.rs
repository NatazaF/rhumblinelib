#[repr(C)]
pub struct Rhumb {}

extern "C" {
    fn rhumb_inverse(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64;
    fn rhumb_direct(
        lat: f64,
        lon: f64,
        azimuth: f64,
        distance: f64,
        out_latitude: *mut f64,
        out_longitude: *mut f64,
        out_azimuth: *mut f64,
    ) -> ();
}

impl Rhumb {
    pub fn rhumbline_inverse(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let distance = unsafe { rhumb_inverse(lat1, lon1, lat2, lon2) };
        distance
    }

    pub fn rhumbline_direct(lat: f64, lon: f64, azimuth: f64, distance: f64) -> (f64, f64, f64) {
        let mut out_latitude: f64 = 1.0;
        let mut out_longitude: f64 = 1.0;
        let mut out_azimuth: f64 = 1.0;

        unsafe {
            rhumb_direct(
                lat,
                lon,
                azimuth,
                distance,
                &mut out_latitude as *mut f64,
                &mut out_longitude as *mut f64,
                &mut out_azimuth as *mut f64,
            )
        };
        (out_latitude, out_longitude, out_azimuth)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_rhumbline_inverse() {
        use crate::Rhumb;
        let (lat1, lon1) = (52.0, 4.0);
        let (lat2, lon2) = (52.0635499025, 4.10303268597);
        let distance = Rhumb::rhumbline_inverse(lat1, lon1, lat2, lon2);

        assert_eq!(distance as u32, 10000);
    }

    #[test]
    fn test_rhumbline_direct() {
        use crate::Rhumb;
        let (lat, lon) = (52.0, 4.0);
        let az = 45.0;
        let dist = 10000.0;
        let (out_latitude, out_longitude, _out_azimuth) = Rhumb::rhumbline_direct(lat, lon, az, dist);
        assert_eq!((out_latitude * 100.0).round() / 100.0, 52.06);
        assert_eq!((out_longitude * 100.0).round() / 100.0, 4.10);
    }
}
