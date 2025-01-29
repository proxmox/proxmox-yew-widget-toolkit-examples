# Proxmox Yew Widget Toolkit CSS Framework

We use this SCSS based framework to implement several themes
for our widget toolkit. The goal was to create three different
themes:

- A theme similar to the layout we currently use for our products.
- A more modern theme for the desktop.
- A theme for mobile devices.

Note: Many ideas are borrowed form the material design
guidelines <https://m3.material.io/>.


## Themes

Each theme can define its own:

- Font sizes
- Colors for dark and light mode, default accent color
- Spacing density and corner shapes

And it is of cause possible to fine tune specific layouts for each theme.


### The Color System

We use names to identify colors.

The following names are used for neutral colors. Neutral colors mostly
use black, white and grey colors. Those colors are useful for backgrounds
and low emphasis areas.

- `neutral`: black on white background
- `surface`: black on grey background
- `inverse-surface`: like `surface`, but foreground/background colors inverted
- `dark-surface`: like `surface` in dark mode, but stays dark, even in light mode
- `neutral-alt`: like `surface`, with mixed in primary color tint.

Accent colors are the theme main colors. We currently define:

- `primary`: used for most important parts (highest emphasis)
- `secondary`: less prominent than `primary`
- `tertiary`: an alternative to `secondary`

We also define colors for action status:

- `success`: used to indicate success
- `error`: used to highlight errors
- `warning`: used to indicate warnings


#### Tonal Color Palettes and Color Schemes

Each theme defines seed colors, which ares used to compute tonal
[HCT]( https://material.io/blog/science-of-color-design) color palettes.

We then choose two colors from each palette, where the `-on` colors should
have a minimum distance of 30% in the color palette (in order to guaranteed
good contrast). CSS variables are used to store those color pairs:

- `--pwt-color-<name>`
- `--pwt-color-on-<name>`

We also call this a color scheme. A second pair, the `-container`
variant, uses lighter colors.

- `--pwt-color-<name>-container`
- `--pwt-color-on-<name>-container`

It is recommended to always use the `-on` variant as the text color,
the other color as background.

The following helper classes can be used to set the CSS `color`
property directly:

- `pwt-color-<name>`:  sets `color` to `var(--pwt-color-<name>)`
- `pwt-color-on-<name>`:  sets `color` to `var(--pwt-color-on-<name>)`
- `pwt-bg-color-<name>`:  sets `background-color` to `var(--pwt-color-<name>)`
- `pwt-bg-color-on<name>`:  sets `background-color` to `var(--pwt-color-on-<name>)`

Another set of classes is used to set fill and stroke colors for SVG elements:

- `pwt-fill-<name>`:  sets `fill` to `var(--pwt-color-<name>)`
- `pwt-fill-on-<name>`:  sets `fill` to `var(--pwt-color-on-<name>)`
- `pwt-stroke-<name>`:  sets `stroke` to `var(--pwt-color-<name>)`
- `pwt-stroke-on-<name>`:  sets `stroke` to `var(--pwt-color-on-<name>)`


#### Light and Dark Mode

Each theme provides a different color set for light and dark mode. Mode change
is done by simply changing the CSS color variables. This can be done by setting
class `pwt-dark-mode` on the DOM root element.


#### Widget Color Schema

Some widgets have an associated color scheme, i.e. they store the color
scheme using two CSS variables:

- `--pwt-color`
- `--pwt-color-background`

And then use those variables to set the color, background or other
aspects of the widget. Thus, changing the value of those variables
changes the colors of the widget. The `pwt-scheme-<name>` and
`pwt-scheme-<name>-container` helper classes can be used to set
those variables.

A similar scheme is used for widgets with an associated accent color, using
the following CSS variables:

- `--pwt-accent-color`
- `--pwt-accent-color-background`

The `pwt-accent-<name>` and `pwt-accent-<name>-container` helper classes
can be used to set them.

### Fonts

We currently use the [Roboto Flex](https://fonts.google.com/specimen/Roboto+Flex) font
for all themes, with slightly adopted font size, weight and line spacing.

We define the following font usage categories:

- `display`:
- `headline`:
- `title`:
- `label`:
- `body`:

Each category provides three font scales, named `large`, `medium` and `small`.

The following classes are available to set fonts size and/or line height:

- `pwt-font-<category>-<scale>`: set `font-size` and `line-height`
- `pwt-font-size-<category>-<scale>`: only set `font-size`
- `pwt-line-height-<category>-<scale>`: only set `line-height`

Example class names: `pwt-font-label-large`, `pwt-line-height-body-medium`


### Spacing

To achieve consistent spacing, we define `var(--pwt-spacer-base-width)`.
Widget properties such as padding are then defined using a multiple of the
spacer base width.

Each theme defines a default spacer base unit, but we also allow to
overwrite it manually using the following CSS classes:

- `pwt-density-high`: very small spacer (`3px`), high density
- `pwt-density-medium`: normal (`5px`), usefule for desktop apps
- `pwt-density-touch`: larger spacer (`10px`), useful for touch devices
