extern crate ip2location;

use ip2location::Ip2Location;

#[test] fn ip2country() {
  let ip2location = Ip2Location::open ("/usr/share/ip2location/ip2location-lite-db1.bin") .expect ("!open");
  assert_eq! (ip2location.ip2country ("127.0.0.1") .expect ("!ip2country"), None);
  assert_eq! (ip2location.ip2country ("8.8.8.8") .expect ("!ip2country"), Some (*b"US"));}
