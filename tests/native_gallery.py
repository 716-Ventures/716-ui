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
            for appearance in ('light','dark'):
                command=[str(ROOT/'target/debug/button-states')]
                if appearance=='dark':command.append('--dark')
                subprocess.run(command,env={**env,'BUTTON_STATE_OUTPUT':str(OUTPUT/('button-states-'+appearance))},check=True,timeout=15)
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
