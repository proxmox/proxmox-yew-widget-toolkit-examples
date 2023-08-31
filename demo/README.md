# Widegt Demo

## Running

Run a debug version of this application:

```bash
trunk serve
```

Run a release version of this application:

```bash
trunk serve --release
```

## I18N

Extract gettext messages with:

```
xgettext -d demo src/*.rs -o demo.pot
msgmerge -U demo.po demo.pot
msgfmt demo.po -o demo.mo
```