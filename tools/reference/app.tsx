import * as React from "react"
import { createRoot } from "react-dom/client"
import { Button } from "../../reference/button"
import { Input } from "../../reference/input"
const variants = ["default", "secondary", "outline", "ghost", "destructive"] as const
function App() {
  const [inputValue,setInputValue] = React.useState("Unicode λ")
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
    <header><h1>Pinned shadcn Button and Input reference</h1><p>New York v4 component, Zinc tokens, Inter from the native qualification environment. Native accessibility and documented contrast differences require separate assessment.</p><label><input aria-label="Dark appearance" type="checkbox" checked={dark} onChange={event=>setDark(event.target.checked)}/> Dark appearance</label></header>
    <section><h2>Natural default size</h2><div className="samples">{variants.map(variant=><Button key={variant} data-testid={`natural-${variant}`} variant={variant} onClick={()=>setClicks(value=>[...value,variant])}>{variant}</Button>)}</div></section>
    <section><h2>Matching state-capture allocation</h2><p>190 × 40 CSS pixels; hover, press, focus and disable are actual browser states.</p><div className="samples">{variants.map(variant=><Button style={{width:190,height:40}} key={variant} data-testid={`sample-${variant}`} variant={variant} onClick={()=>setClicks(value=>[...value,variant])}>{variant}</Button>)}</div></section>
    <section><h2>Disabled</h2><div className="samples">{variants.map(variant=><Button style={{width:190,height:40}} key={variant} data-testid={`disabled-${variant}`} variant={variant} disabled>{variant}</Button>)}</div></section>
    <section><h2>Motion observer</h2><p>Pointer entry/exit records computed browser styles over animation frames. This does not measure physical presentation.</p><Button data-testid="motion-probe" style={{width:190,height:40}} onPointerEnter={event=>traceMotion(event.currentTarget,"enter")} onPointerLeave={event=>traceMotion(event.currentTarget,"leave")}>Observe motion</Button><pre data-testid="motion-log">{motion ? JSON.stringify(motion) : "No pointer trajectory recorded"}</pre></section>
    <section><h2>Input reference</h2><p>190 CSS pixel text fields at the current responsive font size. Focus, editing and disabled behavior use real browser controls.</p><div className="samples"><Input data-testid="input-default" aria-label="Reference input" style={{width:190}} value={inputValue} onChange={event=>setInputValue(event.target.value)}/><Input data-testid="input-empty" aria-label="Empty reference input" style={{width:190}} placeholder="Placeholder"/><Input data-testid="input-disabled" aria-label="Disabled reference input" style={{width:190}} value="Disabled" disabled readOnly/></div><output data-testid="input-value">{inputValue}</output></section>
    <output aria-live="polite">Activations: {clicks.join(", ") || "none"}</output>
  </main>
}
createRoot(document.getElementById("root")!).render(<App/>)
