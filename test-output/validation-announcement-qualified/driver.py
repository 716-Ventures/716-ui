import subprocess,io,tarfile,sys,os,time,json,hashlib
from pathlib import Path
root=Path('/Users/chrisjdavis/development/716-ui')
sys.path.insert(0,'outputs/vm-foundation');import vm
os.environ['AGENT_OS_IMAGE_OWNERSHIP']='/tmp/716-validation-announcement-ownership.json'
import agentos_image_owned_utm as ownership
archive=io.BytesIO()
with tarfile.open(fileobj=archive,mode='w') as tar:
 for raw in subprocess.check_output(['git','-C',str(root),'ls-files','-z']).split(b'\0'):
  if raw:
   p=Path(raw.decode());assert not (root/p).is_symlink();tar.add(root/p,arcname=str(p),recursive=False)
 for p in (root/'test-output/reference-browser').rglob('*'):
  assert not p.is_symlink()
  if p.is_file():tar.add(p,arcname=str(p.relative_to(root)),recursive=False)
 for name in ('src/validation.rs','src/bin/input_validation.rs'):
  tar.add(root/name,arcname=name,recursive=False)
additions=[]
defaults={}
receipt={'base_revision':subprocess.check_output(['git','-C',str(root),'rev-parse','HEAD'],text=True).strip(),'implementation_diff_sha256':hashlib.sha256(subprocess.check_output(['git','-C',str(root),'diff','--binary'])).hexdigest(),'passed':False}
try:
 assert vm.status()=='stopped';vm.start();vm.wait_ready(60)
 subprocess.run(vm.ssh_args()+['tar xf - -C /home/developer/716-ui-verification'],input=archive.getvalue(),check=True)
 vm.ssh('cd /home/developer/716-ui-verification && cargo test --locked --no-default-features && cargo test --locked --all-features && cargo build --locked --all-features && dbus-run-session -- python3 tests/native_gallery.py',check=True)
 dest=root/'test-output/validation-announcement-qualified';dest.mkdir(parents=True,exist_ok=True)
 for appearance in ('light','dark'):
  for scale in (1,2):
   names=[f'input-validation-{appearance}-{scale}.{suffix}' for suffix in ('png','log')]

   for name in names:
    with (dest/name).open('wb') as target:vm.ssh('cat /home/developer/716-ui-verification/test-output/'+name,stdout=target,check=True)
 for name in ('accessibility.log','accessibility-rtl.log'):
  with (dest/name).open('wb') as target:vm.ssh('cat /home/developer/716-ui-verification/test-output/'+name,stdout=target,check=True)
 receipt['passed']=True
 print('PASS: focused validation clear announcement support and complete gallery with LTR and RTL independent name/selection/error journeys',flush=True)
finally:
 vm.ssh('head -n 8 /home/developer/716-ui-verification/test-output/input-validation-light-1.log',check=False)
 vm.stop();deadline=time.monotonic()+45
 while vm.status()!='stopped':
  assert time.monotonic()<deadline;time.sleep(.5)
 ownership.close();assert vm.utm_application_identity() is None
 receipt.update({'guest_stopped':True,'owned_utm_app_closed':True})
 Path('/tmp/716-validation-announcement-qualify.json').write_text(json.dumps(receipt,indent=2)+'\n')
 print(json.dumps(receipt),flush=True)
