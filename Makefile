# This makfile provides targets to update the repositories of our dependencies
# which are included as git subtree.

all:

PWT_ASSETS_DIR="pwt-assets"
PWT_ASSETS_VERSION="6aa8803362c6eb10d1df2b84f625c817616ca1ee"
PWT_ASSETS_REPO="git://git.proxmox.com/git/ui/proxmox-yew-widget-toolkit-assets.git"

update-pwt-assets:
	git subtree pull --squash --prefix $(PWT_ASSETS_DIR) \
	  -m "Update pwt-assets subtree to $(PWT_ASSETS_VERSION)" \
	  $(PWT_ASSETS_REPO) \
	  $(PWT_ASSETS_VERSION)

PWT_DIR="pwt"
PWT_VERSION="df3ba55c46add2ccb8a575a9c3fa1ce8651f98a5"
PWT_REPO="git://git.proxmox.com/git/ui/proxmox-yew-widget-toolkit.git"

update-pwt:
	git subtree pull --squash --prefix $(PWT_DIR) \
	  -m "Update pwt subtree to $(PWT_VERSION)" \
	  $(PWT_REPO) \
	  $(PWT_VERSION)
