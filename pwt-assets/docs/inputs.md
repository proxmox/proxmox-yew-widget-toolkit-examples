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

## Text Input

The `Field` widget renders a standard text input. It is wrapped in a container with the class `pwt-input`.

### Default

```html
<div class="pwt-input pwt-input-type-text pwt-w-100">
    <input class="pwt-flex-fill" type="text" value="Default Text">
</div>
```

<div class="doc-example">
    <div class="pwt-input pwt-input-type-text pwt-w-100" style="max-width: 300px;">
        <input class="pwt-flex-fill" type="text" value="Default Text">
    </div>
</div>

### Placeholder

```html
<div class="pwt-input pwt-input-type-text pwt-w-100">
    <input class="pwt-flex-fill" type="text" placeholder="Enter text here...">
</div>
```

<div class="doc-example">
    <div class="pwt-input pwt-input-type-text pwt-w-100" style="max-width: 300px;">
        <input class="pwt-flex-fill" type="text" placeholder="Enter text here...">
    </div>
</div>

### Disabled

Add the `disabled` class to the container and the disabled attribute to the input.

```html
<div class="pwt-input pwt-input-type-text pwt-w-100 disabled">
    <input class="pwt-flex-fill" type="text" value="Disabled Input" disabled>
</div>
```

<div class="doc-example">
    <div class="pwt-input pwt-input-type-text pwt-w-100 disabled" style="max-width: 300px;">
        <input class="pwt-flex-fill" type="text" value="Disabled Input" disabled>
    </div>
</div>

### Invalid State

Add the `is-invalid` class to the container.

```html
<div class="pwt-input pwt-input-type-text pwt-w-100 is-invalid">
    <input class="pwt-flex-fill" type="text" value="Invalid Value">
</div>
```

<div class="doc-example">
    <div class="pwt-input pwt-input-type-text pwt-w-100 is-invalid" style="max-width: 300px;">
        <input class="pwt-flex-fill" type="text" value="Invalid Value">
    </div>
</div>

### With Icons (Triggers)

Icons can be added using Font Awesome classes.

```html
<div class="pwt-input pwt-input-type-text pwt-w-100">
    <div style="display: contents">
        <div class="pwt-flex-fill-first-child pwt-d-flex pwt-align-self-center">
            <i class="fa fa-user"></i>
        </div>
    </div>
    <input class="pwt-flex-fill" type="text" value="User Name">
</div>
```

<div class="doc-example">
    <div class="pwt-input pwt-input-type-text pwt-w-100" style="max-width: 300px;">
        <div style="display: contents">
            <div class="pwt-flex-fill-first-child pwt-d-flex pwt-align-self-center">
                <i class="fa fa-user"></i>
            </div>
        </div>
        <input class="pwt-flex-fill" type="text" value="User Name">
    </div>
</div>

## Number Input

The `Number` widget includes up/down spinners.

```html
<div class="pwt-input pwt-input-type-number pwt-w-100">
    <input class="pwt-flex-fill" type="text" role="spinbutton" value="42">
    <div class="spinner">
        <i class="fa fa-angle-up"></i>
        <i class="fa fa-angle-down"></i>
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-input pwt-input-type-number pwt-w-100" style="max-width: 300px;">
        <input class="pwt-flex-fill" type="text" role="spinbutton" value="42">
        <div class="spinner">
            <i class="fa fa-angle-up"></i>
            <i class="fa fa-angle-down"></i>
        </div>
    </div>
</div>

### Invalid Number

```html
<div class="pwt-input pwt-input-type-number pwt-w-100 is-invalid">
    <input class="pwt-flex-fill" type="text" role="spinbutton" value="-1">
    <div class="spinner">
        <i class="fa fa-angle-up"></i>
        <i class="fa fa-angle-down"></i>
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-input pwt-input-type-number pwt-w-100 is-invalid" style="max-width: 300px;">
        <input class="pwt-flex-fill" type="text" role="spinbutton" value="-1">
        <div class="spinner">
            <i class="fa fa-angle-up"></i>
            <i class="fa fa-angle-down"></i>
        </div>
    </div>
</div>

## Text Area

The `TextArea` widget renders a `textarea` element with `pwt-textarea` class.

### Default

```html
<textarea class="pwt-textarea" rows="4">Multi-line text area...</textarea>
```

<div class="doc-example">
    <textarea class="pwt-textarea" rows="4" style="max-width: 300px;">Multi-line text area...</textarea>
</div>

### Invalid

```html
<textarea class="pwt-textarea is-invalid" rows="4">Invalid content</textarea>
```

<div class="doc-example">
    <textarea class="pwt-textarea is-invalid" rows="4" style="max-width: 300px;">Invalid content</textarea>
</div>
