extern crate ip2location;

use ip2location::Ip2Location;

#[test] fn ip2country() {
  let ip2location = Ip2Location::open ("/usr/share/ip2location/IP2LOCATION-LITE-DB1.BIN") .expect ("!open");
  assert_eq! (ip2location.ip2country ("127.0.0.1") .expect ("!ip2country"), None);
  assert_eq! (ip2location.ip2country ("8.8.8.8") .expect ("!ip2country"), Some (*b"US"));}

#[test] fn location() {
  let ip2location = Ip2Location::open ("/usr/share/ip2location/IP-COUNTRY-REGION-CITY-LATITUDE-LONGITUDE-SAMPLE.BIN") .expect ("!open");
  assert_eq! (ip2location.ip2country ("127.0.0.1") .expect ("!ip2country"), None);
  assert_eq! (ip2location.ip2country ("8.8.8.8") .expect ("!ip2country"), Some (*b"US"));}
