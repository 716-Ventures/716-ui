"""Exercise the actual AT-SPI bus, independently of GTK widget references."""
from collections import deque
import time
import pyatspi
from gi.repository import GLib


def find(name,role=None):
    deadline=time.monotonic()+10
    while time.monotonic()<deadline:
        queue=deque([pyatspi.Registry.getDesktop(0)]);visited=0
        while queue and visited<4096:
            node=queue.popleft();visited+=1
            try:
                if node.name==name and (role is None or node.getRole()==role):return node
                queue.extend(node[index] for index in range(min(node.childCount,128)))
            except (RuntimeError,LookupError):pass
        time.sleep(.1)
    raise AssertionError('Accessible control missing: '+name)


def text(node):
    interface=node.queryText()
    # GTK 4.14 clamps negative end offsets to zero for Entry/Label.
    return interface.getText(0,interface.characterCount)


def verify():
    field=find('Your draft',pyatspi.ROLE_TEXT)
    assert field.queryEditableText().setTextContents('AT-SPI draft λ 日本語')
    deadline=time.monotonic()+3
    while text(field)!='AT-SPI draft λ 日本語':
        if time.monotonic()>deadline:raise AssertionError('AT-SPI edit observed '+repr(text(field)))
        time.sleep(.05)
    try:
        assert field.queryComponent().grabFocus()
    except GLib.GError as error:
        # GTK 4.14 deliberately returns D-Bus NotSupported for GrabFocus.
        if error.code!=1:raise
        print('LIMITATION: GTK 4.14 AT-SPI GrabFocus is unsupported; native keyboard focus is checked separately')
    disabled=find('Unavailable',pyatspi.ROLE_PUSH_BUTTON)
    assert not disabled.getState().contains(pyatspi.STATE_ENABLED)
    toggle=find('Wrap text',pyatspi.ROLE_CHECK_BOX)
    before=toggle.getState().contains(pyatspi.STATE_CHECKED)
    action=toggle.queryAction();assert action.nActions>0
    assert action.doAction(0)
    deadline=time.monotonic()+3
    while toggle.getState().contains(pyatspi.STATE_CHECKED)==before:
        if time.monotonic()>deadline:raise AssertionError('AT-SPI toggle action did not change native state')
        time.sleep(.05)
    heading=find('Reading section',pyatspi.ROLE_HEADING)
    assert text(heading)=='Reading section'
    rich=find('Literal <b>日本語</b> & λ')
    assert text(rich)=='Literal <b>日本語</b> & λ'
    print('PASS: AT-SPI names/roles, Unicode editable text, disabled state and toggle action')
