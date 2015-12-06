use std::ffi::CStr;
use std::io::Write;
use std::mem::uninitialized;
use std::ptr::null_mut;
use std::str::from_utf8_unchecked;
use super::TwoBytes;

pub type uint8_t = u8;
pub type int32_t = i32;
pub type uint32_t = u32;
pub enum FILE {}
include! ("/usr/local/include/ip2location.rs");

/// The high-level wrapper around the ip2location C library.
pub struct Ip2Location (*mut IP2Location);
impl Ip2Location {
  fn open (path: &str) -> Result<Ip2Location, String> {
    let mut pathz: [u8; 256] = unsafe {uninitialized()}; let pathz = format! ("{}\0", path);
    let i2l = unsafe {IP2Location_open (pathz.as_ptr() as *mut i8)};
    if i2l == null_mut() {return ERR! ("!IP2Location_open ({})", path)}
    let rc = unsafe {IP2Location_open_mem (i2l, IP2LOCATION_SHARED_MEMORY)};
    if rc != 0 {return ERR! ("!IP2Location_open_mem")}
    Ok (Ip2Location (i2l))}

  /// Get a country from the IP.
  pub fn ip2country (&self, ip: &str, debug: &Debug) -> Result<Option<TwoBytes>, String> {
    let _ = debug.enter ("ip2country");
    let mut ipz: [u8; 64] = unsafe {uninitialized()}; let ipz = gstring! (ipz, {try_s! (write! (ipz, "{}\0", ip))});
    assert! (self.0 != null_mut());
    let rec = unsafe {IP2Location_get_country_short (self.0, ipz.as_ptr() as *mut i8)};
    if rec == null_mut() {return ERR! ("!IP2Location_get_country_short")}
    if unsafe {(*rec).country_short} == null_mut() {return ERR! ("!country_short")}
    let country = unsafe {CStr::from_ptr ((*rec).country_short)} .to_bytes();
    if country == b"-" {return Ok (None)}
    if country.len() != 2 {return ERR! ("ip2country] !iso2: '{}'.", unsafe {from_utf8_unchecked (country)})}
    let country = TwoBytes ([country[0], country[1]]);
    debug! (debug, "ip2country ({}) = {}", ip, country);
    unsafe {IP2Location_free_record (rec)};
    Ok (Some (country))}}

impl Drop for Ip2Location {
  fn drop (&mut self) {
    unsafe {IP2Location_close (self.0);}
    self.0 = null_mut()}}

/// There's a rumor that ip2location is thread-safe, cf. http://stackoverflow.com/questions/13234711/is-ip2location-thread-safe.
/// From what I've seen, it just reads an mmap-ed file, e.g. immutable.
unsafe impl Sync for Ip2Location {}