"""Exercise the actual AT-SPI bus, independently of GTK widget references."""
from collections import deque
import time
import pyatspi
import gi
gi.require_version('Gtk','4.0')
from gi.repository import Gtk
ERROR_RELATION = pyatspi.RELATION_DESCRIBED_BY if Gtk.get_major_version()==4 and Gtk.get_minor_version()<16 else pyatspi.RELATION_ERROR_MESSAGE
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
    queue=deque([pyatspi.Registry.getDesktop(0)]);observed=[]
    while queue and len(observed)<64:
        node=queue.popleft()
        try:
            observed.append((node.name,node.getRoleName()))
            queue.extend(node[index] for index in range(min(node.childCount,128)))
        except (RuntimeError,LookupError):pass
    print('Accessible tree diagnostic:',observed,flush=True)
    raise AssertionError('Accessible control missing: '+name)


def text(node):
    interface=node.queryText()
    # GTK 4.14 clamps negative end offsets to zero for Entry/Label.
    return interface.getText(0,interface.characterCount)


def verify():
    invalid=find('Validation draft',pyatspi.ROLE_TEXT)
    assert invalid.getState().contains(pyatspi.STATE_INVALID_ENTRY)
    relations=invalid.getRelationSet()
    errors=[relation for relation in relations if relation.getRelationType()==ERROR_RELATION]
    assert len(errors)==1 and errors[0].getNTargets()==1
    assert errors[0].getTarget(0).name=='Choose a saved workspace.'
    assert text(invalid)=='Keep this draft λ'
    assert find('Clear validation',pyatspi.ROLE_PUSH_BUTTON).queryAction().doAction(0)
    deadline=time.monotonic()+3
    while invalid.getState().contains(pyatspi.STATE_INVALID_ENTRY):
        if time.monotonic()>deadline:raise AssertionError('Clearing validation left invalid state')
        time.sleep(.05)
    assert text(invalid)=='Keep this draft λ'
    assert not any(relation.getRelationType()==ERROR_RELATION for relation in invalid.getRelationSet())
    print('PASS: independent AT-SPI invalid state, related readable error, clear and exact retained draft')
    for count,name in enumerate(('Add item','Create item λ','Next item'),1):
        button=find(name,pyatspi.ROLE_PUSH_BUTTON)
        assert button.getState().contains(pyatspi.STATE_SENSITIVE)
        assert button.queryAction().doAction(0)
        find(f'Icon activations: {count}',pyatspi.ROLE_LABEL)
    assert not find('Unavailable icon',pyatspi.ROLE_PUSH_BUTTON).getState().contains(pyatspi.STATE_SENSITIVE)
    print('PASS: independent AT-SPI named icon actions and disabled icon state')
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
    # GTK 4.14 uses SENSITIVE for disabled=false; ENABLED is not emitted.
    # https://github.com/GNOME/gtk/blob/4.14.5/gtk/a11y/gtkatspicontext.c
    assert not disabled.getState().contains(pyatspi.STATE_SENSITIVE)
    toggle=find('Wrap text',pyatspi.ROLE_CHECK_BOX)
    assert toggle.getState().contains(pyatspi.STATE_SENSITIVE)
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
    first=find('First',pyatspi.ROLE_PAGE_TAB)
    second=find('Second',pyatspi.ROLE_PAGE_TAB)
    assert first.getState().contains(pyatspi.STATE_SELECTED)
    tabs=first.parent
    assert tabs.getRole()==pyatspi.ROLE_PAGE_TAB_LIST
    # Notebook exports Selection on the tab list, not a tab Action.
    # https://github.com/GNOME/gtk/blob/4.18.6/gtk/a11y/gtkatspiselection.c
    selection=tabs.querySelection()
    for target,other in ((second,first),(first,second)):
        assert selection.selectChild(target.getIndexInParent())
        deadline=time.monotonic()+3
        while not target.getState().contains(pyatspi.STATE_SELECTED):
            if time.monotonic()>deadline:raise AssertionError('AT-SPI tab action did not select page')
            time.sleep(.05)
        assert not other.getState().contains(pyatspi.STATE_SELECTED)
        assert selection.nSelectedChildren==1
        assert selection.getSelectedChild(0).name==target.name
    print('PASS: AT-SPI tab roles, Selection interface and mutually exclusive pages')
    print('PASS: AT-SPI names/roles, Unicode editable text, disabled state and toggle action')
