import {useEffect,useMemo,useState} from "react";
import {Channel,Handshake,InfoServerClient,ItemDetail,MediaDescriptor,QueueItem} from "./protocol";
import {defaultLanguage,Language,languages,translator} from "./i18n";
import {parseRegistration,RegistrationFile} from "./registration";
function fmt(v?:string){if(!v)return"";const d=new Date(v);return Number.isNaN(d.getTime())?v:d.toLocaleString()}

function MediaPreview({media,client,t}:{media:MediaDescriptor;client:InfoServerClient;t:(k:string)=>string}){
 const[src,setSrc]=useState<string>("");const[error,setError]=useState("");
 useEffect(()=>{let alive=true;setSrc("");setError("");if(!media.preview_available||!media.preview_url)return;client.preview(media.preview_url).then(v=>{if(alive)setSrc(v)}).catch(e=>{if(alive)setError(e instanceof Error?e.message:String(e))});return()=>{alive=false}},[media.preview_available,media.preview_url,client]);
 return <div className="media-card">{src?<img src={src} alt={t("item.media")}/>:<><strong>{media.kind??t("item.media")}</strong><span>{media.policy??""}</span>{error&&<small>{error}</small>}</>}</div>
}

export function App(){
 const[language,setLanguage]=useState<Language>(defaultLanguage);const t=useMemo(()=>translator(language),[language]);
 const[server,setServer]=useState("");const[token,setToken]=useState("");const[hello,setHello]=useState<Handshake|null>(null);
 const[channels,setChannels]=useState<Channel[]>([]);const[queue,setQueue]=useState<QueueItem[]>([]);const[selected,setSelected]=useState<ItemDetail|null>(null);
 const[active,setActive]=useState("*");const[busy,setBusy]=useState(false);const[error,setError]=useState("");const[registration,setRegistration]=useState<RegistrationFile|null>(null);
 const client=useMemo(()=>new InfoServerClient(server.trim(),token.trim()),[server,token]);
 const visible=useMemo(()=>active==="*"?queue:queue.filter(i=>i.channel===active),[queue,active]);

 async function connect(){if(!server.trim()){setError(t("error.no_server"));return}setBusy(true);setError("");try{const[h,ch,q]=await Promise.all([client.handshake(),client.channels(),client.queue(200)]);setHello(h);setChannels(ch);setQueue(q.items);setSelected(q.items[0]?await client.item(q.items[0].id):null)}catch(e){setHello(null);setChannels([]);setQueue([]);setSelected(null);setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function open(i:QueueItem){setBusy(true);setError("");try{setSelected(await client.item(i.id))}catch(e){setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function importRegistration(file:File|undefined){
  if(!file)return;
  setError("");
  try{
   const parsed=await parseRegistration(await file.text());
   setRegistration(parsed);
   setServer(`http://${parsed.server}`);
  }catch(e){
   setRegistration(null);
   setError(e instanceof Error?e.message:String(e));
  }
 }
 function lang(v:Language){setLanguage(v);localStorage.setItem("info-desktop-language",v)}
 const media=selected?.materials?.flatMap(m=>m.media??[])??[];

 return <main className="app-shell">
  <header className="topbar"><div className="brand"><strong>INFO Desktop</strong><span>{t("app.read_only")}</span></div><div className="connection">
    <label className="import-button">{t("registration.import")}<input className="file-input" type="file" accept=".info-registration,.txt,text/plain" onChange={e=>importRegistration(e.target.files?.[0])}/></label><input placeholder={t("server.address")} value={server} onChange={e=>setServer(e.target.value)}/><input placeholder={t("server.token")} type="password" value={token} onChange={e=>setToken(e.target.value)}/><button onClick={connect} disabled={busy}>{busy?t("status.connecting"):t("server.connect")}</button></div><select value={language} onChange={e=>lang(e.target.value as Language)}>{Object.entries(languages).map(([k,v])=><option key={k} value={k}>{v.label}</option>)}</select></header>
  <section className="server-strip"><span className={hello?"dot online":"dot"}/><span>{hello?`${hello.instance_name??hello.server_name??"INFO"} · ${hello.server_version??"?"}`:t("status.disconnected")}</span><span className="queue-count">{t("queue.title")}: {queue.length}</span></section>
  {registration&&<div className="registration-banner"><strong>{t("registration.valid")}</strong><span>{registration.server}:{registration.port}</span><span>{t("registration.expires")}: {fmt(registration.expires_at)}</span><span>{registration.transport}</span></div>}
  {error&&<div className="error-banner">{error}</div>}
  <section className="workspace">
   <aside className="channels-pane"><h2>{t("channels.title")}</h2><button className={active==="*"?"channel active":"channel"} onClick={()=>setActive("*")}><span>{t("channels.all")}</span><strong>{queue.length}</strong></button>{channels.map(c=><button className={active===c.code?"channel active":"channel"} key={c.code} onClick={()=>setActive(c.code)}><span>{c.name}</span><strong>{c.pending??queue.filter(i=>i.channel===c.code).length}</strong></button>)}</aside>
   <section className="queue-pane"><h2>{t("queue.title")}</h2><div className="queue-list">{visible.length===0&&<div className="empty">{t("queue.empty")}</div>}{visible.map(i=><button key={String(i.id)} className={String(selected?.id)===String(i.id)?"queue-row selected":"queue-row"} onClick={()=>open(i)}><span className="row-time">{fmt(i.first_published_at??i.created_at)}</span><span className="row-channel">{i.channel_name??i.channel}</span><span className="row-title">{i.title??t("item.no_title")}</span><span className="row-meta">{i.bucket??""}{typeof i.quality_score==="number"?` · ${i.quality_score.toFixed(1)}`:""}{typeof i.media_count==="number"&&i.media_count>0?` · 📎 ${i.media_count}`:""}</span></button>)}</div></section>
   <article className="detail-pane"><h2>{t("item.title")}</h2>{!selected?<div className="empty">{t("item.select")}</div>:<><h1>{selected.title??t("item.no_title")}</h1><div className="detail-meta"><span>{t("item.channel")}: {selected.channel_name??selected.channel}</span><span>{t("item.language")}: {selected.language??"—"}</span>{selected.bucket&&<span>{selected.bucket}{typeof selected.quality_score==="number"?` · ${selected.quality_score.toFixed(1)}`:""}</span>}</div><div className="article-text">{selected.draft_text??t("item.no_text")}</div>
    {!!selected.materials?.length&&<section><h3>{t("item.source")} · {selected.materials.length}</h3>{selected.materials.map((m,n)=><div className="source-card" key={String(m.id??n)}><strong>{m.source_name??m.source??"—"}</strong><span>{m.title??""}</span><span>{m.url??""}</span><p>{m.translated_summary??m.summary??""}</p></div>)}</section>}
    {!!media.length&&<section className="media-block"><h3>{t("item.media")} · {media.length}</h3><div className="media-grid">{media.map((m,n)=><MediaPreview key={n} media={m} client={client} t={t}/>)}</div></section>}
   </>}</article>
  </section>
 </main>
}
