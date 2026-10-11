// Measure the built, checksum-pinned reference using an isolated headless Chrome.
import {createServer} from "node:http"
import {readFile,writeFile,mkdtemp,rm} from "node:fs/promises"
import {spawn} from "node:child_process"
import {tmpdir} from "node:os"
import {resolve,extname} from "node:path"
import {fileURLToPath} from "node:url"
const root=fileURLToPath(new URL("../../",import.meta.url))
const assets=resolve(root,"test-output/reference-browser")
const chrome=process.env.REFERENCE_CHROME || "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
const iconMode=process.argv.includes("--icons")
const rtlMode=process.argv.includes("--rtl")
if(rtlMode&&!iconMode)throw Error("RTL measurement requires --icons")
const selector=iconMode?'[data-testid^="icon-"]':'[data-testid^="size-"]'
const sampleCount=iconMode?96:24
const script=`
import "./app.js";
try {
 for(let i=0;document.querySelectorAll('${selector}').length!==${sampleCount};i++) {
  if(i===40)throw Error("Reference sizes did not render");
  await new Promise(resolve=>setTimeout(resolve,25));
 }
 await document.fonts.load('500 12px "716 Reference Inter"');
 await document.fonts.load('500 14px "716 Reference Inter"');
 await document.fonts.ready;
 document.documentElement.dir=${JSON.stringify(rtlMode?"rtl":"ltr")};
 // Measure settled endpoints, not the first frame of transition-all after a
 // theme toggle. Motion has its own observer and is not measured here.
 const endpointStyle=document.createElement("style");
 endpointStyle.textContent="*,*::before,*::after{transition:none!important;animation:none!important}";
 document.head.append(endpointStyle);
 const metrics=[];
 for(const appearance of ${JSON.stringify(iconMode?["light","dark"]:["light"])}) {
 document.documentElement.classList.toggle("dark",appearance==="dark");
 metrics.push(...[...document.querySelectorAll('${selector}')].map(button=>{
  const bounds=button.getBoundingClientRect(),style=getComputedStyle(button);
  const svg=button.querySelector("svg"),icon=svg?.getBoundingClientRect();
  const iconColor=svg?getComputedStyle(svg).color:null;
  const icon_rgb=iconColor?.match(/[0-9.]+/g)?.slice(0,3).map(Number);
  const label=[...button.childNodes].find(node=>node.nodeType===Node.TEXT_NODE&&node.textContent.trim());
  let icon_side="only";
  if(label&&icon){const range=document.createRange();range.selectNodeContents(label);icon_side=icon.left<range.getBoundingClientRect().left?"left":"right";}
  return {appearance,icon_rgb,...(${rtlMode}?{direction:style.direction,icon_side}:{}),gap:parseFloat(style.columnGap)||0,size:button.dataset.referenceSize||button.dataset.size,mode:button.dataset.mode,icon:button.dataset.icon,icon_width:icon?.width,icon_height:icon?.height,variant:button.dataset.variant,width:bounds.width,height:bounds.height,font_size_px:parseFloat(style.fontSize),font_weight:parseInt(style.fontWeight),font_family:style.fontFamily};
 }));
 }
 document.body.dataset.measurement=btoa(JSON.stringify({browser:navigator.userAgent,metrics}));
} catch(error) {document.body.dataset.measurement=btoa(JSON.stringify({error:String(error)}));}
`
const profile=await mkdtemp(resolve(tmpdir(),"716-button-reference-"))
let child
const server=createServer(async(request,response)=>{
 try {
  const path=resolve(assets,"."+new URL(request.url,"http://localhost").pathname)
  if(path!==assets&&!path.startsWith(assets+"/")){response.writeHead(403).end();return}
  if(path===assets+"/measure.html") {
   const html=(await readFile(resolve(assets,"index.html"),"utf8")).replace('<script type="module" src="app.js"></script>',`<script type="module">${script}</script>`)
   response.setHeader("Content-Type","text/html");response.end(html);return
  }
  response.setHeader("Content-Type",({".js":"text/javascript",".css":"text/css",".otf":"font/otf"})[extname(path)]||"application/octet-stream")
  response.end(await readFile(path))
 } catch {response.writeHead(404).end()}
})
try {
 await new Promise(resolve=>server.listen(0,"127.0.0.1",resolve))
 const run=args=>new Promise((resolve,reject)=>{
  child=spawn(chrome,args,{stdio:["ignore","pipe","pipe"],detached:process.platform!=="win32"});let stdout="",stderr=""
  const timer=setTimeout(()=>{child.kill("SIGKILL");reject(Error("Headless reference timed out: "+stderr.slice(-2048)))},20000)
  child.stdout.on("data",chunk=>stdout+=chunk);child.stderr.on("data",chunk=>stderr+=chunk)
  child.once("error",error=>{clearTimeout(timer);reject(error)})
  child.once("close",code=>{clearTimeout(timer);code===0?resolve(stdout):reject(Error(`Chrome exited ${code}: ${stderr.slice(-2048)}`))})
 })
 const html=await run(["--headless","--disable-gpu","--disable-breakpad","--disable-crash-reporter","--no-first-run","--no-default-browser-check",`--user-data-dir=${profile}`,"--virtual-time-budget=4000","--dump-dom",`http://127.0.0.1:${server.address().port}/measure.html`])
 const encoded=html.match(/data-measurement="([A-Za-z0-9+/=]+)"/)?.[1]
 if(!encoded)throw Error("Reference returned no measurement")
 const result=JSON.parse(Buffer.from(encoded,"base64").toString("utf8"))
 if(result.error)throw Error(result.error)
 const provenance=JSON.parse(await readFile(resolve(assets,"provenance.json"),"utf8"))
 const report={measured_at:new Date().toISOString(),profile_revision:provenance.revision,browser:result.browser,scope:iconMode?(rtlMode?"RTL leading/trailing/icon-only controls, pinned Lucide Plus/ArrowRight; measured ordering and geometry, not full RTL language, paint or motion parity":"Leading/trailing/icon-only controls, pinned Lucide Plus/ArrowRight; measured geometry, not full paint or motion parity"):"Natural text-only xs, sm, default and lg; pinned component, Zinc, Inter and default Tailwind spacing; not full appearance or motion parity",native_width_tolerance_px:1,provenance,metrics:result.metrics}
 if(process.argv[2])await writeFile(resolve(process.argv[2]),JSON.stringify(report,null,2)+"\n")
 console.log(JSON.stringify(report,null,2))
} finally {
 if(child) {
  if(process.platform!=="win32") {try {process.kill(-child.pid,"SIGKILL")} catch(error) {if(error.code!=="ESRCH")throw error}}
  else if(child.exitCode===null)child.kill("SIGKILL")
 }
 server.closeAllConnections()
 await new Promise(resolve=>server.close(resolve))
 await rm(profile,{recursive:true,force:true})
}
