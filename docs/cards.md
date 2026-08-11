<link rel="stylesheet" type="text/css" href="css/desktop-yew-style.css" />
<link rel="stylesheet" type="text/css" href="css/font-awesome.css" />
<link rel="stylesheet" type="text/css" href="css/doc-style.css" />

<style>
    .doc-example {
        padding: var(--pwt-spacer-2);
        margin-block: var(--pwt-spacer-2);
        border: 1px solid var(--pwt-color-border);
        border-radius: var(--pwt-shape-corner-medium);
        background-color: var(--pwt-color-background-2);
    }
</style>

# Cards

The `Card` widget (`pwt-card`) is a versatile container with multiple layout variants. It supports flat, outlined, elevated, and interactive styles.

## Standard Card

The default card has a subtle shadow and rounded corners.

```html
<div class="pwt-card" style="padding: 1rem;">
    Standard Card Content
</div>
```

<div class="doc-example">
    <div class="pwt-card" style="padding: 1rem;">
        Standard Card Content
    </div>
</div>

## Card Variants

Cards can be styled as "Flat" (no shadow, background color) or "Outlined" (border, no shadow).

### Flat Card

Use the `pwt-card-flat` class for a card with no elevation.

```html
<div class="pwt-card pwt-card-flat" style="padding: 1rem;">
    Flat Card Content
</div>
```

<div class="doc-example">
    <div class="pwt-card pwt-card-flat" style="padding: 1rem;">
        Flat Card Content
    </div>
</div>

### Outlined Card

Use the `pwt-card-outline` class for a card with a border.

```html
<div class="pwt-card pwt-card-outline" style="padding: 1rem;">
    Outlined Card Content
</div>
```

<div class="doc-example">
    <div class="pwt-card pwt-card-outline" style="padding: 1rem;">
        Outlined Card Content
    </div>
</div>

## Elevation

Cards support varying degrees of elevation (shadow depth) using `pwt-elevation0` through `pwt-elevation5`. This only applies to standard cards (not flat or outlined).

```html
<div class="pwt-card pwt-elevation1" style="padding: 1rem; margin-bottom: 1rem;">Elevation 1 (Default)</div>
<div class="pwt-card pwt-elevation3" style="padding: 1rem; margin-bottom: 1rem;">Elevation 3</div>
<div class="pwt-card pwt-elevation5" style="padding: 1rem;">Elevation 5</div>
```

<div class="doc-example">
    <div class="pwt-d-flex pwt-flex-direction-column pwt-gap-2">
        <div class="pwt-card pwt-elevation1" style="padding: 1rem;">Elevation 1 (Default)</div>
        <div class="pwt-card pwt-elevation3" style="padding: 1rem;">Elevation 3</div>
        <div class="pwt-card pwt-elevation5" style="padding: 1rem;">Elevation 5</div>
    </div>
</div>

## Interactive Card

Adding the `pwt-interactive` class makes the card respond to hover and active states, useful for clickable cards.

```html
<div class="pwt-card pwt-interactive" style="padding: 1rem; cursor: pointer;">
    Hover over me!
</div>
```

<div class="doc-example">
    <div class="pwt-card pwt-interactive" style="padding: 1rem; cursor: pointer;">
        Hover over me!
    </div>
</div>

## Color Schemes

Cards support theme-based color schemes using `pwt-scheme-<name>` classes. This determines the background and foreground colors. Similar to regular panels, you can use these classes to apply standard theme colors.

Available schemes:
- `pwt-scheme-primary`
- `pwt-scheme-secondary`
- `pwt-scheme-tertiary`
- `pwt-scheme-success`
- `pwt-scheme-error`
- `pwt-scheme-warning`
- `pwt-scheme-surface`
- `pwt-scheme-neutral`

You can also use the `-container` variants (e.g., `pwt-scheme-primary-container`) for lighter/alternative styling within the same color family.

### Standard Cards with Color Schemes

```html
<div class="pwt-d-flex pwt-gap-2 pwt-flex-wrap">
    <div class="pwt-card pwt-scheme-primary" style="padding: 1rem;">Primary</div>
    <div class="pwt-card pwt-scheme-secondary" style="padding: 1rem;">Secondary</div>
    <div class="pwt-card pwt-scheme-tertiary" style="padding: 1rem;">Tertiary</div>
    <div class="pwt-card pwt-scheme-success" style="padding: 1rem;">Success</div>
    <div class="pwt-card pwt-scheme-error" style="padding: 1rem;">Error</div>
    <div class="pwt-card pwt-scheme-warning" style="padding: 1rem;">Warning</div>
</div>
```

<div class="doc-example">
    <div class="pwt-d-flex pwt-gap-2 pwt-flex-wrap">
        <div class="pwt-card pwt-scheme-primary" style="padding: 1rem;">Primary</div>
        <div class="pwt-card pwt-scheme-secondary" style="padding: 1rem;">Secondary</div>
        <div class="pwt-card pwt-scheme-tertiary" style="padding: 1rem;">Tertiary</div>
        <div class="pwt-card pwt-scheme-success" style="padding: 1rem;">Success</div>
        <div class="pwt-card pwt-scheme-error" style="padding: 1rem;">Error</div>
        <div class="pwt-card pwt-scheme-warning" style="padding: 1rem;">Warning</div>
    </div>
</div>

### Container Variants

```html
<div class="pwt-d-flex pwt-gap-2 pwt-flex-wrap">
    <div class="pwt-card pwt-scheme-primary-container" style="padding: 1rem;">Primary Container</div>
    <div class="pwt-card pwt-scheme-secondary-container" style="padding: 1rem;">Secondary Container</div>
    <div class="pwt-card pwt-scheme-tertiary-container" style="padding: 1rem;">Tertiary Container</div>
    <div class="pwt-card pwt-scheme-success-container" style="padding: 1rem;">Success Container</div>
    <div class="pwt-card pwt-scheme-error-container" style="padding: 1rem;">Error Container</div>
    <div class="pwt-card pwt-scheme-warning-container" style="padding: 1rem;">Warning Container</div>
</div>
```

<div class="doc-example">
    <div class="pwt-d-flex pwt-gap-2 pwt-flex-wrap">
        <div class="pwt-card pwt-scheme-primary-container" style="padding: 1rem;">Primary Container</div>
        <div class="pwt-card pwt-scheme-secondary-container" style="padding: 1rem;">Secondary Container</div>
        <div class="pwt-card pwt-scheme-tertiary-container" style="padding: 1rem;">Tertiary Container</div>
        <div class="pwt-card pwt-scheme-success-container" style="padding: 1rem;">Success Container</div>
        <div class="pwt-card pwt-scheme-error-container" style="padding: 1rem;">Error Container</div>
        <div class="pwt-card pwt-scheme-warning-container" style="padding: 1rem;">Warning Container</div>
    </div>
</div>
