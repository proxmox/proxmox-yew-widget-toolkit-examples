# This makfile provides targets to update the repositories of our dependencies
# which are included as git subtree.

all:

PWT_ASSETS_DIR="pwt-assets"
PWT_ASSETS_VERSION="2335e3430f10f57f79d8419624c1e20a3d90bcb6"
PWT_ASSETS_REPO="git://git.proxmox.com/git/ui/proxmox-yew-widget-toolkit-assets.git"

update-pwt-assets:
	git subtree pull --squash --prefix $(PWT_ASSETS_DIR) \
	  -m "Update pwt-assets subtree to $(PWT_ASSETS_VERSION)" \
	  $(PWT_ASSETS_REPO) \
	  $(PWT_ASSETS_VERSION)

PWT_DIR="pwt"
PWT_VERSION="349a9438088538c321a313e806b799e13936d267"
PWT_REPO="git://git.proxmox.com/git/ui/proxmox-yew-widget-toolkit.git"

update-pwt:
	git subtree pull --squash --prefix $(PWT_DIR) \
	  -m "Update pwt subtree to $(PWT_VERSION)" \
	  $(PWT_REPO) \
	  $(PWT_VERSION)
