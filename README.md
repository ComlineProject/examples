# Comline examples

Example `.ids` schemas the [playground](https://github.com/ComlineProject/playground)
offers from its **Examples** menu. Each is a real, compilable project with
protocols you can wire up and drive in the simulate view.

## Layout

```
<id>/
  meta.json     { title, blurb, entry, order }
  <entry>.ids   the primary file (namespace = its stem)
  *.ids         any further files it `use`s
```

Add an example by adding a directory. `bundle.mjs` walks them into
`examples.json`:

```jsonc
[{ "id", "title", "blurb", "entry", "files": [{ "name", "source" }] }]
```

## Consuming it

A git dependency — no registry, no build tooling beyond Node:

```jsonc
"comline-examples": "github:ComlineProject/examples#<sha>"
```

```ts
import examples from "comline-examples"; // examples.json, built in prepare/prepack
```

`npm run bundle` regenerates `examples.json` locally.
