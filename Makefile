FONT_DIR = ./assets/fonts

# Nothing to do by default
all:

roboto_base_url = "https://github.com/googlefonts/roboto-flex/raw/main/fonts"
roboto_flex_src = "RobotoFlex[GRAD,XOPQ,XTRA,YOPQ,YTAS,YTDE,YTFI,YTLC,YTUC,opsz,slnt,wdth,wght].ttf"
roboto_flex_filename = "RobotoFlexVariableFont.ttf"

update-roboto-flex:
	wget "${roboto_base_url}/${roboto_flex_src}" -O "${FONT_DIR}/${roboto_flex_filename}"


.PHONY: Documentation.md
Documentation.md:
	./extract-scss-doc.sh > $@