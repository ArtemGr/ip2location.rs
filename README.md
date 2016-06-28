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
    wget http://www.ip2location.com/downloads/ip2location-c-7.0.2.tar.gz
    tar -xzf ip2location-c-*.tar.gz
    rm -f ip2location-c-*.tar.gz
    cd ip2location-c-*
    ./configure --disable-static && make && make install && echo Done.
    ldconfig

    apt-get install -y libclang-dev
    cd /tmp
    git clone https://github.com/crabtw/rust-bindgen.git
    cd /rust-bindgen
    cargo build --release
    bindgen --link=IP2Location --match=IP2Location.h --match=IP2Loc_DBInterface.h --output=/usr/local/include/ip2location.rs /usr/local/include/IP2Location.h
    perl -i.tmp -0pe 's/#!\[allow\(.*?\)\]//s' /usr/local/include/ip2location.rs

Running the unit test(s) needs a database installed:

    mkdir -p /usr/share/ip2location && cd /usr/share/ip2location
    wget http://download.ip2location.com/lite/IP2LOCATION-LITE-DB1.BIN.ZIP
    unzip IP2LOCATION-LITE-DB1.BIN.ZIP IP2LOCATION-LITE-DB1.BIN
    cp IP2LOCATION-LITE-DB1.BIN /usr/share/ip2location/ip2location-lite-db1.bin
    wget http://www.ip2location.com/downloads/sample.bin.db5.zip
    unzip sample.bin.db5.zip IP-COUNTRY-REGION-CITY-LATITUDE-LONGITUDE-SAMPLE.BIN
    chmod 0644 *.BIN
    rm -f *.zip *.ZIP
