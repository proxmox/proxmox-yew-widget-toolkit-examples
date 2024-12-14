# This makfile is a helper to update the external repositories
# which are included as git subtree.

all:

PWT_ASSETS_DIR="pwt-assets"
PWT_ASSETS_VERSION="c8ed5c06fce1c193c3560bd861281bdda10e0ce3"
PWT_ASSETS_REPO="https://git.proxmox.com/rust/proxmox-yew-wiget-toolkit-assets.git"
PWT_ASSETS_REPO="gitolite3@proxdev.maurer-it.com:yew/proxmox-yew-widget-toolkit-assets"

update-pwt-assets:
	git subtree pull --squash --prefix $(PWT_ASSETS_DIR) \
	  -m "Update pwt-assets subtree to $(PWT_ASSETS_VERSION)" \
	  $(PWT_ASSETS_REPO) \
	  $(PWT_ASSETS_VERSION)

PWT_DIR="pwt"
PWT_VERSION="349a9438088538c321a313e806b799e13936d267"
PWT_REPO="https://git.proxmox.com/rust/proxmox-yew-wiget-toolkit.git"
PWT_REPO="gitolite3@proxdev.maurer-it.com:yew/proxmox-yew-widget-toolkit"

update-pwt:
	git subtree pull --squash --prefix $(PWT_DIR) \
	  -m "Update pwt subtree to $(PWT_VERSION)" \
	  $(PWT_REPO) \
	  $(PWT_VERSION)
