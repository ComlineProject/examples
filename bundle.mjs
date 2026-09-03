// Bundle every example directory into `examples.json` — the single module the
// playground imports (`import examples from "comline-examples"`). Runs in
// `prepare` / `prepack`, so consumers never see a stale bundle and only that
// file ships (see `files` in package.json).
//
//   examples.json: [
//     { id, title, blurb, entry, files: [{ name, source }, …] }, …
//   ]
// ordered by each dir's meta.json `order`.

import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL(".", import.meta.url));

const examples = readdirSync(root, { withFileTypes: true })
	.filter((d) => d.isDirectory() && !d.name.startsWith("."))
	.map((d) => {
		const dir = `${root}/${d.name}`;
		const meta = JSON.parse(readFileSync(`${dir}/meta.json`, "utf8"));
		const files = readdirSync(dir)
			.filter((f) => f.endsWith(".ids"))
			.sort((a, b) => (a === meta.entry ? -1 : b === meta.entry ? 1 : a.localeCompare(b)))
			.map((name) => ({ name, source: readFileSync(`${dir}/${name}`, "utf8") }));
		if (!files.some((f) => f.name === meta.entry)) {
			throw new Error(`${d.name}: meta.json entry "${meta.entry}" is not a file in the directory`);
		}
		return { id: d.name, title: meta.title, blurb: meta.blurb, entry: meta.entry, order: meta.order, files };
	})
	.sort((a, b) => a.order - b.order)
	.map(({ order, ...e }) => e); // drop `order` from the output

writeFileSync(`${root}/examples.json`, JSON.stringify(examples, null, 2) + "\n");
console.log(`examples.json: ${examples.map((e) => `${e.id} (${e.files.length})`).join(", ")}`);
