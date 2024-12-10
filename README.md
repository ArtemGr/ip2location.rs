#### What's there

Not much by now.

This piece helps opening and closing the database and lets you turn the IP address into ISO2 country code
or fetch a longitude and latitude for it.

#### Requirements
This thing needs a little preparation.
First, the ip2location C library needs to be already installed.
Then you need the rust-bindgen to generate the raw Rust version of the headers for you.

    cd /tmp
    wget https://github.com/chrislim2888/IP2Location-C-Library/archive/refs/tags/8.6.1.tar.gz
    tar -xzf 8.6.1.tar.gz
    rm -f 8.6.1.tar.gz
    cd IP2Location-C-Library-8.6.1/
    touch README
    autoreconf -i -v --force
    export CXXFLAGS=-fPIC
    ./configure --enable-shared --disable-static --with-pic && make && sudo make install && echo Done.
    sudo ldconfig

installs “/usr/local/lib/libIP2Location.so” and “/usr/local/include/IP2Location.h”.

    sudo apt-get install -y libclang-dev clang
    cargo install -f bindgen-cli
    rustup component add rustfmt
    bindgen --output=gen.rs --allowlist-function='IP2Location.*' --allowlist-type='IP2Location.*' /usr/local/include/IP2Location.h

    cd /tmp
    rm -rf IP2Location-C-Library-8.6.1/
    rm -rf rust-bindgen/

Running the unit test(s) needs a database installed:

    mkdir -p /usr/share/ip2location && cd /usr/share/ip2location
    wget https://download.ip2location.com/lite/IP2LOCATION-LITE-DB1.BIN.ZIP
    unzip IP2LOCATION-LITE-DB1.BIN.ZIP IP2LOCATION-LITE-DB1.BIN
    sudo mkdir -p /usr/share/ip2location
    sudo cp IP2LOCATION-LITE-DB1.BIN /usr/share/ip2location/ip2location-lite-db1.bin
    wget https://www.ip2location.com/downloads/sample.bin.db5.zip
    unzip sample.bin.db5.zip IP-COUNTRY-REGION-CITY-LATITUDE-LONGITUDE-SAMPLE.BIN
    chmod 0644 *.BIN
    rm -f *.zip *.ZIP
