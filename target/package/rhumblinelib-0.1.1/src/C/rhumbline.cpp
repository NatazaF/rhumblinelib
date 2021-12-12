#include <GeographicLib/Geodesic.hpp>
#include <GeographicLib/Rhumb.hpp>

extern "C" double rhumb_inverse(double lon1, double lat1, double lon2, double lat2)
{
        static const GeographicLib::Rhumb &rhumb = GeographicLib::Rhumb::WGS84();
        double azimuth;
        double distance;
        rhumb.Inverse(lat1, lon1, lat2, lon2, distance, azimuth);
        return distance;
}

extern "C" void rhumb_direct(const double lat, const double lon, const double azimuth, const double distance, double *out_lat,
                             double *out_lon, double *out_az)
{
        double _out_lat, _out_lon, _out_az;       
        static const GeographicLib::Rhumb &rhumb = GeographicLib::Rhumb::WGS84();
        rhumb.Direct(lat, lon, azimuth, distance, _out_lat, _out_lon, _out_az);
        *out_lat = _out_lat;
        *out_lon = _out_lon;
        *out_az = _out_az;
        return;
}
