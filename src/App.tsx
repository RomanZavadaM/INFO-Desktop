import {useEffect,useMemo,useRef,useState} from "react";
import {Channel,Handshake,InfoServerClient,ItemDetail,MediaDescriptor,ProbeStatus,QueueItem,SettingsStatus,Theme,Why} from "./protocol";
import {defaultLanguage,Language,languages,translator} from "./i18n";
import pkg from "../package.json";
import {activateRegistration,importRegistration,RegistrationSummary,rotateDeviceCredential} from "./registration";

function fmt(v?:string){if(!v)return"";const d=new Date(v);return Number.isNaN(d.getTime())?v:d.toLocaleString()}
function safeHtml(raw:string){
 const doc=new DOMParser().parseFromString(raw||"","text/html");
 const allowed=new Set(["B","STRONG","I","EM","U","S","BLOCKQUOTE","A","BR","CODE","PRE"]);
 const walk=(node:Element)=>{
  Array.from(node.children).forEach(child=>{
   const tag=child.tagName;
   if(["SCRIPT","STYLE","SVG","MATH","IFRAME","OBJECT","EMBED","IMG","FORM"].includes(tag)){child.remove();return}
   walk(child);
   if(!allowed.has(tag)){child.replaceWith(...Array.from(child.childNodes));return}
   Array.from(child.attributes).forEach(a=>{if(tag==="A"&&a.name==="href"){try{const u=new URL(a.value);if(!["http:","https:","tg:"].includes(u.protocol))child.removeAttribute(a.name)}catch{child.removeAttribute(a.name)}}else child.removeAttribute(a.name)});
   if(tag==="A"){child.setAttribute("target","_blank");child.setAttribute("rel","noreferrer")}
  });
 };
 walk(doc.body);
 return doc.body.innerHTML;
}

/** The server retains Telegram-compatible HTML; the editor renders it while editing. */
function VisualTextEditor({html,onChange,t}:{html:string;onChange:(value:string)=>void;t:(key:string)=>string}){
 const area=useRef<HTMLDivElement|null>(null);
 const[showHtml,setShowHtml]=useState(false);
 const last=useRef<string>("");
 useEffect(()=>{
  if(showHtml||!area.current)return;
  if(document.activeElement!==area.current && area.current.innerHTML!==safeHtml(html)){
   area.current.innerHTML=safeHtml(html);
   last.current=html;
  }
 },[html,showHtml]);
 function serialize(){
  if(!area.current)return;
  const clone=area.current.cloneNode(true) as HTMLElement;
  clone.querySelectorAll("div,p").forEach(n=>{
   const br=document.createElement("br");
   n.parentNode?.insertBefore(br,n);
   n.replaceWith(...Array.from(n.childNodes));
  });
  const value=safeHtml(clone.innerHTML).replace(/^(<br>)+/,"");
  last.current=value;onChange(value);
 }
 function format(command:string){
  area.current?.focus();
  document.execCommand(command,false);
  serialize();
 }
 function insertLink(){
  const url=window.prompt(t("editor.link_prompt"),"https://");
  if(!url)return;
  try{const u=new URL(url);if(!["https:","http:"].includes(u.protocol))return}catch{return}
  area.current?.focus();document.execCommand("createLink",false,url);serialize();
 }
 return <div className="visual-editor-shell">
  <div className="format-toolbar" role="toolbar" aria-label={t("editor.toolbar")}>
   <button type="button" onMouseDown={e=>e.preventDefault()} onClick={()=>format("bold")} aria-label="Bold"><b>B</b></button>
   <button type="button" onMouseDown={e=>e.preventDefault()} onClick={()=>format("italic")} aria-label="Italic"><i>I</i></button>
   <button type="button" onMouseDown={e=>e.preventDefault()} onClick={()=>format("underline")} aria-label="Underline"><u>U</u></button>
   <button type="button" onMouseDown={e=>e.preventDefault()} onClick={()=>{area.current?.focus();document.execCommand("formatBlock",false,"blockquote");serialize()}}>{t("format.quote")}</button>
   <button type="button" onMouseDown={e=>e.preventDefault()} onClick={insertLink}>{t("format.link")}</button>
   <button type="button" className="mode-switch" onClick={()=>{if(!showHtml)serialize();setShowHtml(!showHtml)}} aria-pressed={showHtml}>{showHtml?t("editor.visual"):t("editor.html")}</button>
  </div>
  {showHtml
   ?<textarea className="text-editor" value={html} onChange={e=>{last.current=e.target.value;onChange(e.target.value)}} aria-label={t("editor.html")}/>
   :<div ref={area} className="visual-text-editor" contentEditable suppressContentEditableWarning
       role="textbox" aria-label={t("editor.visual")} aria-multiline="true"
       onInput={serialize} onBlur={serialize} onPaste={e=>{e.preventDefault();document.execCommand("insertText",false,e.clipboardData.getData("text/plain"))}}/>}
 </div>;
}

function HtmlPreview({html}:{html:string}){return <div className="article-text rendered" dangerouslySetInnerHTML={{__html:safeHtml(html)}}/>}

function TelegramPreview({title,html,media,t}:{title:string;html:string;media:MediaDescriptor[];t:(k:string)=>string}){
 const hasMedia=media.some(m=>m.preview_available);
 return <div className="platform-preview">
  <div className="platform-tabs"><button className="active">Telegram</button><button disabled>TikTok</button><button disabled>Facebook</button></div>
  <div className="telegram-stage">
   <div className="telegram-bubble">
    {hasMedia&&<div className="telegram-media-placeholder">{t("preview.media_from_item")}</div>}
    <div className="telegram-channel">З-під Ляди</div>
    {title&&<div className="telegram-title">{title}</div>}
    <div className="telegram-body" dangerouslySetInnerHTML={{__html:safeHtml(html)}}/>
    <div className="telegram-time">18:14 ✓✓</div>
   </div>
  </div>
 </div>
}

function WhyPanel({why,t}:{why:Why;t:(k:string)=>string}){
 return <section className="why-panel">
  <h3>{t("why.title")} #{why.event_id}</h3>
  <div className="why-channels">{why.channels.map(c=><div key={c.code} className={c.ok?"why-row yes":"why-row no"}><strong>{c.ok?"✓":"✗"} {c.code}</strong><span>{c.reason}</span></div>)}</div>
  {why.rules&&<><h3>{t("why.rules")}: {why.rules.channel}</h3>
   {why.rules.has_rules?<pre className="why-lines">{why.rules.lines.join("\n")}</pre>:<div className="empty">{t("why.no_rules")}</div>}
   {why.rules.truncated&&<small>{t("why.truncated")}</small>}</>}
 </section>
}

function toBase64(buf:ArrayBuffer){const bytes=new Uint8Array(buf);let s="";for(let i=0;i<bytes.length;i+=0x8000)s+=String.fromCharCode(...bytes.subarray(i,i+0x8000));return btoa(s)}
function fromBase64(b64:string){const s=atob(b64);const out=new Uint8Array(s.length);for(let i=0;i<s.length;i++)out[i]=s.charCodeAt(i);return out}

function SettingsPanel({client,t,onError}:{client:InfoServerClient;t:(k:string)=>string;onError:(m:string)=>void}){
 const[probe,setProbe]=useState<ProbeStatus>({state:"none"});
 const[status,setStatus]=useState<SettingsStatus|null>(null);const[summary,setSummary]=useState("");const[busy,setBusy]=useState(false);const[message,setMessage]=useState("");
 async function refresh(){try{setStatus(await client.settingsStatus())}catch(e){onError(e instanceof Error?e.message:String(e))}}
 useEffect(()=>{refresh()},[client]);
 useEffect(()=>{let alive=true;async function poll(){try{const p=await client.settingsProbeStatus();if(alive)setProbe(p)}catch{}}poll();
  const timer=window.setInterval(()=>{if(probe.state==="queued"||probe.state==="running")poll()},5000);return()=>{alive=false;window.clearInterval(timer)}},[client,probe.state]);
 async function runProbe(){await run(async()=>{await client.settingsProbe(48);setProbe({state:"queued"})})}
 async function run(fn:()=>Promise<void>){setBusy(true);setMessage("");onError("");try{await fn();await refresh()}catch(e){onError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function download(){await run(async()=>{const x=await client.settingsExport();const url=URL.createObjectURL(new Blob([fromBase64(x.base64)],{type:"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"}));const a=document.createElement("a");a.href=url;a.download=x.filename;a.click();URL.revokeObjectURL(url);setMessage(t("settings.exported"))})}
 async function upload(file:File|undefined){if(!file)return;await run(async()=>{const r=await client.settingsCheck(toBase64(await file.arrayBuffer()));setProbe({state:"none"});setSummary(`${file.name}\n${r.summary}`);setMessage(t("settings.checked"))})}
 async function apply(){if(!status?.pending_digest){onError(t("settings.digest_missing"));return}if(!(probe.state==="done"&&probe.plan_digest===status.pending_digest)&&!window.confirm(t("settings.confirm_unprobed")))return;if(!window.confirm(t("settings.confirm_apply")))return;await run(async()=>{await client.settingsApply(status.pending_digest);setProbe({state:"none"});setSummary("");setMessage(t("settings.applied"))})}
 async function rollback(){if(!window.confirm(t("settings.confirm_rollback")))return;await run(async()=>{await client.settingsRollback();setMessage(t("settings.rolled_back"))})}
 return <section className="settings-panel">
  <h2>{t("settings.title")}</h2>
  <div className="warning-banner">{t("settings.warning")}</div>
  <p>{t("settings.intro")}</p>
  <div className="settings-actions"><button onClick={download} disabled={busy}>{t("settings.download")}</button>
   <label className="import-button">{t("settings.choose")}<input className="file-input" type="file" accept=".xlsx,application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" disabled={busy} onChange={e=>{upload(e.target.files?.[0]);e.target.value=""}}/></label></div>
  {summary&&<pre className="why-lines">{summary}</pre>}
  <div className="settings-actions"><button onClick={runProbe} disabled={busy||!status?.has_pending||probe.state==="queued"||probe.state==="running"}>{t("settings.probe")}</button>
   {(probe.state==="queued"||probe.state==="running")&&<span>{t("settings.probe_running")}</span>}</div>
  {(probe.state==="done"||probe.state==="error")&&!!probe.text&&<><small>{probe.state==="done"&&status?.pending_digest===probe.plan_digest?t("settings.probe_valid"):t("settings.probe_stale")}</small><pre className="why-lines">{probe.text}</pre></>}
  <div className="settings-actions"><button className="publish" onClick={apply} disabled={busy||!status?.has_pending||!status?.pending_digest||probe.state==="queued"||probe.state==="running"}>{t("settings.apply")}</button>
   <button onClick={rollback} disabled={busy||!status?.can_rollback}>{t("settings.rollback")}</button></div>
  {message&&<div className="notice-banner">{message}</div>}
  <small>{t("settings.probe_hint")}</small>
 </section>
}

function ThemesPanel({client,t,onError}:{client:InfoServerClient;t:(k:string)=>string;onError:(m:string)=>void}){
 const[themes,setThemes]=useState<Theme[]>([]);const[code,setCode]=useState("");const[text,setText]=useState("");const[sample,setSample]=useState("");const[matched,setMatched]=useState<string[]|null>(null);const[busy,setBusy]=useState(false);const[message,setMessage]=useState("");
 const current=themes.find(x=>x.code===code);
 async function load(keep?:string){try{const r=await client.themes();setThemes(r.themes);const c=keep&&r.themes.some(x=>x.code===keep)?keep:r.themes[0]?.code??"";setCode(c);setText((r.themes.find(x=>x.code===c)?.words??[]).join("\n"))}catch(e){onError(e instanceof Error?e.message:String(e))}}
 useEffect(()=>{load()},[client]);
 function pick(c:string){setCode(c);setText((themes.find(x=>x.code===c)?.words??[]).join("\n"));setMatched(null);setMessage("")}
 const words=()=>text.split("\n").map(w=>w.trim()).filter(Boolean);
 async function run(fn:()=>Promise<void>){setBusy(true);onError("");setMessage("");try{await fn()}catch(e){onError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function check(){await run(async()=>{const r=await client.themesTest(words(),sample);setMatched(r.matched)})}
 async function save(){if(!window.confirm(t("themes.confirm_save")))return;await run(async()=>{const r=await client.themesSave(code,words());setMessage(`${t("themes.saved")}: ${r.words}`);await load(code)})}
 return <section className="settings-panel">
  <h2>{t("themes.title")}</h2><div className="warning-banner">{t("themes.warning")}</div><p>{t("themes.intro")}</p>
  <div className="settings-actions"><select value={code} onChange={e=>pick(e.target.value)}>{themes.map(x=><option key={x.code} value={x.code}>{x.title||x.code} ({x.words.length})</option>)}</select></div>
  <textarea className="text-editor" value={text} onChange={e=>setText(e.target.value)} placeholder={t("themes.words_hint")}/>
  <small>{t("themes.syntax")}</small>
  <h3>{t("themes.test")}</h3>
  <textarea className="text-editor" value={sample} onChange={e=>setSample(e.target.value)} placeholder={t("themes.sample_hint")}/>
  <div className="settings-actions"><button onClick={check} disabled={busy||!sample.trim()}>{t("themes.check")}</button>
   <button className="publish" onClick={save} disabled={busy||!current}>{t("themes.save")}</button></div>
  {matched&&<div className={matched.length?"notice-banner":"error-banner"}>{matched.length?`${t("themes.matched")}: ${matched.join(", ")}`:t("themes.none")}</div>}
  {message&&<div className="notice-banner">{message}</div>}
  <small>{t("themes.rollback_hint")}</small>
 </section>
}

function MediaPreview({media,client,t}:{media:MediaDescriptor;client:InfoServerClient;t:(k:string)=>string}){
 const[src,setSrc]=useState("");const[error,setError]=useState("");
 useEffect(()=>{let alive=true;setSrc("");setError("");if(!media.preview_available||!media.preview_url)return;client.preview(media.preview_url).then(v=>{if(alive)setSrc(v)}).catch(e=>{if(alive)setError(e instanceof Error?e.message:String(e))});return()=>{alive=false}},[media.preview_available,media.preview_url,client]);
 return <div className="media-card">{src?<img src={src} alt={t("item.media")}/>:<><strong>{media.kind??t("item.media")}</strong><span>{media.policy??""}</span>{error&&<small>{error}</small>}</>}</div>
}

export function App(){
 const[language,setLanguage]=useState<Language>(defaultLanguage);const t=useMemo(()=>translator(language),[language]);
 const[server,setServer]=useState(()=>localStorage.getItem("info-desktop-server")??"");
 const[deviceId,setDeviceId]=useState(()=>localStorage.getItem("info-desktop-device-id")??"");
 const[credentialExpiry,setCredentialExpiry]=useState(()=>localStorage.getItem("info-desktop-credential-expiry")??"");
 const[serverCaDer,setServerCertificateDer]=useState(()=>localStorage.getItem("info-desktop-server-cert")??"");
 const[hello,setHello]=useState<Handshake|null>(null);const[channels,setChannels]=useState<Channel[]>([]);const[queue,setQueue]=useState<QueueItem[]>([]);const[selected,setSelected]=useState<ItemDetail|null>(null);
 const[active,setActive]=useState("*");const[busy,setBusy]=useState(false);const[error,setError]=useState("");const[notice,setNotice]=useState("");const[registration,setRegistration]=useState<RegistrationSummary|null>(null);
 const[editTitle,setEditTitle]=useState("");const[editText,setEditText]=useState("");const[editChannel,setEditChannel]=useState("");
 const[creating,setCreating]=useState(false);const[newTitle,setNewTitle]=useState("");const[newText,setNewText]=useState("");const[newChannel,setNewChannel]=useState("");const[newUrl,setNewUrl]=useState("");
 const[why,setWhy]=useState<Why|null>(null);const[view,setView]=useState<"queue"|"settings"|"themes">("queue");
 const client=useMemo(()=>new InfoServerClient(server.trim(),deviceId.trim(),serverCaDer),[server,deviceId,serverCaDer]);
 const visible=useMemo(()=>active==="*"?queue:queue.filter(i=>i.channel===active),[queue,active]);
 useEffect(()=>{setWhy(null);setEditTitle(selected?.title??"");setEditText(selected?.draft_text??"");setEditChannel(selected?.channel??"")},[selected]);

 async function load(prefer?:string|number){
  const[h,ch,q]=await Promise.all([client.handshake(),client.channels(),client.queueAll()]);
  setHello(h);setChannels(ch);setQueue(q.items);
  const target=(prefer!==undefined?q.items.find(x=>String(x.id)===String(prefer)):q.items[0])??q.items[0];
  setSelected(target?await client.item(target.id):null);
  if(!newChannel&&ch[0])setNewChannel(ch[0].code);
 }
 async function connect(){if(!server.trim()){setError(t("error.no_server"));return}setBusy(true);setError("");setNotice("");try{
   if(deviceId&&credentialExpiry){const exp=new Date(credentialExpiry).getTime();if(Number.isFinite(exp)&&exp<=Date.now()+7*86400000){const rotated=await rotateDeviceCredential(server.trim(),deviceId,serverCaDer);setCredentialExpiry(rotated.credential_expires_at);localStorage.setItem("info-desktop-credential-expiry",rotated.credential_expires_at)}}
   await load();
  }catch(e){setHello(null);setChannels([]);setQueue([]);setSelected(null);setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function open(i:QueueItem){setBusy(true);setError("");try{setSelected(await client.item(i.id))}catch(e){setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function act(fn:()=>Promise<unknown>,message:string){setBusy(true);setError("");setNotice("");try{await fn();setNotice(message);await load()}catch(e){setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function loadWhy(){if(!selected)return;setBusy(true);setError("");try{setWhy(await client.why(selected.id,editChannel||selected.channel))}catch(e){setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function save(){if(!selected)return;setBusy(true);setError("");setNotice("");try{const r=await client.save(selected.id,{title:editTitle,draft_text:editText,channel:editChannel});setNotice(t("notice.saved"));await load(r.item_id)}catch(e){setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function createNew(){setBusy(true);setError("");setNotice("");try{const r=await client.create({title:newTitle,draft_text:newText,channel:newChannel,url:newUrl});setCreating(false);setNewTitle("");setNewText("");setNewUrl("");setNotice(t("notice.created"));await load(r.item_id)}catch(e){setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function importRegistrationFile(file:File|undefined){if(!file)return;setError("");try{const parsed=await importRegistration(await file.text());setRegistration(parsed);const serverUrl=`https://${parsed.server}:${parsed.port}`;setServer(serverUrl);setServerCertificateDer(parsed.server_ca_der);localStorage.setItem("info-desktop-server",serverUrl);localStorage.setItem("info-desktop-server-cert",parsed.server_ca_der);const activated=await activateRegistration(parsed,"INFO Desktop");setDeviceId(activated.device_id);setCredentialExpiry(activated.credential_expires_at);localStorage.setItem("info-desktop-device-id",activated.device_id);localStorage.setItem("info-desktop-credential-expiry",activated.credential_expires_at)}catch(e){setRegistration(null);setError(e instanceof Error?e.message:String(e))}}
 function lang(v:Language){setLanguage(v);localStorage.setItem("info-desktop-language",v)}
 const media=selected?.materials?.flatMap(m=>m.media??[])??[];

 return <main className="app-shell">
  <header className="topbar"><div className="brand"><strong>INFO Desktop</strong><span>{t("app.editor")}</span><small className="app-version">v{pkg.version}</small></div><div className="connection">
   <button className="new-button" onClick={()=>setCreating(v=>!v)}>+ {t("create.new")}</button>
   <button onClick={()=>setView(v=>v==="settings"?"queue":"settings")} disabled={!hello}>{view==="settings"?t("settings.back"):t("settings.open")}</button>
   <button onClick={()=>setView(v=>v==="themes"?"queue":"themes")} disabled={!hello}>{view==="themes"?t("settings.back"):t("themes.open")}</button>
   <label className="import-button">{t("registration.import")}<input className="file-input" type="file" accept=".registration,.info-registration,.txt,text/plain" onChange={e=>importRegistrationFile(e.target.files?.[0])}/></label>
   <input placeholder={t("server.address")} value={server} onChange={e=>setServer(e.target.value)}/><button onClick={connect} disabled={busy}>{busy?t("status.connecting"):t("server.connect")}</button></div>
   <select value={language} onChange={e=>lang(e.target.value as Language)}>{Object.entries(languages).map(([k,v])=><option key={k} value={k}>{v.label}</option>)}</select></header>
  <section className="server-strip"><span className={hello?"dot online":"dot"}/><span>{hello?`${hello.instance_name??hello.server_name??"INFO"} · ${hello.server_version??"?"}`:t("status.disconnected")}</span><span className="queue-count">{t("queue.title")}: {queue.length}</span></section>
  {registration&&<div className="registration-banner"><strong>{t("registration.valid")}</strong><span>{registration.server}:{registration.port}</span><span>{t("registration.expires")}: {fmt(registration.expires_at)}</span><span>{deviceId?t("registration.device_ready"):""}</span></div>}
  {notice&&<div className="notice-banner">{notice}</div>}{error&&<div className="error-banner">{error}</div>}
  {creating&&<section className="create-panel"><h2>{t("create.new")}</h2><input placeholder={t("create.title")} value={newTitle} onChange={e=>setNewTitle(e.target.value)}/><select value={newChannel} onChange={e=>setNewChannel(e.target.value)}>{channels.map(c=><option key={c.code} value={c.code}>{c.name}</option>)}</select><input placeholder={t("create.source_url")} value={newUrl} onChange={e=>setNewUrl(e.target.value)}/><textarea placeholder={t("create.text")} value={newText} onChange={e=>setNewText(e.target.value)}/><div className="actions"><button onClick={createNew} disabled={busy}>{t("create.submit")}</button><button className="ghost" onClick={()=>setCreating(false)}>{t("action.cancel")}</button></div></section>}
  {view==="settings"&&hello&&<SettingsPanel client={client} t={t} onError={setError}/>}
  {view==="themes"&&hello&&<ThemesPanel client={client} t={t} onError={setError}/>}
  {view==="queue"&&<section className="workspace">
   <aside className="channels-pane"><h2>{t("channels.title")}</h2><button className={active==="*"?"channel active":"channel"} onClick={()=>setActive("*")}><span>{t("channels.all")}</span><strong>{queue.length}</strong></button>{channels.map(c=><button className={active===c.code?"channel active":"channel"} key={c.code} onClick={()=>setActive(c.code)}><span>{c.name}</span><strong>{c.pending??queue.filter(i=>i.channel===c.code).length}</strong></button>)}</aside>
   <section className="queue-pane"><h2>{t("queue.title")}</h2><div className="queue-list">{visible.map(i=><button key={String(i.id)} className={String(selected?.id)===String(i.id)?"queue-row selected":"queue-row"} onClick={()=>open(i)}><span className="row-time">{fmt(i.first_published_at??i.created_at)}</span><span className="row-channel">{i.channel_name??i.channel}</span><span className="row-title">{i.title}</span><span className="row-meta">{i.bucket??""}{typeof i.quality_score==="number"?` · ${i.quality_score.toFixed(1)}`:""}{i.media_count?` · 📎 ${i.media_count}`:""}</span></button>)}</div></section>
   <article className="detail-pane">{!selected?<div className="empty">{t("item.select")}</div>:<>
    <div className="editor-head"><h2>{t("item.title")} #{selected.id}</h2><select value={editChannel} onChange={e=>setEditChannel(e.target.value)}>{channels.map(c=><option key={c.code} value={c.code}>{c.name}</option>)}</select></div>
    <input className="title-editor" value={editTitle} onChange={e=>setEditTitle(e.target.value)}/>
    <VisualTextEditor key={String(selected.id)} html={editText} onChange={setEditText} t={t}/>
    <div className="preview-label">{t("preview.telegram")}</div><TelegramPreview title={editTitle} html={editText} media={media} t={t}/>
    <div className="actions sticky-actions"><button onClick={save} disabled={busy}>{t("action.save")}</button><button className="publish" onClick={()=>act(async()=>{const saved=await client.save(selected.id,{title:editTitle,draft_text:editText,channel:editChannel});await client.publish(saved.item_id)},t("notice.published"))} disabled={busy}>{t("action.publish")}</button><button className="ghost" onClick={()=>act(()=>client.defer(selected.id,60),t("notice.deferred"))} disabled={busy}>{t("action.defer_hour")}</button><button className="danger" onClick={()=>act(()=>client.skip(selected.id),t("notice.skipped"))} disabled={busy}>{t("action.skip")}</button></div>
    <div className="why-block"><button onClick={loadWhy} disabled={busy}>{t("why.button")}</button></div>{why&&<WhyPanel why={why} t={t}/>}
    <div className="detail-meta"><span>{t("item.channel")}: {selected.channel_name??selected.channel}</span><span>{selected.bucket}{typeof selected.quality_score==="number"?` · ${selected.quality_score.toFixed(1)}`:""}</span></div>
    {!!selected.materials?.length&&<section><h3>{t("sources.title")} · {selected.materials.length}</h3>{selected.materials.map((m,n)=><div className="source-card" key={String(m.id??n)}><strong>{m.source_name??m.source??"—"}</strong><span>{m.title??""}</span>{m.url&&<a href={m.url} target="_blank" rel="noreferrer">{m.url}</a>}<p>{m.translated_summary??m.summary??""}</p></div>)}</section>}
    {!!media.length&&<section className="media-block"><h3>{t("item.media")} · {media.length}</h3><div className="media-grid">{media.map((m,n)=><MediaPreview key={n} media={m} client={client} t={t}/>)}</div></section>}
   </>}</article>
  </section>}
 </main>
}