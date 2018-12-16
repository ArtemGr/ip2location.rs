#### What's there

Not much by now.

This piece helps opening and closing the database and lets you turn the IP address into ISO2 country code
or fetch a longitude and latitude for it.

#### Requirements
This thing needs a little preparation.
First, the ip2location C library needs to be already installed.
Then you need the rust-bindgen to generate the raw Rust version of the headers for you.
On my Debian Jessie it looks like that:

    cd /tmp
    wget https://github.com/chrislim2888/IP2Location-C-Library/archive/8.0.7.tar.gz
    tar -xzf 8.0.7.tar.gz
    rm -f 8.0.7.tar.gz
    cd IP2Location-C-Library-8.0.7/
    touch README
    autoreconf -i -v --force
    ./configure --disable-static && make && sudo make install && echo Done.
    sudo ldconfig

    sudo apt-get install -y libclang-dev clang
    cd /tmp
    git clone --progress --depth 1 https://github.com/ArtemGr/rust-bindgen.git
    cd rust-bindgen
    cargo build --release
    sudo target/release/bindgen --link=IP2Location --match=IP2Location.h --match=IP2Loc_DBInterface.h --output=/usr/local/include/ip2location.rs /usr/local/include/IP2Location.h
    sudo perl -i.tmp -0pe 's/#!\[allow\(.*?\)\]//s' /usr/local/include/ip2location.rs

    cd /tmp
    rm -rf IP2Location-C-Library-8.0.7/
    rm -rf rust-bindgen/

Running the unit test(s) needs a database installed:

    mkdir -p /usr/share/ip2location && cd /usr/share/ip2location
    wget http://download.ip2location.com/lite/IP2LOCATION-LITE-DB1.BIN.ZIP
    unzip IP2LOCATION-LITE-DB1.BIN.ZIP IP2LOCATION-LITE-DB1.BIN
    cp IP2LOCATION-LITE-DB1.BIN /usr/share/ip2location/ip2location-lite-db1.bin
    wget http://www.ip2location.com/downloads/sample.bin.db5.zip
    unzip sample.bin.db5.zip IP-COUNTRY-REGION-CITY-LATITUDE-LONGITUDE-SAMPLE.BIN
    chmod 0644 *.BIN
    rm -f *.zip *.ZIP
