# This makfile is a helper to update the external repositories
# which are included as git subtree.

all:

PWT_ASSETS_DIR="pwt-assets"
PWT_ASSETS_VERSION="096f690f5e184774426f78926c312d308164620b"
PWT_ASSETS_REPO="https://git.proxmox.com/rust/proxmox-yew-wiget-toolkit-assets.git"
PWT_ASSETS_REPO="gitolite3@proxdev.maurer-it.com:yew/proxmox-yew-widget-toolkit-assets"

update-pwt-assets:
	git subtree pull --squash --prefix $(PWT_ASSETS_DIR) \
    -m "Update pwt-assets subtree to $(PWT_ASSETS_VERSION)" \
    $(PWT_ASSETS_REPO) \
    $(PWT_ASSETS_VERSION)

PWT_DIR="pwt"
PWT_VERSION="f99c53cc8b26c1dbbbee2fb442992d3cc12f6710"
PWT_REPO="https://git.proxmox.com/rust/proxmox-yew-wiget-toolkit.git"
PWT_REPO="gitolite3@proxdev.maurer-it.com:yew/proxmox-yew-widget-toolkit"

update-pwt:
	git subtree pull --squash --prefix $(PWT_DIR) \
    -m "Update pwt subtree to $(PWT_VERSION)" \
    $(PWT_REPO) \
    $(PWT_VERSION)
