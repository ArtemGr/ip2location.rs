fn main() {
  println! ("cargo::rustc-link-search=native=/usr/local/lib");
  println! ("cargo::rustc-link-lib=IP2Location");
  println! ("cargo::rerun-if-changed=build.rs");}
