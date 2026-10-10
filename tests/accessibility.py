"""Exercise the actual AT-SPI bus, independently of GTK widget references."""
from collections import deque
import time
import pyatspi


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


def verify():
    field=find('Your draft',pyatspi.ROLE_TEXT)
    assert field.queryEditableText().setTextContents('AT-SPI draft λ 日本語')
    deadline=time.monotonic()+3
    while field.queryText().getText(0,-1)!='AT-SPI draft λ 日本語':
        if time.monotonic()>deadline:raise AssertionError('AT-SPI edit observed '+repr(field.queryText().getText(0,-1)))
        time.sleep(.05)
    assert field.queryComponent().grabFocus()
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
    assert heading.queryText().getText(0,-1)=='Reading section'
    rich=find('Literal <b>日本語</b> & λ')
    assert rich.queryText().getText(0,-1)=='Literal <b>日本語</b> & λ'
    print('PASS: AT-SPI names/roles, Unicode editable text, native focus, disabled state and toggle action')
