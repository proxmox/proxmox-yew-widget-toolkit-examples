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

### The "Desktop" Theme

This theme is meant to be used on standard desktops with large
screen sizes. The default spacer width is `5px`, and it uses a
relatively small corner radius.

The primary color is used as default accent color.

#### Color setup

Base colors are from <https://en.wikipedia.org/wiki/Adwaita_(design_language)>.


### Fine Tuning

`pwt-panel-header`, `pwt-menubar`
: use `neutral` color scheme (instead of `surface`)

`pwt-panel`
: use box shadows instead of normal borders

`pwt-content-spacer`
: overwrite content spacer layout

  - use elevated box with box-shadow
  - use bigger spacing

`h1`
: use `headline-large` font

`h2`
: use `headline-medium` font

`h3`
: use `title-large` font

`h4`
: use `headline-small` font

`h5`
: use `title-medium` font

`h6`
: use `title-small` font


### The "Material" Theme

This theme is intended for use on mobile/touch devices and uses a very
large `10px` spacer. It is broadly similar to material design as
described in <https://m3.material.io/>.


#### Color setup

This theme uses the [material design baseline colors](https://m3.material.io/styles/color/static/baseline>).


### Fine Tuning

`pwt-panel-header`, `pwt-menubar`
: use `neutral` color scheme (instead of `surface`)

`pwt-nav-menu`
: add padding

`pwt-tab-material`
: add padding

`pwt-content-spacer`
: overwrite content spacer layout

- use elevated box with box-shadow


### The "Crisp" Theme

This theme is similar to what we currently use for our products.
The `3px` spacing is very dense, allowing to pack as much info
as possible onto a single page.

The primary color is used as default accent color.

#### Color setup

Use proxmox company colors, as described in the [Proxmox Brand Guidelines](https://www.proxmox.com/images/proxmox/Proxmox-Corporate-Brandguideline-2021.pdf).


### Fine Tuning

`pwt-content-spacer`
: use neutral color scheme instead of surface.


## Common CSS Utility classes

We provide CSS utility classes for commonly used styles.
The goal is to style widgets primarily through the application
of those CSS classes. Setting the CSS “style” directly is still possible
and is recommended if no appropriate utility class is available.

### Padding

Padding classes comes at 5 different sizes, using a multiple
of the spacer base width (`size` in 0, 1, 2, 3, 4):

- `pwt-p-<size>`: set `padding` to `size*spacer_width`
- `pwt-ps-<size>`: set `padding-inline-start` to `size*spacer_width`
- `pwt-pe-<size>`: set `padding-inline-end` to `size*spacer_width`
- `pwt-pt-<size>`: set `padding-top` to `size*spacer_width`
- `pwt-pb-<size>`: set `padding-bottom` to `size*spacer_width`
- `pwt-px-<size>`: set `padding-inline` to `size*spacer_width`
- `pwt-py-<size>`: set `padding-block` to `size*spacer_width`

### Margin

Margin classes comes at 5 different sizes, using a multiple
of the spacer base width (`size` in 0, 1, 2, 3, 4):

- `pwt-m-<size>`: set `margin` to `size*spacer_width`
- `pwt-ms-<size>`: set `margin-inline-start` to `size*spacer_width`
- `pwt-me-<size>`: set `margin-inline-end` to `size*spacer_width`
- `pwt-mt-<size>`: set `margin-top` to `size*spacer_width`
- `pwt-mb-<size>`: set `margin-bottom` to `size*spacer_width`
- `pwt-mx-<size>`: set `margin-inline` to `size*spacer_width`
- `pwt-my-<size>`: set `margin-block` to `size*spacer_width`

### Flex and Grid Gap

Gap classes comes at 5 different sizes, using a multiple
of the spacer base width (`size` in 0, 1, 2, 3, 4):

- `pwt-gap-<size>`: set `gap` to `size*spacer_width`
- `pwt-column-gap-<size>`: set `column-gap` to `size*spacer_width`
- `pwt-row-gap-<size>`: set `row-gap` to `size*spacer_width`

### Text related classes
* `.pwt-white-space-<value>`: set `white-space` property to one of
  - `normal`, `nowrap`, `pre`, `pre-wrap`, `pre-line` or `break-spaces`.
* `pwt-text-truncate`: truncate text if too long using ellipsis.
* `.pwt-user-select-<value>`: set `user-select` property to one of
  - `auto`, `none`, `text`, `all`
* `.pwt-text-align-<value>`: set `text-align` property to one of
  - `start`, `end`, `left`, `right`, `center`, `justify`, `justify-all`

### Width and Height

For the following values: `25`, `50`, `75`, `100`, `auto`,
`min-content`, `max-content`, `fit-content`

- `pwt-h-<value>`: set `height` to value.
- `pwt-w-<value>`: set `width` to value.
- `pwt-size-<value>`: set `width` and `height` to value.
- `pwt-fit`: fit container into partent container using 100% width and height.
  This can be used for non-flexbox layouts (also set `overflow: auto`).
- `pwt-viewport`: set dimensions and position to use the whole
  viewport (also set `overflow: auto`).

Additionally, all those classes sets `box-sizing` to `border-box`.

### Layout and Flexbox

- `pwt-d-<value>`: set `display` property to one of
  - `inline`, `inline-block`, `block`, `grid`, `table`, `table-row`
    `table-cell`, `flex`, `inline-flex`, `contents`, `none`
- `pwt-flex-fit`: use available space inside flexbox (`flex: 1 1 auto;overflow: auto;`)
- `pwt-flex-fill-first-child`: Apply `flex: 1 1 auto;` to the first child of a container
- `pwt-flex-fill` or `pwt-flex-auto`: set `flex: 1 1 auto;`
- `pwt-flex-initial`: set `flex: 0 1 auto;`
- `pwt-flex-none`: set `flex: 0 0 auto;`
- `pwt-flex-directzion-<value>`: set `flex-direction` property to one of
  - `row`, `column`, `row-reverse`, `column-reverse`
- `pwt-flex-wrap-<value>`: set `flex-wrap` property to one of
  - `wrap`, `nowrap`, `wrap-reverse`
- `pwt-justify-content-<value>`: set `justify-content` property to one of
  - `start`, `end`, `flex-start`, `flex-end`, `center`, `left`, `right`,
    `normal`, `space-between`, `space-around`, `space-evenly`, `stretch`
- `pwt-align-items-<value>`: set `align-items` property to one of
  -  `start`, `end`, `flex-start`, `flex-end`, `normal`, `center`, `baseline`, `stretch`
- `pwt-align-self-<value>`: set `align-self` property to one of
  - `auto`, `normal`, `flex-start`, `flex-end`, `center`, `baseline`, `stretch`
- `pwt-align-content-<value>`: set `align-content` property to one of
  - `start`, `center`, `end`, `space-between`, `space-around`, `stretch`
### Overflow (Scrollbars)

For the following values: `auto`, `hidden`, `visible`, `scroll`

- `pwt-overflow-<value>`: set `overflow` property to value
- `pwt-overflow-x-<value>`: set `overflow-x` property to value
- `pwt-overflow-y-<value>`: set `overflow-y` property to value
### Position

For the following values: `absolute`, `fixed`, `relative`, `sticky`, `static`;
- `pwt-position-<value>`: set `position` property to value
### Borders

- `pwt-border`: add border on all sides
- `pwt-no-border`: remove all borders
- `pwt-border-top`: add border on top
- `pwt-no-border-top`: remove top border
- `pwt-border-left`: add border on the left side
- `pwt-no-border-left`: remove left border
- `pwt-border-start`: add border on start side
- `pwt-no-border-start`: remove start border
- `pwt-border-right`: add border on the right side
- `pwt-no-border-right`: remove right border
- `pwt-border-end`: add border on end side
- `pwt-no-border-end`: remove end border
- `pwt-border-bottom`: add border on bottom
- `pwt-no-border-bottom`: remove bottom border
### Shapes

- `pwt-shape-circle`: make it a circle (fully rounded corners)

For the following values: `none`, `extra-small`, `small`, `medium`, `large`,
`extra-large`, `full`

- `pwt-shape-<value>`: set `border-radius` to the value defined for the theme
- `pwt-shape-top-<value>`: set `border-top-left-radius` and `border-top-right-radius`
- `pwt-shape-bottom-<value>`: set `border-bottom-left-radius` and `border-bottom-right-radius`
- `pwt-shape-start-<value>`: set `border-start-start-radius` and `border-end-start-radius`
- `pwt-shape-end-<value>`: set `border-start-end-radius` and `border-end-end-radius`
- `pwt-shape-top-start-<value>`: set `border-start-start-radius`
- `pwt-shape-top-end-<value>`: set `border-start-end-radius`
- `pwt-shape-bottom-start-<value>`: set `border-end-start-radius`
- `pwt-shape-bottom-end-<value>`: set `border-end-end-radius`
### Box Shadows

For the following elevation levels: 0, 1, 2, 3, 4, 5
- `pwt-shadow-<level>`: add box shadow as defined in the theme
### Rendering

- `pwt-opacity-<value>`: set `opacity` property to percentage for values in
  - 0, 25, 50, 75, 100

## Panel Widget

Class `pwt-panel`

This is the top level class for panels, using a flex column layout.
It set the color scheme to `neutral`, and use the color scheme variables
for `color` and `background-color`. The accent color scheme is set to 'primary',
and is used as color for the header text.

Note: color schemes can be adopted by the user,
i.e. by using `pwt-panel.pwt-color-scheme-primary`

Class `pwt-panel-header`

Panel headers use the `surface` color scheme, and add
a `--pwt-spacer-2` padding for the header text.


Class `pwt-panel-header-text`

Header text uses font `headline-small` and the accent color from the panel.


