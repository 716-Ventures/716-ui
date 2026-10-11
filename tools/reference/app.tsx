import * as React from "react"
import { createRoot } from "react-dom/client"
import { Button } from "../../reference/button"
import { Checkbox } from "../../reference/checkbox"
import { Input } from "../../reference/input"
import { Tabs, TabsList, TabsTrigger, TabsContent } from "../../reference/tabs"
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "../../reference/select"
import { Table, TableHeader, TableBody, TableHead, TableRow, TableCell } from "../../reference/table"
const variants = ["default", "secondary", "outline", "ghost", "destructive"] as const
function App() {
  const [inputValue,setInputValue] = React.useState("Unicode λ")
  const [checked,setChecked] = React.useState(false)
  const [choice,setChoice] = React.useState("alpha")
  const [selectedRow,setSelectedRow] = React.useState<string|null>(null)
  const [tab,setTab] = React.useState("account")
  const [dark, setDark] = React.useState(false)
  const [clicks, setClicks] = React.useState<string[]>([])
  const [motion, setMotion] = React.useState<unknown>(null)
  const motionEpoch = React.useRef(0)
  function traceMotion(element: HTMLButtonElement, direction: string) {
    const epoch = ++motionEpoch.current
    const started = performance.now()
    const samples: unknown[] = []
    function frame(now: number) {
      if(epoch !== motionEpoch.current)return
      const style = getComputedStyle(element)
      samples.push({elapsed_ms:Math.max(0,now-started),hover:element.matches(":hover"),background:style.backgroundColor,transition:style.transitionDuration,easing:style.transitionTimingFunction})
      if(now-started<250)requestAnimationFrame(frame)
      else setMotion({direction,complete:true,samples})
    }
    requestAnimationFrame(frame)
  }
  React.useEffect(() => {document.documentElement.classList.toggle("dark",dark)}, [dark])
  return <main>
    <header><h1>Pinned shadcn Button, Input, Tabs, Checkbox, Select and Table reference</h1><p>New York v4 component, Zinc tokens, Inter from the native qualification environment. Native accessibility and documented contrast differences require separate assessment.</p><label><input aria-label="Dark appearance" type="checkbox" checked={dark} onChange={event=>setDark(event.target.checked)}/> Dark appearance</label></header>
    <section><h2>Natural default size</h2><div className="samples">{variants.map(variant=><Button key={variant} data-testid={`natural-${variant}`} variant={variant} onClick={()=>setClicks(value=>[...value,variant])}>{variant}</Button>)}</div></section>
    <section><h2>Matching state-capture allocation</h2><p>190 × 40 CSS pixels; hover, press, focus and disable are actual browser states.</p><div className="samples">{variants.map(variant=><Button style={{width:190,height:40}} key={variant} data-testid={`sample-${variant}`} variant={variant} onClick={()=>setClicks(value=>[...value,variant])}>{variant}</Button>)}</div></section>
    <section><h2>Disabled</h2><div className="samples">{variants.map(variant=><Button style={{width:190,height:40}} key={variant} data-testid={`disabled-${variant}`} variant={variant} disabled>{variant}</Button>)}</div></section>
    <section><h2>Motion observer</h2><p>Pointer entry/exit records computed browser styles over animation frames. This does not measure physical presentation.</p><Button data-testid="motion-probe" style={{width:190,height:40}} onPointerEnter={event=>traceMotion(event.currentTarget,"enter")} onPointerLeave={event=>traceMotion(event.currentTarget,"leave")}>Observe motion</Button><pre data-testid="motion-log">{motion ? JSON.stringify(motion) : "No pointer trajectory recorded"}</pre></section>
    <section><h2>Input reference</h2><p>190 CSS pixel text fields at the current responsive font size. Focus, editing and disabled behavior use real browser controls.</p><div className="samples"><Input data-testid="input-default" aria-label="Reference input" style={{width:190}} value={inputValue} onChange={event=>setInputValue(event.target.value)}/><Input data-testid="input-empty" aria-label="Empty reference input" style={{width:190}} placeholder="Placeholder"/><Input data-testid="input-disabled" aria-label="Disabled reference input" style={{width:190}} value="Disabled" disabled readOnly/></div><output data-testid="input-value">{inputValue}</output></section>
    <section><h2>Tabs reference</h2><Tabs value={tab} onValueChange={setTab} data-testid="tabs-default"><TabsList aria-label="Reference settings"><TabsTrigger value="account">Account</TabsTrigger><TabsTrigger value="security">Security</TabsTrigger><TabsTrigger value="disabled" disabled>Unavailable</TabsTrigger></TabsList><TabsContent value="account">Account λ content</TabsContent><TabsContent value="security">Security 日本語 content</TabsContent></Tabs><output data-testid="tabs-value">Selected: {tab}</output></section>
    <section><h2>Select reference</h2><div className="samples"><Select value={choice} onValueChange={setChoice}><SelectTrigger data-testid="select-default" aria-label="Reference choice" style={{width:190}}><SelectValue/></SelectTrigger><SelectContent><SelectItem value="alpha">Alpha λ</SelectItem><SelectItem value="beta">Beta 日本語</SelectItem><SelectItem value="disabled" disabled>Unavailable</SelectItem></SelectContent></Select><Select value="alpha" disabled><SelectTrigger data-testid="select-disabled" aria-label="Disabled reference choice" style={{width:190}}><SelectValue/></SelectTrigger><SelectContent><SelectItem value="alpha">Alpha λ</SelectItem></SelectContent></Select></div><output data-testid="select-value">Selected: {choice}</output></section>
    <section><h2>Checkbox reference</h2><div className="samples"><label className="flex items-center gap-2"><Checkbox data-testid="checkbox-default" aria-label="Reference checkbox" checked={checked} onCheckedChange={value=>setChecked(value===true)}/>Enabled λ 日本語</label><label className="flex items-center gap-2"><Checkbox data-testid="checkbox-checked" aria-label="Checked reference checkbox" checked/>Checked</label><label className="flex items-center gap-2"><Checkbox data-testid="checkbox-disabled" aria-label="Disabled reference checkbox" checked disabled/>Disabled</label></div><output data-testid="checkbox-value">Checked: {String(checked)}</output></section>
    <section><h2>Table reference</h2><p>Read-only rows; the external fixture control sets the upstream selected visual state.</p><Button data-testid="table-select" onClick={()=>setSelectedRow("row-2")}>Select fixture row</Button><div style={{width:360,marginTop:12}}><Table aria-label="Reference table" data-testid="table-default"><TableHeader><TableRow><TableHead>Resource</TableHead><TableHead>Status</TableHead></TableRow></TableHeader><TableBody>{[["Document λ","Saved"],["Background job","Running"],["External app","Observed"]].map((cells,index)=><TableRow key={index} data-testid={`table-row-${index}`} data-state={selectedRow===`row-${index}`?"selected":undefined}>{cells.map((cell,column)=><TableCell key={column}>{cell}</TableCell>)}</TableRow>)}</TableBody></Table></div></section>
    <output aria-live="polite">Activations: {clicks.join(", ") || "none"}</output>
  </main>
}
createRoot(document.getElementById("root")!).render(<App/>)
