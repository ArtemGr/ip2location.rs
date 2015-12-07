#![feature(plugin,test)]
#![plugin(power_assert)]

extern crate ip2location;
extern crate test;

use ip2location::Ip2Location;
use test::Bencher;

#[bench] fn ip2country (bencher: &mut Bencher) {
  let ip2location = Ip2Location::open ("/usr/share/ip2location/IP2LOCATION-LITE-DB1.BIN") .expect ("!open");
  bencher.iter (|| {
    assert_eq! (ip2location.ip2country ("8.8.8.8") .expect ("!ip2country"), Some (*b"US"));})}

#[bench] fn location (bencher: &mut Bencher) {
  let ip2location = Ip2Location::open ("/usr/share/ip2location/IP-COUNTRY-REGION-CITY-LATITUDE-LONGITUDE-SAMPLE.BIN") .expect ("!open");
  bencher.iter (|| {
    let location = ip2location.location ("8.8.8.8") .expect ("!location") .expect ("!8.8.8.8");
    power_assert! (location.longitude.round() == -122.0);})}
