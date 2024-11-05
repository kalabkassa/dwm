# dwm - dynamic window manager
# See LICENSE file for copyright and license details.

include config.mk

BIN_DIR = bin
BUILD_DIR = build

SRC = drw.c dwm.c util.c
OBJ = ${SRC:.c=.o}

all: options $(BUILD_DIR)/dwm rust

options:
	@echo dwm build options:
	@echo "CFLAGS   = ${CFLAGS}"
	@echo "LDFLAGS  = ${LDFLAGS}"
	@echo "CC       = ${CC}"

.c.o:
	mkdir -p $(BUILD_DIR)
	${CC} -c ${CFLAGS} $< -o $(BUILD_DIR)/$@

${OBJ}: config.h config.mk

config.h:
	cp config.def.h $@

$(BUILD_DIR)/dwm: ${OBJ}
	mkdir -p $(BIN_DIR)
	${CC} -o $(BIN_DIR)/dwm ${BUILD_DIR}/*.o ${LDFLAGS}

clean:
	rm -f $(BIN_DIR)/dwm ${BUILD_DIR}/*.o dwm-${VERSION}.tar.gz

dist: clean
	mkdir -p dwm-${VERSION}
	cp -R LICENSE Makefile README config.def.h config.mk\
		dwm.1 drw.h util.h ${SRC} dwm.png transient.c dwm-${VERSION}
	tar -cf dwm-${VERSION}.tar dwm-${VERSION}
	gzip dwm-${VERSION}.tar
	rm -rf dwm-${VERSION}

rust:
	rustc panelfilling.rs --out-dir $(BIN_DIR)/

install: all
	rm -fr ~/.dwm
	mkdir -p ~/.dwm
	cp -f dotfiles/.dwm/* ~/.dwm/
	mkdir -p ${DESTDIR}${PREFIX}/bin
	sudo cp -f $(BIN_DIR)/dwm ${DESTDIR}${PREFIX}/bin
	sudo chmod 755 ${DESTDIR}${PREFIX}/bin/dwm
	sudo cp -f $(BIN_DIR)/panelfilling ${DESTDIR}${PREFIX}/bin
	sudo chmod 755 ${DESTDIR}${PREFIX}/bin/panelfilling
	mkdir -p ${DESTDIR}${MANPREFIX}/man1
	sudo sed "s/VERSION/${VERSION}/g" < dwm.1 > sudo ${DESTDIR}${MANPREFIX}/man1/dwm.1
	sudo chmod 644 ${DESTDIR}${MANPREFIX}/man1/dwm.1
	

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/dwm\
		${DESTDIR}${MANPREFIX}/man1/dwm.1

.PHONY: all options clean dist install uninstall
