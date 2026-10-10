import * as React from "react"
import { createRoot } from "react-dom/client"
import { Button } from "../../reference/button"
const variants = ["default", "secondary", "outline", "ghost", "destructive"] as const
function App() {
  const [dark, setDark] = React.useState(false)
  const [clicks, setClicks] = React.useState<string[]>([])
  React.useEffect(() => {document.documentElement.classList.toggle("dark",dark)}, [dark])
  return <main>
    <header><h1>Pinned shadcn Button reference</h1><p>New York v4 component, Zinc tokens, Inter from the native qualification environment. Native accessibility and documented contrast differences require separate assessment.</p><label><input aria-label="Dark appearance" type="checkbox" checked={dark} onChange={event=>setDark(event.target.checked)}/> Dark appearance</label></header>
    <section><h2>Natural default size</h2><div className="samples">{variants.map(variant=><Button key={variant} data-testid={`natural-${variant}`} variant={variant} onClick={()=>setClicks(value=>[...value,variant])}>{variant}</Button>)}</div></section>
    <section><h2>Matching state-capture allocation</h2><p>190 × 40 CSS pixels; hover, press, focus and disable are actual browser states.</p><div className="samples">{variants.map(variant=><Button style={{width:190,height:40}} key={variant} data-testid={`sample-${variant}`} variant={variant} onClick={()=>setClicks(value=>[...value,variant])}>{variant}</Button>)}</div></section>
    <section><h2>Disabled</h2><div className="samples">{variants.map(variant=><Button style={{width:190,height:40}} key={variant} data-testid={`disabled-${variant}`} variant={variant} disabled>{variant}</Button>)}</div></section>
    <output aria-live="polite">Activations: {clicks.join(", ") || "none"}</output>
  </main>
}
createRoot(document.getElementById("root")!).render(<App/>)
