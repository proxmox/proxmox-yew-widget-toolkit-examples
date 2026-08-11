# This makfile provides targets to update the repositories of our dependencies
# which are included as git subtree.

all:

PWT_ASSETS_DIR="pwt-assets"
PWT_ASSETS_VERSION="5b3a866ff9e2a0945bf5d84176d0d23ae83ef782"
PWT_ASSETS_REPO="git://git.proxmox.com/git/ui/proxmox-yew-widget-toolkit-assets.git"

update-pwt-assets:
	git subtree pull --squash --prefix $(PWT_ASSETS_DIR) \
	  -m "Update pwt-assets subtree to $(PWT_ASSETS_VERSION)" \
	  $(PWT_ASSETS_REPO) \
	  $(PWT_ASSETS_VERSION)

PWT_DIR="pwt"
PWT_VERSION="15a3ad6a10b2f013b40d9badf775e29b8f5946c7"
PWT_REPO="git://git.proxmox.com/git/ui/proxmox-yew-widget-toolkit.git"

update-pwt:
	git subtree pull --squash --prefix $(PWT_DIR) \
	  -m "Update pwt subtree to $(PWT_VERSION)" \
	  $(PWT_REPO) \
	  $(PWT_VERSION)
