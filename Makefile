# FontLens — Makefile
# Usage:
#   make       — build release binary + assemble .deb into dist/
#   make clean — remove all build artifacts, leaving source only

PACKAGE   := fontlens
VERSION   := 1.0.0
ARCH      := amd64
DEB_NAME  := $(PACKAGE)_$(VERSION)_$(ARCH).deb
DEB_DIR   := dist/$(PACKAGE)-deb
DIST_DIR  := dist

.PHONY: all clean

all:
	cargo build --release
	@echo "  [FontLens] Assembling .deb package..."
	@rm -rf $(DEB_DIR)
	@mkdir -p $(DEB_DIR)/DEBIAN
	@mkdir -p $(DEB_DIR)/usr/bin
	@mkdir -p $(DEB_DIR)/usr/share/applications
	@mkdir -p $(DEB_DIR)/usr/share/icons
	@mkdir -p $(DIST_DIR)
	@cp target/release/$(PACKAGE) $(DEB_DIR)/usr/bin/$(PACKAGE)
	@chmod 755 $(DEB_DIR)/usr/bin/$(PACKAGE)
	@cp dist/DEBIAN/control  $(DEB_DIR)/DEBIAN/control
	@cp dist/DEBIAN/postinst $(DEB_DIR)/DEBIAN/postinst
	@cp dist/DEBIAN/postrm   $(DEB_DIR)/DEBIAN/postrm
	@chmod 755 $(DEB_DIR)/DEBIAN/postinst
	@chmod 755 $(DEB_DIR)/DEBIAN/postrm
	@cp dist/usr/share/applications/$(PACKAGE).desktop $(DEB_DIR)/usr/share/applications/$(PACKAGE).desktop
	@cp -r dist/usr/share/icons/. $(DEB_DIR)/usr/share/icons/
	@dpkg-deb --build $(DEB_DIR) $(DIST_DIR)/$(DEB_NAME)
	@echo "  [FontLens] Built: $(DIST_DIR)/$(DEB_NAME)"

clean:
	cargo clean
	@rm -rf $(DEB_DIR)
	@rm -f $(DIST_DIR)/*.deb
	@echo "  [FontLens] Clean — source only."
