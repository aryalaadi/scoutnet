PREFIX = /usr/local
MANPREFIX = ${PREFIX}/share/man

all:
	cargo build --release 

install: 
	mkdir -p ${PREFIX}/bin
	cp -f ./target/release/scoutnet ${PREFIX}/bin
	chmod 755 ${PREFIX}/bin/scoutnet 
	mkdir -p ${MANPREFIX}/man1
	cp ./docs/scoutnet.man1 ${MANPREFIX}/man1/scoutnet.1
	chmod 644 ${MANPREFIX}/man1/scoutnet.1

clean:
	rm -rf target

uninstall:
	rm ${PREFIX}/bin/scoutnet
	rm ${MANPREFIX}/man1/scoutnet.1
