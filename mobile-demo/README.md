# Mobile Demo

A travel explorer that shows the touch widgets of the toolkit in one app:

- Material app with a page stack, page animations, and hero animations that
  carry a picture from a list into the destination page and on into the
  gallery
- Adaptive scaffold with a bottom navigation bar on phones and a navigation
  rail on wider screens
- Swipeable picture gallery (page view)
- Drawer, end drawer with filters, and modal bottom sheets
- Adaptive dialog: a bottom sheet on phones, a centered dialog on wide screens
- Slidable list items, a floating action button, and a FAB menu
- Long press gestures and snackbars with undo

Settings has a slow motion switch, which makes the hero animations easy to
follow, and lets you choose the page animation style.

The illustrations in `images/` were made for this demo and share its license.

## Running

Run a debug version of this application:

```bash
trunk serve
```

Run a release version of this application:

```bash
trunk serve --release
```
