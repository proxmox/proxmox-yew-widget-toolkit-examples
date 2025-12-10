FONT_DIR = ./assets/fonts

# Nothing to do by default
all:

ROBOTO_FLEX_URL = "https://github.com/googlefonts/roboto-flex/raw/refs/heads/main/fonts/RobotoFlex%5BGRAD,XOPQ,XTRA,YOPQ,YTAS,YTDE,YTFI,YTLC,YTUC,opsz,slnt,wdth,wght%5D.ttf"

ROBOTO_FLEX_FILENAME_TTF = RobotoFlexVariableFont.ttf
ROBOTO_FLEX_FILENAME_WOFF2 = RobotoFlexVariableFont.woff2

.PHONY: update-roboto-flex
update-roboto-flex:
	cd $(FONT_DIR) && rm -f ./$(ROBOTO_FLEX_FILENAME_TTF) ./$(ROBOTO_FLEX_FILENAME_WOFF2)
	$(MAKE) $(FONT_DIR)/$(ROBOTO_FLEX_FILENAME_TTF) $(FONT_DIR)/$(ROBOTO_FLEX_FILENAME_WOFF2)

$(FONT_DIR)/$(ROBOTO_FLEX_FILENAME_TTF):
	rm -f "$(FONT_DIR)/$(ROBOTO_FLEX_FILENAME_TTF).tmp"
	wget $(ROBOTO_FLEX_URL) -O "$(FONT_DIR)/$(ROBOTO_FLEX_FILENAME_TTF).tmp"
	mv "$(FONT_DIR)/$(ROBOTO_FLEX_FILENAME_TTF).tmp" "$(FONT_DIR)/$(ROBOTO_FLEX_FILENAME_TTF)"

$(FONT_DIR)/$(ROBOTO_FLEX_FILENAME_WOFF2): $(FONT_DIR)/$(ROBOTO_FLEX_FILENAME_TTF)
	@command -v woff2_compress > /dev/null || { echo 'woff2_compress not found, use: apt install woff2'; exit 1; }
	woff2_compress "$(FONT_DIR)/$(ROBOTO_FLEX_FILENAME_TTF)"

.PHONY: Documentation.md
Documentation.md:
	./extract-scss-doc.sh > $@
