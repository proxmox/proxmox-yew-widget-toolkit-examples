# This makfile is a helper to update the external repositories
# which are included as git subtree.

all:

PWT_ASSETS_DIR="pwt-assets"
PWT_ASSETS_VERSION="bed60c960d7874b146a161e910f069e5a07d7cf2"
PWT_ASSETS_REPO="https://git.proxmox.com/rust/proxmox-yew-wiget-toolkit-assets.git"
PWT_ASSETS_REPO="gitolite3@proxdev.maurer-it.com:yew/proxmox-yew-widget-toolkit-assets"

update-pwt-assets:
	git subtree pull --squash --prefix $(PWT_ASSETS_DIR) \
    -m "Update pwt-assets subtree to $(PWT_ASSETS_VERSION)" \
    $(PWT_ASSETS_REPO) \
    $(PWT_ASSETS_VERSION)

PWT_DIR="pwt"
PWT_VERSION="323ecc3b7a95e74b3c94abb636d4592250bec617"
PWT_REPO="https://git.proxmox.com/rust/proxmox-yew-wiget-toolkit.git"
PWT_REPO="gitolite3@proxdev.maurer-it.com:yew/proxmox-yew-widget-toolkit"

update-pwt:
	git subtree pull --squash --prefix $(PWT_DIR) \
    -m "Update pwt subtree to $(PWT_VERSION)" \
    $(PWT_REPO) \
    $(PWT_VERSION)
