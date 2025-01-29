#!/bin/bash

# Experimental, super simple script to generate useful doc

add_doc () {
    cat $1
    echo ""

}

extract_doc () {
    sed -n  '/\s*\/\/\// p' $1 | sed -e 's/\s*\/\/\/\s\?//'
    echo ""

}

# General introduction, explain themes
add_doc "scss/introduction.md"

# Explain desktop theme
extract_doc "scss/desktop/_theme_config_desktop.scss"
extract_doc "scss/desktop/_theme_colors_adwaita.scss"
extract_doc "scss/desktop/_theme_post_desktop.scss"

# Explain material theme
extract_doc "scss/material/_theme_config_material.scss"
extract_doc "scss/material/_theme_colors_material.scss"
extract_doc "scss/material/_theme_post_material.scss"


# Explain crisp theme
extract_doc "scss/crisp/_theme_config_crisp.scss"
extract_doc "scss/crisp/_theme_colors_crisp.scss"
extract_doc "scss/crisp/_theme_post_crisp.scss"


# Explain common utility classes
extract_doc "scss/_utilities.scss"

# Explain individual widget classes
extract_doc "scss/_panel.scss"
