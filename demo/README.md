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
xgettext -d catalog src/*.rs -o catalog.pot
msgmerge -U catalog-de.po catalog.pot
msgfmt catalog-de.po -o catalog-de.mo
```

or better:

1.) Install cargo xtr expension

  cargo install xtr

and extract messages using xtr:

  xtr src/main.rs -o catalog.pot