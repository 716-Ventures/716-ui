import {readFile,mkdir,writeFile,copyFile} from "node:fs/promises"
import {createHash} from "node:crypto"
import {execFileSync} from "node:child_process"
import {build} from "esbuild"
import {fileURLToPath} from "node:url"
const root=fileURLToPath(new URL("../../",import.meta.url))
const here=fileURLToPath(new URL("./",import.meta.url))
const profile=JSON.parse(await readFile(`${root}reference/profile.json`,"utf8"))
for(const name of ["button.tsx","zinc.json"]){
 const bytes=await readFile(`${root}reference/${name}`)
 if(createHash("sha256").update(bytes).digest("hex")!==profile.files[name])throw Error(`Reference checksum changed: ${name}`)
}
const zinc=JSON.parse(await readFile(`${root}reference/zinc.json`,"utf8"))
const tokens=Object.keys(zinc.cssVars.light)
const out=`${root}test-output/reference-browser`
await mkdir(`${out}/fonts`,{recursive:true})
for(const [weight,key] of [[400,"REFERENCE_INTER_REGULAR"],[500,"REFERENCE_INTER_MEDIUM"],[600,"REFERENCE_INTER_SEMIBOLD"]]){
 const input=process.env[key];if(!input)throw Error(`${key} must identify the exact native qualification font`)
 await copyFile(input,`${out}/fonts/Inter-${weight}.otf`)
}
const fontLicense=process.env.REFERENCE_INTER_LICENSE
if(!fontLicense)throw Error("REFERENCE_INTER_LICENSE must identify the font copyright and license notices")
await copyFile(fontLicense,`${out}/fonts/LICENSE.txt`)
await copyFile(`${root}reference/LICENSE.shadcn`,`${out}/LICENSE.shadcn`)
const css=`@import "tailwindcss";
@source "../../reference/button.tsx";
@source "./app.tsx";
@custom-variant dark (&:is(.dark *));
@theme inline {--font-sans:"716 Reference Inter",sans-serif;${tokens.map(token=>`--color-${token}:var(--${token});`).join("")}}
:root {${tokens.map(token=>`--${token}:hsl(${zinc.cssVars.light[token]});`).join("")}}
.dark {${tokens.map(token=>`--${token}:hsl(${zinc.cssVars.dark[token]});`).join("")}}
${[400,500,600].map(weight=>`@font-face{font-family:"716 Reference Inter";font-style:normal;font-weight:${weight};src:url("fonts/Inter-${weight}.otf") format("opentype");}`).join("\n")}
body{background:var(--background);color:var(--foreground);font-family:"716 Reference Inter",sans-serif;margin:0;padding:28px;}
main{max-width:1100px;}h1{font-size:24px;font-weight:600;}h2{font-size:18px;font-weight:600;margin-bottom:16px;}header p,section p{font-size:14px;max-width:900px;margin:12px 0;}section{margin:28px 0;}.samples{display:flex;gap:12px;flex-wrap:wrap;}output{font-size:14px;}header label{display:flex;align-items:center;gap:8px;}pre{white-space:pre-wrap;overflow-wrap:anywhere;max-height:200px;overflow:auto;font-size:12px;}
`
await writeFile(`${here}generated.css`,css)
try{execFileSync(`${here}node_modules/.bin/tailwindcss`,["-i",`${here}generated.css`,"-o",`${out}/style.css`,"--minify"],{cwd:here,stdio:"inherit"})}finally{await import("node:fs/promises").then(fs=>fs.unlink(`${here}generated.css`))}
await build({entryPoints:[`${here}app.tsx`],bundle:true,nodePaths:[`${here}node_modules`],minify:true,format:"esm",outfile:`${out}/app.js`,jsx:"automatic",define:{"process.env.NODE_ENV":"\"production\""}})
await writeFile(`${out}/index.html`,"<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>716 UI pinned Button reference</title><link rel=\"stylesheet\" href=\"style.css\"></head><body><div id=\"root\"></div><script type=\"module\" src=\"app.js\"></script></body></html>")
await writeFile(`${out}/provenance.json`,JSON.stringify({upstream:profile.upstream,revision:profile.revision,files:{button:profile.files["button.tsx"],zinc:profile.files["zinc.json"]},scope:"pinned component source + Zinc tokens; default Tailwind radius/spacing; not upstream website globals",fonts:Object.fromEntries(await Promise.all([400,500,600].map(async weight=>[weight,createHash("sha256").update(await readFile(`${out}/fonts/Inter-${weight}.otf`)).digest("hex")]))),toolchain:JSON.parse(await readFile(`${here}package.json`,"utf8")).dependencies},null,2)+"\n")
console.log(out)
