#!/usr/bin/env python3
"""Run the real GTK gallery in a private Wayland display; always reap the display."""
import os
from pathlib import Path
import signal
import subprocess
import tempfile
import time

ROOT=Path(__file__).resolve().parents[1]
OUTPUT=ROOT/'test-output'
OUTPUT.mkdir(exist_ok=True)
with tempfile.TemporaryDirectory(prefix='716-wayland-') as runtime:
    env={**os.environ,'XDG_RUNTIME_DIR':runtime,'WAYLAND_DISPLAY':'716-test','GDK_BACKEND':'wayland','GSK_RENDERER':'cairo','GTK_A11Y':'atspi','G_DEBUG':'fatal-criticals'}
    with (OUTPUT/'weston.log').open('w') as log:
        compositor=subprocess.Popen(['weston','--backend=headless-backend.so','--use-pixman','--socket=716-test','--idle-time=0','--width=1280','--height=800'],env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
        try:
            deadline=time.monotonic()+15
            while not (Path(runtime)/'716-test').exists():
                if compositor.poll() is not None:raise RuntimeError('Nested Weston failed; inspect test-output/weston.log')
                if time.monotonic()>deadline:raise RuntimeError('Nested display startup timed out')
                time.sleep(.05)
            result=subprocess.run([str(ROOT/'target/debug/text-area-events')],env=env,capture_output=True,text=True,timeout=20)
            (OUTPUT/'text-area-events.log').write_text(result.stdout+result.stderr)
            result.check_returncode()
            for appearance in ('light','dark'):
                outcome=[str(ROOT/'target/debug/outcome-updates')]+(['--dark'] if appearance=='dark' else [])
                result=subprocess.run(outcome,env=env,capture_output=True,text=True,timeout=20)
                (OUTPUT/('outcome-updates-'+appearance+'.log')).write_text(result.stdout+result.stderr)
                result.check_returncode()
                for case in ('positive','negative','mixed','zero','tiny','subnormal','integer'):
                    chart=[str(ROOT/'target/debug/chart-states')]
                    if appearance=='dark':chart.append('--dark')
                    result=subprocess.run(chart,env={**env,'CHART_STATE_OUTPUT':str(OUTPUT/('chart-states-'+appearance)), 'CHART_CASE':case},capture_output=True,text=True,timeout=15)
                    (OUTPUT/('chart-states-'+appearance+'-'+case+'.json')).write_text(result.stdout)
                    if result.stderr:print(result.stderr)
                    result.check_returncode()
                updates=[str(ROOT/'target/debug/collection-updates')]+(['--dark'] if appearance=='dark' else [])
                result=subprocess.run(updates,env=env,capture_output=True,text=True,timeout=20)
                (OUTPUT/('collection-updates-'+appearance+'.log')).write_text(result.stdout+result.stderr)
                result.check_returncode()
                result=subprocess.run([str(ROOT/'target/debug/table-clipboard')],env={**env,'TABLE_CLIPBOARD_LOCAL':'1'},capture_output=True,text=True,timeout=20)
                (OUTPUT/('table-clipboard-'+appearance+'.log')).write_text(result.stdout+result.stderr)
                result.check_returncode()
                tables=[str(ROOT/'target/debug/table-states')]
                if appearance=='dark':tables.append('--dark')
                result=subprocess.run(tables,env={**env,'TABLE_STATE_OUTPUT':str(OUTPUT/('table-states-'+appearance))},capture_output=True,text=True,timeout=15)
                (OUTPUT/('table-states-'+appearance+'.json')).write_text(result.stdout)
                if result.stderr:print(result.stderr)
                result.check_returncode()
                choices=[str(ROOT/'target/debug/choice-states')]
                if appearance=='dark':choices.append('--dark')
                result=subprocess.run(choices,env={**env,'CHOICE_STATE_OUTPUT':str(OUTPUT/('choice-states-'+appearance))},capture_output=True,text=True,timeout=15)
                (OUTPUT/('choice-states-'+appearance+'.json')).write_text(result.stdout)
                if result.stderr:print(result.stderr)
                result.check_returncode()
                toggles=[str(ROOT/'target/debug/toggle-states')]
                if appearance=='dark':toggles.append('--dark')
                result=subprocess.run(toggles,env={**env,'TOGGLE_STATE_OUTPUT':str(OUTPUT/('toggle-states-'+appearance))},capture_output=True,text=True,timeout=15)
                (OUTPUT/('toggle-states-'+appearance+'.json')).write_text(result.stdout)
                if result.stderr:print(result.stderr)
                result.check_returncode()
                tabs=[str(ROOT/'target/debug/tabs-states')]
                if appearance=='dark':tabs.append('--dark')
                result=subprocess.run(tabs,env={**env,'TABS_STATE_OUTPUT':str(OUTPUT/('tabs-states-'+appearance))},capture_output=True,text=True,timeout=15)
                (OUTPUT/('tabs-states-'+appearance+'.json')).write_text(result.stdout)
                if result.stderr:print(result.stderr)
                result.check_returncode()
                fields=[str(ROOT/'target/debug/input-states')]
                if appearance=='dark':fields.append('--dark')
                result=subprocess.run(fields,env={**env,'INPUT_STATE_OUTPUT':str(OUTPUT/('input-states-'+appearance))},capture_output=True,text=True,timeout=15)
                (OUTPUT/('input-states-'+appearance+'.json')).write_text(result.stdout)
                if result.stderr:print(result.stderr)
                result.check_returncode()
                metrics=[str(ROOT/'target/debug/button-metrics')]
                if appearance=='dark':metrics.append('--dark')
                result=subprocess.run(metrics,env=env,capture_output=True,text=True,check=True,timeout=15)
                (OUTPUT/('button-metrics-'+appearance+'.json')).write_text(result.stdout)
                if result.stderr:print(result.stderr)
                command=[str(ROOT/'target/debug/button-states')]
                if appearance=='dark':command.append('--dark')
                subprocess.run(command,env={**env,'BUTTON_STATE_OUTPUT':str(OUTPUT/('button-states-'+appearance))},check=True,timeout=15)
                for reduced in (False,True):
                    motion=[str(ROOT/'target/debug/button-motion')]
                    if appearance=='dark':motion.append('--dark')
                    if reduced:motion.append('--reduced-motion')
                    result=subprocess.run(motion,env=env,capture_output=True,text=True,timeout=15)
                    (OUTPUT/('button-motion-'+appearance+('-reduced' if reduced else '')+'.json')).write_text(result.stdout)
                    if result.stderr:print(result.stderr)
                    result.check_returncode()
                for scale in (1,2):
                    command=[str(ROOT/'target/debug/gallery'),'--self-test','--capture',str(OUTPUT/(appearance+'-'+str(scale)+'.png')),'--scale',str(scale)]
                    if appearance=='dark':command.append('--dark')
                    if scale==2:command.append('--reduced-motion')
                    subprocess.run(command,env=env,check=True,timeout=30)
            # Keep a gallery alive for an independent assistive-client journey.
            with (OUTPUT/'accessibility.log').open('w') as accessible_log:
                gallery=subprocess.Popen([str(ROOT/'target/debug/gallery')],env=env,stdout=accessible_log,stderr=subprocess.STDOUT,start_new_session=True)
                try:
                    import accessibility
                    accessibility.verify()
                finally:
                    if gallery.poll() is None:os.killpg(gallery.pid,signal.SIGTERM)
                    try:gallery.wait(timeout=5)
                    except subprocess.TimeoutExpired:os.killpg(gallery.pid,signal.SIGKILL);gallery.wait(timeout=5)
        finally:
            os.killpg(compositor.pid,signal.SIGTERM) if compositor.poll() is None else None
            try:compositor.wait(timeout=5)
            except subprocess.TimeoutExpired:os.killpg(compositor.pid,signal.SIGKILL);compositor.wait(timeout=5)
print('PASS: independent native gallery, light/dark captures, display cleanup')
