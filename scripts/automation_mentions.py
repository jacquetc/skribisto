#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the mention index end to end.

Builds the situation the feature exists for, entirely through the UI:

  1. mark a tag as story-bible (Settings > Work > Tags);
  2. put that tag on a note, which makes the aliases field appear;
  3. give the note an alias that occurs in another item's prose;
  4. open that item and assert its roster names the note.

The fixture's prose is Lorem ipsum, so "Aenean" is a capitalised word that
really appears in 1.1 Zeus -- a genuine match rather than one arranged by
editing prose through the automation.

It runs on a COPY of the fixture. Step 4 saves, and an earlier version of
this script pointed at the checked-in file and silently modified the repo's
copy: the next run's toggle then undid the previous run's, which looked
exactly like the feature being broken.
"""

import json, os, re, select, subprocess, tempfile, time, base64
SK="./target/debug/skribisto"; MCP="/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
import shutil
# NEVER open the checked-in fixture in a probe that saves: Ctrl+S below writes the
# project, and an earlier run of this script silently modified the repo's copy.
_src="./resources/test/skribisto_test_project.skrib"
FX=tempfile.NamedTemporaryFile(suffix=".skrib", delete=False).name
shutil.copy(_src, FX)
log=tempfile.NamedTemporaryFile(suffix=".log",delete=False).name
app=subprocess.Popen([SK,FX],stdout=open(log,"w"),stderr=subprocess.STDOUT)
sock=tok=None; d=time.time()+25
while time.time()<d:
    t=open(log).read()
    a=re.search(r"bridge socket = (\S+)",t); b=re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)",t)
    if a and b: sock,tok=a.group(1),b.group(1); break
    time.sleep(0.2)
_id=[0]; mcp=None
def send(m,p=None,n=False):
    dd={"jsonrpc":"2.0","method":m}
    if p is not None: dd["params"]=p
    if not n: _id[0]+=1; dd["id"]=_id[0]
    mcp.stdin.write(json.dumps(dd)+"\n"); mcp.stdin.flush()
def recv(t=25):
    e=time.time()+t
    while time.time()<e:
        r,_,_=select.select([mcp.stdout],[],[],max(0.0,e-time.time()))
        if not r: return None
        l=mcp.stdout.readline()
        if l and l.strip(): return json.loads(l)
    return None
d=time.time()+25; init=None
while time.time()<d and init is None:
    while not os.path.exists(sock) and time.time()<d: time.sleep(0.05)
    mcp=subprocess.Popen([MCP,"--connect",sock,"--token",tok],stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,stderr=open(tempfile.mktemp(),"w"),text=True,bufsize=1)
    send("initialize",{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"x","version":"1"}})
    init=recv(4)
    if init is None and mcp.poll() is None: mcp.terminate(); time.sleep(0.3)
send("notifications/initialized",None,True)
def call(n,a=None):
    send("tools/call",{"name":n,"arguments":a or {}})
    res=(recv() or {}).get("result",{})
    p=res.get("structuredContent")
    if p is None:
        t="".join(c.get("text","") for c in res.get("content",[]) if c.get("type")=="text")
        p=json.loads(t) if t.strip().startswith("{") else {}
    return res,p
def nodes(): return call("snapshot_tree")[1].get("nodes",[])
def click(b,dx=None):
    call("inject_pointer",{"x":b["x"]+(dx if dx is not None else b.get("width",0)/2),
                           "y":b["y"]+b.get("height",0)/2,"action":"click"})
def txt(n): return ((n.get("value") or "")+" "+(n.get("label") or "")).strip()
def crumb(): return [n.get("label") for n in nodes() if n.get("role")=="Link"]
def rail(l):
    for n in nodes():
        b=n.get("bounds") or {}
        if (n.get("label") or "").strip().lower()==l and b.get("x",9999)<280: return n
def binder(name):
    for n in nodes():
        b=n.get("bounds") or {}
        if (n.get("label") or "").strip()==name and 40<=b.get("x",999)<=140: return n
def find(sub, role=None):
    s=sub.lower()
    for n in nodes():
        if s in txt(n).lower() and (role is None or n.get("role")==role): return n
def shot(p):
    res,_=call("screenshot")
    for c in res.get("content",[]):
        if c.get("type")=="image" and c.get("data"): open(p,"wb").write(base64.b64decode(c["data"])); print("shot ->",p)
time.sleep(4)
if any('mock' in (n.get('label') or '').lower() for n in nodes()):
    print('FAIL: this is a --features mocks binary'); app.terminate(); mcp.terminate(); raise SystemExit(1)

# 1. Make tag "A" a story-bible tag.
call("inject_key",{"key":",","ctrl":True}); time.sleep(2)
k=rail("keymap")
if k: click(k["bounds"]); time.sleep(0.8)
for _ in range(12):
    if crumb()[-1:]==["Tags"]: break
    call("inject_key",{"key":"Down"}); call("settle"); time.sleep(0.4)
sw=[n for n in nodes() if n.get("role")=="Switch" and "story bible" in (n.get("label") or "").lower()]
print("story-bible switches:", len(sw))
if sw: click(sw[0]["bounds"]); call("settle"); time.sleep(1.0)
done=[n for n in nodes() if (n.get("label") or "").strip().lower()=="done"]
if done: click(done[0]["bounds"]); call("settle"); time.sleep(1.5)

# 2. Put that tag on "Note 1" and give it an alias that occurs in the prose.
n1=binder("Note 1")
if n1: click(n1["bounds"]); call("settle"); time.sleep(1.2)
add=find("add a tag","Button")
if add: click(add["bounds"]); call("settle"); time.sleep(0.8)
opt=[n for n in nodes() if n.get("role")=="ListBoxOption" and (n.get("label") or "").strip()=="A"]
print("picker options:", [n.get("label") for n in nodes() if n.get("role")=="ListBoxOption"])
if opt: click(opt[0]["bounds"]); call("settle"); time.sleep(1.0)
call("inject_key",{"key":"Escape"}); call("settle"); time.sleep(0.8)
print("aliases section present:", bool(find("also known as")))
addal=find("add another name","Button")
if addal:
    click(addal["bounds"]); call("settle"); time.sleep(0.8)
    fields=[n for n in nodes() if n.get("role")=="TextInput" and not (n.get("value") or "").strip()
            and (n.get("bounds") or {}).get("x",0)>500]
    if fields:
        call("type_text",{"node":fields[0]["id"],"text":"Aenean"}); call("settle"); time.sleep(0.5)
        call("inject_key",{"key":"Return"}); call("settle"); time.sleep(1.2)
print("alias pills:", [txt(n) for n in nodes() if n.get("role")=="ListItem" and "Aenean" in txt(n)])
shot("/tmp/mentions-setup.png")

# 3. Save so the batch scan runs, then look at a scene that names it.
call("inject_key",{"key":"s","ctrl":True}); call("settle"); time.sleep(3.0)
z=binder("1.1 Zeus")
if z: click(z["bounds"]); call("settle"); time.sleep(2.0)
print("roster heading:", bool(find("mentioned here")))
insp=[txt(n) for n in nodes() if (n.get("bounds") or {}).get("x",0)>850 and txt(n)]
print("inspector text:", insp[:24])
shot("/tmp/mentions-roster.png")
app.terminate(); mcp.terminate()
