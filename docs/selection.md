<link rel="stylesheet" type="text/css" href="css/desktop-yew-style.css" />
<link rel="stylesheet" type="text/css" href="css/font-awesome.css" />
<link rel="stylesheet" type="text/css" href="css/doc-style.css" />

<style>
    .doc-example {
        padding: var(--pwt-spacer-2);
        margin-block: var(--pwt-spacer-2);
        border: 1px solid var(--pwt-color-border);
        border-radius: var(--pwt-shape-corner-medium);
    }
</style>

# Selection Controls

This page demonstrates valid selection widgets.

## Checkbox

The `Checkbox` widget.

### Standard Checkbox

```html
<div class="pwt-checkbox-state">
    <!-- Unchecked -->
    <div class="pwt-checkbox" tabindex="0" role="checkbox" aria-checked="false">
        <span class="pwt-checkbox-icon"><i class="fa fa-check"></i></span>
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-checkbox-state">
        <div class="pwt-checkbox" tabindex="0" role="checkbox" aria-checked="false">
            <span class="pwt-checkbox-icon"><i class="fa fa-check"></i></span>
        </div>
    </div>
</div>

### Checked

Add the `checked` class to the `pwt-checkbox` container.

```html
<div class="pwt-checkbox-state">
    <div class="pwt-checkbox checked" tabindex="0" role="checkbox" aria-checked="true">
        <span class="pwt-checkbox-icon"><i class="fa fa-check"></i></span>
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-checkbox-state">
        <div class="pwt-checkbox checked" tabindex="0" role="checkbox" aria-checked="true">
            <span class="pwt-checkbox-icon"><i class="fa fa-check"></i></span>
        </div>
    </div>
</div>

### Disabled

```html
<div class="pwt-checkbox-state">
    <div class="pwt-checkbox disabled">
        <span class="pwt-checkbox-icon"><i class="fa fa-check"></i></span>
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-checkbox-state">
        <div class="pwt-checkbox disabled">
            <span class="pwt-checkbox-icon"><i class="fa fa-check"></i></span>
        </div>
    </div>
</div>

### With Label

Checkboxes are usually wrapped in a `Row` with a label.

<div class="doc-example">
    <div class="pwt-d-flex pwt-align-items-center">
        <div class="pwt-checkbox-state">
            <div class="pwt-checkbox checked">
                <span class="pwt-checkbox-icon"><i class="fa fa-check"></i></span>
            </div>
        </div>
        <div class="pwt-field-label pwt-ps-2">I agree to the terms</div>
    </div>
</div>

## Toggle Switch

A `Checkbox` with `switch=true`.

### Off (Unchecked)

```html
<div class="pwt-checkbox-state">
    <div class="pwt-switch" tabindex="0" role="checkbox" aria-checked="false">
        <span class="pwt-switch-slider"><i class="fa fa-check"></i></span>
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-checkbox-state">
        <div class="pwt-switch" tabindex="0" role="checkbox" aria-checked="false">
            <span class="pwt-switch-slider"><i class="fa fa-check"></i></span>
        </div>
    </div>
</div>

### On (Checked)

```html
<div class="pwt-checkbox-state">
    <div class="pwt-switch checked" tabindex="0" role="checkbox" aria-checked="true">
        <span class="pwt-switch-slider"><i class="fa fa-check"></i></span>
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-checkbox-state">
        <div class="pwt-switch checked" tabindex="0" role="checkbox" aria-checked="true">
            <span class="pwt-switch-slider"><i class="fa fa-check"></i></span>
        </div>
    </div>
</div>

## Radio Button

The `RadioButton` widget uses the `pwt-radio-button` class directly.

### Unselected

```html
<div class="pwt-checkbox-state">
    <div class="pwt-radio-button" tabindex="0" role="radio" aria-checked="false">
        <span class="pwt-checkbox-icon"><i class="fa fa-check"></i></span>
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-checkbox-state">
        <div class="pwt-radio-button" tabindex="0" role="radio" aria-checked="false">
            <span class="pwt-checkbox-icon"><i class="fa fa-check"></i></span>
        </div>
    </div>
</div>

### Selected

```html
<div class="pwt-checkbox-state">
    <div class="pwt-radio-button checked" tabindex="0" role="radio" aria-checked="true">
        <span class="pwt-checkbox-icon"><i class="fa fa-check"></i></span>
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-checkbox-state">
        <div class="pwt-radio-button checked" tabindex="0" role="radio" aria-checked="true">
            <span class="pwt-checkbox-icon"><i class="fa fa-check"></i></span>
        </div>
    </div>
</div>

## Combobox (Selector)

The `Combobox` widget renders as an input with a dropdown trigger.

### Default (Collapsed)

```html
<div class="pwt-input pwt-input-type-text pwt-w-100" style="max-width: 300px;">
    <input class="pwt-flex-fill" type="text" value="Option 1" readonly>
    <i class="fa pwt-dropdown-icon pwt-pointer fa-angle-down" tabindex="-1"></i>
</div>
```

<div class="doc-example">
    <div class="pwt-input pwt-input-type-text pwt-w-100" style="max-width: 300px;">
        <input class="pwt-flex-fill" type="text" value="Option 1" readonly>
        <i class="fa pwt-dropdown-icon pwt-pointer fa-angle-down" tabindex="-1"></i>
    </div>
</div>

### Picker Open

When open, the `picker-open` class is added to the container, and the icon changes to `fa-angle-up`.

```html
<div class="pwt-input pwt-input-type-text picker-open pwt-w-100" style="max-width: 300px;">
    <input class="pwt-flex-fill" type="text" value="Option 1" readonly>
    <i class="fa pwt-dropdown-icon pwt-pointer fa-angle-up" tabindex="-1"></i>
</div>
```

<div class="doc-example">
    <div class="pwt-input pwt-input-type-text picker-open pwt-w-100" style="max-width: 300px;">
        <input class="pwt-flex-fill" type="text" value="Option 1" readonly>
        <i class="fa pwt-dropdown-icon pwt-pointer fa-angle-up" tabindex="-1"></i>
    </div>
</div>

### Invalid

```html
<div class="pwt-input pwt-input-type-text pwt-w-100 is-invalid" style="max-width: 300px;">
    <input class="pwt-flex-fill" type="text" value="Invalid Choice" readonly>
    <i class="fa pwt-dropdown-icon pwt-pointer fa-angle-down" tabindex="-1"></i>
</div>
```

<div class="doc-example">
    <div class="pwt-input pwt-input-type-text pwt-w-100 is-invalid" style="max-width: 300px;">
        <input class="pwt-flex-fill" type="text" value="Invalid Choice" readonly>
        <i class="fa pwt-dropdown-icon pwt-pointer fa-angle-down" tabindex="-1"></i>
    </div>
</div>
