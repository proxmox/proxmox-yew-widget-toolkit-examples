# This makfile is a helper to update the external repositories
# which are included as git subtree.

all:

PWT_ASSETS_DIR="pwt-assets"
PWT_ASSETS_VERSION="bdac29e7923e4a9acbd4675f6206c5dea5cfa4d1"
PWT_ASSETS_REPO="https://git.proxmox.com/rust/proxmox-yew-wiget-toolkit-assets.git"
PWT_ASSETS_REPO="gitolite3@proxdev.maurer-it.com:yew/proxmox-yew-widget-toolkit-assets"

update-pwt-assets:
	git subtree pull --squash --prefix $(PWT_ASSETS_DIR) \
    -m "Update pwt-assets subtree to $(PWT_ASSETS_VERSION)" \
    $(PWT_ASSETS_REPO) \
    $(PWT_ASSETS_VERSION)

PWT_DIR="pwt"
PWT_VERSION="56e9083f6bc88712f965476ab64bbcb991c88a2a"
PWT_REPO="https://git.proxmox.com/rust/proxmox-yew-wiget-toolkit.git"
PWT_REPO="gitolite3@proxdev.maurer-it.com:yew/proxmox-yew-widget-toolkit"

update-pwt:
	git subtree pull --squash --prefix $(PWT_DIR) \
    -m "Update pwt subtree to $(PWT_VERSION)" \
    $(PWT_REPO) \
    $(PWT_VERSION)
