import {useEffect,useMemo,useRef,useState} from "react";
import {Channel,Handshake,InfoServerClient,ItemDetail,MediaDescriptor,QueueItem} from "./protocol";
import {defaultLanguage,Language,languages,translator} from "./i18n";
import {activateRegistration,importRegistration,RegistrationSummary,rotateDeviceCredential} from "./registration";

function fmt(v?:string){if(!v)return"";const d=new Date(v);return Number.isNaN(d.getTime())?v:d.toLocaleString()}
function safeHtml(raw:string){
 const doc=new DOMParser().parseFromString(raw||"","text/html");
 const allowed=new Set(["B","STRONG","I","EM","U","S","BLOCKQUOTE","A","BR","CODE","PRE"]);
 const walk=(node:Element)=>{
  [...node.children].forEach(child=>{
   if(!allowed.has(child.tagName)){child.replaceWith(...Array.from(child.childNodes));return}
   [...child.attributes].forEach(a=>{if(child.tagName==="A"&&a.name==="href"){try{const u=new URL(a.value);if(!["http:","https:","tg:"].includes(u.protocol))child.removeAttribute(a.name)}catch{child.removeAttribute(a.name)}}else child.removeAttribute(a.name)});
   if(child.tagName==="A"){child.setAttribute("target","_blank");child.setAttribute("rel","noreferrer")}
   walk(child);
  });
 };
 walk(doc.body);
 return doc.body.innerHTML;
}
function HtmlPreview({html}:{html:string}){return <div className="article-text rendered" dangerouslySetInnerHTML={{__html:safeHtml(html)}}/>}

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
 const editorRef=useRef<HTMLTextAreaElement|null>(null);
 const client=useMemo(()=>new InfoServerClient(server.trim(),deviceId.trim(),serverCaDer),[server,deviceId,serverCaDer]);
 const visible=useMemo(()=>active==="*"?queue:queue.filter(i=>i.channel===active),[queue,active]);
 useEffect(()=>{setEditTitle(selected?.title??"");setEditText(selected?.draft_text??"");setEditChannel(selected?.channel??"")},[selected]);

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
 async function save(){if(!selected)return;setBusy(true);setError("");setNotice("");try{const r=await client.save(selected.id,{title:editTitle,draft_text:editText,channel:editChannel});setNotice("Збережено");await load(r.item_id)}catch(e){setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function createNew(){setBusy(true);setError("");setNotice("");try{const r=await client.create({title:newTitle,draft_text:newText,channel:newChannel,url:newUrl});setCreating(false);setNewTitle("");setNewText("");setNewUrl("");setNotice("Новий матеріал створено");await load(r.item_id)}catch(e){setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 function wrap(openTag:string,closeTag:string=openTag.replace("<","</")){const el=editorRef.current;if(!el)return;const a=el.selectionStart,b=el.selectionEnd;const value=editText.slice(0,a)+openTag+editText.slice(a,b)+closeTag+editText.slice(b);setEditText(value);requestAnimationFrame(()=>{el.focus();el.setSelectionRange(a+openTag.length,b+openTag.length)})}
 async function importRegistrationFile(file:File|undefined){if(!file)return;setError("");try{const parsed=await importRegistration(await file.text());setRegistration(parsed);const serverUrl=`https://${parsed.server}:${parsed.port}`;setServer(serverUrl);setServerCertificateDer(parsed.server_ca_der);localStorage.setItem("info-desktop-server",serverUrl);localStorage.setItem("info-desktop-server-cert",parsed.server_ca_der);const activated=await activateRegistration(parsed,"INFO Desktop");setDeviceId(activated.device_id);setCredentialExpiry(activated.credential_expires_at);localStorage.setItem("info-desktop-device-id",activated.device_id);localStorage.setItem("info-desktop-credential-expiry",activated.credential_expires_at)}catch(e){setRegistration(null);setError(e instanceof Error?e.message:String(e))}}
 function lang(v:Language){setLanguage(v);localStorage.setItem("info-desktop-language",v)}
 const media=selected?.materials?.flatMap(m=>m.media??[])??[];

 return <main className="app-shell">
  <header className="topbar"><div className="brand"><strong>INFO Desktop</strong><span>редактор</span></div><div className="connection">
   <button className="new-button" onClick={()=>setCreating(v=>!v)}>+ Новий матеріал</button>
   <label className="import-button">{t("registration.import")}<input className="file-input" type="file" accept=".registration,.info-registration,.txt,text/plain" onChange={e=>importRegistrationFile(e.target.files?.[0])}/></label>
   <input placeholder={t("server.address")} value={server} onChange={e=>setServer(e.target.value)}/><button onClick={connect} disabled={busy}>{busy?t("status.connecting"):t("server.connect")}</button></div>
   <select value={language} onChange={e=>lang(e.target.value as Language)}>{Object.entries(languages).map(([k,v])=><option key={k} value={k}>{v.label}</option>)}</select></header>
  <section className="server-strip"><span className={hello?"dot online":"dot"}/><span>{hello?`${hello.instance_name??hello.server_name??"INFO"} · ${hello.server_version??"?"}`:t("status.disconnected")}</span><span className="queue-count">Черга: {queue.length}</span></section>
  {registration&&<div className="registration-banner"><strong>Реєстраційний файл дійсний</strong><span>{registration.server}:{registration.port}</span><span>Діє до: {fmt(registration.expires_at)}</span><span>{deviceId?"Ключ пристрою готовий":""}</span></div>}
  {notice&&<div className="notice-banner">{notice}</div>}{error&&<div className="error-banner">{error}</div>}
  {creating&&<section className="create-panel"><h2>Новий матеріал</h2><input placeholder="Заголовок" value={newTitle} onChange={e=>setNewTitle(e.target.value)}/><select value={newChannel} onChange={e=>setNewChannel(e.target.value)}>{channels.map(c=><option key={c.code} value={c.code}>{c.name}</option>)}</select><input placeholder="Посилання на джерело (необов'язково)" value={newUrl} onChange={e=>setNewUrl(e.target.value)}/><textarea placeholder="Текст матеріалу" value={newText} onChange={e=>setNewText(e.target.value)}/><div className="actions"><button onClick={createNew} disabled={busy}>Створити</button><button className="ghost" onClick={()=>setCreating(false)}>Скасувати</button></div></section>}
  <section className="workspace">
   <aside className="channels-pane"><h2>Канали</h2><button className={active==="*"?"channel active":"channel"} onClick={()=>setActive("*")}><span>Усі канали</span><strong>{queue.length}</strong></button>{channels.map(c=><button className={active===c.code?"channel active":"channel"} key={c.code} onClick={()=>setActive(c.code)}><span>{c.name}</span><strong>{c.pending??queue.filter(i=>i.channel===c.code).length}</strong></button>)}</aside>
   <section className="queue-pane"><h2>Черга</h2><div className="queue-list">{visible.map(i=><button key={String(i.id)} className={String(selected?.id)===String(i.id)?"queue-row selected":"queue-row"} onClick={()=>open(i)}><span className="row-time">{fmt(i.first_published_at??i.created_at)}</span><span className="row-channel">{i.channel_name??i.channel}</span><span className="row-title">{i.title}</span><span className="row-meta">{i.bucket??""}{typeof i.quality_score==="number"?` · ${i.quality_score.toFixed(1)}`:""}{i.media_count?` · 📎 ${i.media_count}`:""}</span></button>)}</div></section>
   <article className="detail-pane">{!selected?<div className="empty">Виберіть матеріал</div>:<>
    <div className="editor-head"><h2>Матеріал #{selected.id}</h2><select value={editChannel} onChange={e=>setEditChannel(e.target.value)}>{channels.map(c=><option key={c.code} value={c.code}>{c.name}</option>)}</select></div>
    <input className="title-editor" value={editTitle} onChange={e=>setEditTitle(e.target.value)}/>
    <div className="format-toolbar"><button onClick={()=>wrap("<b>","</b>")}><b>B</b></button><button onClick={()=>wrap("<i>","</i>")}><i>I</i></button><button onClick={()=>wrap("<u>","</u>")}><u>U</u></button><button onClick={()=>wrap("<blockquote>","</blockquote>")}>Цитата</button><button onClick={()=>wrap('<a href="https://">',"</a>")}>Посилання</button></div>
    <textarea ref={editorRef} className="text-editor" value={editText} onChange={e=>setEditText(e.target.value)}/>
    <div className="preview-label">Попередній вигляд</div><HtmlPreview html={editText}/>
    <div className="actions sticky-actions"><button onClick={save} disabled={busy}>Зберегти</button><button className="publish" onClick={()=>act(async()=>{await client.save(selected.id,{title:editTitle,draft_text:editText,channel:editChannel});await client.publish(selected.id)},"Опубліковано")} disabled={busy}>Опублікувати</button><button className="ghost" onClick={()=>act(()=>client.defer(selected.id,60),"Відкладено на годину")} disabled={busy}>+1 година</button><button className="danger" onClick={()=>act(()=>client.skip(selected.id),"Пропущено")} disabled={busy}>Пропустити</button></div>
    <div className="detail-meta"><span>Канал: {selected.channel_name??selected.channel}</span><span>{selected.bucket}{typeof selected.quality_score==="number"?` · ${selected.quality_score.toFixed(1)}`:""}</span></div>
    {!!selected.materials?.length&&<section><h3>Джерела · {selected.materials.length}</h3>{selected.materials.map((m,n)=><div className="source-card" key={String(m.id??n)}><strong>{m.source_name??m.source??"—"}</strong><span>{m.title??""}</span>{m.url&&<a href={m.url} target="_blank" rel="noreferrer">{m.url}</a>}<p>{m.translated_summary??m.summary??""}</p></div>)}</section>}
    {!!media.length&&<section className="media-block"><h3>Медіа · {media.length}</h3><div className="media-grid">{media.map((m,n)=><MediaPreview key={n} media={m} client={client} t={t}/>)}</div></section>}
   </>}</article>
  </section>
 </main>
}