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

# Panels & Containers

This page demonstrates container widgets used for grouping content.

## Panel

The `Panel` widget (`pwt-panel`) is a container that offers a header section and a body.

### Standard Panel

```html
<div class="pwt-panel">
    <div class="pwt-panel-header pwt-align-items-center pwt-gap-1" role="group" aria-label="panel header">
        <div role="none" class="pwt-panel-header-text">Panel Title</div>
    </div>
    <div style="padding: 1rem;">
        Panel Content
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-panel">
        <div class="pwt-panel-header pwt-align-items-center pwt-gap-1" role="group" aria-label="panel header">
            <div role="none" class="pwt-panel-header-text">Panel Title</div>
        </div>
        <div style="padding: 1rem;">
            Panel Content
        </div>
    </div>
</div>

### Panel with Tools

Tools can be added to the header.

```html
<div class="pwt-panel">
    <div class="pwt-panel-header pwt-d-flex pwt-align-items-center pwt-gap-1" role="group" aria-label="panel header">
        <div role="none" class="pwt-panel-header-text">Panel with Tools</div>
        <div style="flex: 1;"></div>
        <button class="pwt-button pwt-button-text"><i class="fa fa-refresh"></i></button>
        <button class="pwt-button pwt-button-text"><i class="fa fa-cog"></i></button>
    </div>
    <div style="padding: 1rem;">
        Content with tools in header
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-panel">
        <div class="pwt-panel-header pwt-d-flex pwt-align-items-center pwt-gap-1" role="group" aria-label="panel header">
            <div role="none" class="pwt-panel-header-text">Panel with Tools</div>
            <div style="flex: 1;"></div>
            <button class="pwt-button pwt-button-text"><i class="fa fa-refresh"></i></button>
            <button class="pwt-button pwt-button-text"><i class="fa fa-cog"></i></button>
        </div>
        <div style="padding: 1rem;">
            Content with tools in header
        </div>
    </div>
</div>


## Input Panel (Form Grid)

The `InputPanel` uses a CSS grid layout to align labels and fields. The default is a 2-column layout (Label + Field).

### Standard Layout

Simulates `InputPanel` with `pwt-form-grid-col2`.

```html
<div class="pwt-form-grid-col2 pwt-d-grid pwt-gap-2" style="grid-template-columns: minmax(130px, 0.65fr) minmax(200px, 1fr); padding: 10px;">
    <!-- Row 1 -->
    <div class="pwt-field-label pwt-align-self-center pwt-grid-column-1">First Name</div>
    <div class="pwt-align-self-center pwt-grid-column-2">
        <div class="pwt-input pwt-input-type-text pwt-w-100"><input class="pwt-flex-fill" value="John"></div>
    </div>
    
    <!-- Row 2 -->
    <div class="pwt-field-label pwt-align-self-center pwt-grid-column-1">Last Name</div>
    <div class="pwt-align-self-center pwt-grid-column-2">
        <div class="pwt-input pwt-input-type-text pwt-w-100"><input class="pwt-flex-fill" value="Doe"></div>
    </div>
</div>
```

<div class="doc-example">
    <div class="pwt-form-grid-col2 pwt-d-grid pwt-gap-2" style="grid-template-columns: minmax(130px, 0.65fr) minmax(200px, 1fr); padding: 10px;">
        <div class="pwt-field-label pwt-align-self-center pwt-grid-column-1">First Name</div>
        <div class="pwt-align-self-center pwt-grid-column-2">
            <div class="pwt-input pwt-input-type-text pwt-w-100"><input class="pwt-flex-fill" type="text" value="John"></div>
        </div>
        <div class="pwt-field-label pwt-align-self-center pwt-grid-column-1">Last Name</div>
        <div class="pwt-align-self-center pwt-grid-column-2">
            <div class="pwt-input pwt-input-type-text pwt-w-100"><input class="pwt-flex-fill" type="text" value="Doe"></div>
        </div>
    </div>
</div>
