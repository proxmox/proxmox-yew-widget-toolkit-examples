<link rel="stylesheet" type="text/css" href="css/material-yew-style.css" />
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

# Floating Action Button (FAB)

FABs are used for a primary action on a screen. Because they are primarily designed for mobile/touch devices, they work best with themes designed for mobile devices, i.e. our material design theme (`css/material-yew-style.css`).

## Standard FAB

The standard FAB is used for most cases.

<div class="doc-example">
    <button class="pwt-button pwt-fab">
        <i class="pwt-fab-icon fa fa-plus"></i>
    </button>
    <button class="pwt-button pwt-fab pwt-scheme-secondary">
        <i class="pwt-fab-icon fa fa-edit"></i>
    </button>
    <button class="pwt-button pwt-fab pwt-scheme-tertiary">
        <i class="pwt-fab-icon fa fa-check"></i>
    </button>
</div>

```html
<button class="pwt-button pwt-fab">
    <i class="pwt-fab-icon fa fa-plus"></i>
</button>
```

## Small FAB

A smaller variant.

<div class="doc-example">
    <button class="pwt-button pwt-fab pwt-fab-small">
        <i class="pwt-fab-icon fa fa-plus"></i>
    </button>
</div>

```html
<button class="pwt-button pwt-fab pwt-fab-small">
    <i class="pwt-fab-icon fa fa-plus"></i>
</button>
```

## Large FAB

A larger variant.

<div class="doc-example">
    <button class="pwt-button pwt-fab pwt-fab-large">
        <i class="pwt-fab-icon fa fa-plus"></i>
    </button>
</div>

```html
<button class="pwt-button pwt-fab pwt-fab-large">
    <i class="pwt-fab-icon fa fa-plus"></i>
</button>
```

## Extended FAB

FABs can contain text.

<div class="doc-example">
    <button class="pwt-button pwt-fab pwt-fab-extended">
        <i class="pwt-fab-icon fa fa-plus"></i>
        <span>Create</span>
    </button>
</div>

```html
<button class="pwt-button pwt-fab pwt-fab-extended">
    <i class="pwt-fab-icon fa fa-plus"></i>
    <span>Create</span>
</button>
```

## FAB Menu

FABs can expand to show multiple actions. The HTML structure involves a container `pwt-fab-menu-outer` and `pwt-fab-menu-container` for the items.

<div class="doc-example" style="height: 300px; position: relative;">
    <div class="pwt-fab-menu-outer pwt-fab-direction-up pwt-fab-align-end" style="position: absolute; bottom: 16px; right: 16px;">
        <div class="pwt-fab-menu-main">
             <button class="pwt-button pwt-fab">
                <i class="pwt-fab-icon fa fa-plus"></i>
            </button>
        </div>
        <!-- Active state would be toggled by JS/Yew, here we simulate active for demo if needed, 
             but typically hidden. To show: add 'active' to pwt-fab-menu-container -->
         <div class="pwt-fab-menu-container active">
            <button class="pwt-button pwt-fab-menu-item medium pwt-scheme-secondary-container">
                 <i class="fa fa-user"></i>
                 <span>User</span>
            </button>
             <button class="pwt-button pwt-fab-menu-item medium pwt-scheme-secondary-container">
                 <i class="fa fa-cog"></i>
                 <span>Settings</span>
            </button>
         </div>
    </div>
</div>

```html
<div class="pwt-fab-menu-outer pwt-fab-direction-up pwt-fab-align-end">
    <div class="pwt-fab-menu-main">
            <button class="pwt-button pwt-fab">
            <i class="pwt-fab-icon fa fa-plus"></i>
        </button>
    </div>
    <div class="pwt-fab-menu-container active">
        <button class="pwt-button pwt-fab-menu-item medium pwt-scheme-secondary-container">
                <i class="fa fa-user"></i>
                <span>User</span>
        </button>
            <button class="pwt-button pwt-fab-menu-item medium pwt-scheme-secondary-container">
                <i class="fa fa-cog"></i>
                <span>Settings</span>
        </button>
    </div>
</div>
```
