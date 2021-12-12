#include <GeographicLib/Geodesic.hpp>
#include <GeographicLib/Rhumb.hpp>

const GeographicLib::Rhumb &rhumb = GeographicLib::Rhumb::WGS84();

extern "C" void rhumb_inverse(double lat1, double lon1, double lat2, double lon2, double *distance, double *azimuth)
{
        static const GeographicLib::Rhumb &rhumb = GeographicLib::Rhumb::WGS84();
        double _azimuth;
        double _distance;
        rhumb.Inverse(lat1, lon1, lat2, lon2, _distance, _azimuth);
        *distance = _distance;
        *azimuth = _azimuth;
}

extern "C" void rhumb_direct(const double lat, const double lon, const double azimuth, const double distance, double *out_lat,
                             double *out_lon, double *out_az)
{
        double _out_lat, _out_lon, _out_az;       
        rhumb.Direct(lat, lon, azimuth, distance, _out_lat, _out_lon, _out_az);
        *out_lat = _out_lat;
        *out_lon = _out_lon;
        *out_az = _out_az;
        return;
}
