# This makfile provides targets to update the repositories of our dependencies
# which are included as git subtree.

all:

PWT_ASSETS_DIR="pwt-assets"
PWT_ASSETS_VERSION="33e4e7179d79018d889943edb5c02d6435242fb6"
PWT_ASSETS_REPO="git://git.proxmox.com/git/ui/proxmox-yew-widget-toolkit-assets.git"

update-pwt-assets:
	git subtree pull --squash --prefix $(PWT_ASSETS_DIR) \
	  -m "Update pwt-assets subtree to $(PWT_ASSETS_VERSION)" \
	  $(PWT_ASSETS_REPO) \
	  $(PWT_ASSETS_VERSION)

PWT_DIR="pwt"
PWT_VERSION="3277b6a174c6a17147d29e4a40d9b06c74bedfe6"
PWT_REPO="git://git.proxmox.com/git/ui/proxmox-yew-widget-toolkit.git"

update-pwt:
	git subtree pull --squash --prefix $(PWT_DIR) \
	  -m "Update pwt subtree to $(PWT_VERSION)" \
	  $(PWT_REPO) \
	  $(PWT_VERSION)
