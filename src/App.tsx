import {useMemo,useState} from "react";
import {Channel,Handshake,InfoServerClient,ItemDetail,QueueItem} from "./protocol";
import {defaultLanguage,Language,languages,translator} from "./i18n";
function fmt(v?:string){if(!v)return"";const d=new Date(v);return Number.isNaN(d.getTime())?v:d.toLocaleString()}
export function App(){
 const[language,setLanguage]=useState<Language>(defaultLanguage);const t=useMemo(()=>translator(language),[language]);
 const[server,setServer]=useState("");const[token,setToken]=useState("");const[hello,setHello]=useState<Handshake|null>(null);
 const[channels,setChannels]=useState<Channel[]>([]);const[queue,setQueue]=useState<QueueItem[]>([]);const[selected,setSelected]=useState<ItemDetail|null>(null);
 const[active,setActive]=useState("*");const[busy,setBusy]=useState(false);const[error,setError]=useState("");
 const visible=useMemo(()=>active==="*"?queue:queue.filter(i=>String(i.channel??"")===active),[queue,active]);
 async function connect(){if(!server.trim()){setError(t("error.no_server"));return}setBusy(true);setError("");try{const c=new InfoServerClient(server.trim(),token.trim());const[h,ch,q]=await Promise.all([c.handshake(),c.channels(),c.queue(200)]);setHello(h);setChannels(ch);setQueue(q.items);setSelected(q.items[0]?await c.item(q.items[0].id):null)}catch(e){setHello(null);setChannels([]);setQueue([]);setSelected(null);setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 async function open(i:QueueItem){setBusy(true);setError("");try{setSelected(await new InfoServerClient(server.trim(),token.trim()).item(i.id))}catch(e){setError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 function lang(v:Language){setLanguage(v);localStorage.setItem("info-desktop-language",v)}
 return <main className="app-shell">
  <header className="topbar"><div className="brand"><strong>INFO Desktop</strong><span>{t("app.read_only")}</span></div>
   <div className="connection"><input placeholder={t("server.address")} value={server} onChange={e=>setServer(e.target.value)}/><input placeholder={t("server.token")} type="password" value={token} onChange={e=>setToken(e.target.value)}/><button onClick={connect} disabled={busy}>{busy?t("status.connecting"):t("server.connect")}</button></div>
   <select value={language} onChange={e=>lang(e.target.value as Language)}>{Object.entries(languages).map(([k,v])=><option key={k} value={k}>{v.label}</option>)}</select>
  </header>
  <section className="server-strip"><span className={hello?"dot online":"dot"}/><span>{hello?`${hello.instance_name??hello.server_name??"INFO"} · ${hello.server_version??"?"}`:t("status.disconnected")}</span><span className="queue-count">{t("queue.title")}: {queue.length}</span></section>
  {error&&<div className="error-banner">{error}</div>}
  <section className="workspace">
   <aside className="channels-pane"><h2>{t("channels.title")}</h2><button className={active==="*"?"channel active":"channel"} onClick={()=>setActive("*")}><span>{t("channels.all")}</span><strong>{queue.length}</strong></button>{channels.map(c=>{const k=String(c.code??c.id??"");const n=c.pending??queue.filter(i=>String(i.channel??"")===k).length;return <button className={active===k?"channel active":"channel"} key={k} onClick={()=>setActive(k)}><span>{c.name??c.code??c.id}</span><strong>{n}</strong></button>})}</aside>
   <section className="queue-pane"><h2>{t("queue.title")}</h2><div className="queue-list">{visible.length===0&&<div className="empty">{t("queue.empty")}</div>}{visible.map(i=><button key={String(i.id)} className={String(selected?.id)===String(i.id)?"queue-row selected":"queue-row"} onClick={()=>open(i)}><span className="row-time">{fmt(i.created_at??i.published_at)}</span><span className="row-channel">{i.channel_name??i.channel??""}</span><span className="row-title">{i.title??t("item.no_title")}</span><span className="row-meta">{i.quality_label??""}{typeof i.media_count==="number"&&i.media_count>0?` · 📎 ${i.media_count}`:""}</span></button>)}</div></section>
   <article className="detail-pane"><h2>{t("item.title")}</h2>{!selected?<div className="empty">{t("item.select")}</div>:<><h1>{selected.title??t("item.no_title")}</h1><div className="detail-meta"><span>{t("item.source")}: {selected.source_name??selected.source??"—"}</span><span>{t("item.channel")}: {selected.channel_name??selected.channel??"—"}</span><span>{t("item.language")}: {selected.language??"—"}</span></div><div className="article-text">{selected.text??selected.prepared_text??selected.original_text??t("item.no_text")}</div>{!!selected.media?.length&&<section className="media-block"><h3>{t("item.media")} · {selected.media.length}</h3><div className="media-grid">{selected.media.map((m,n)=><div className="media-card" key={String(m.id??n)}>{m.preview_url||m.url?<img src={m.preview_url??m.url} alt={`${t("item.media")} ${n+1}`}/>:<span>{m.kind??m.mime_type??t("item.media")}</span>}</div>)}</div></section>}</>}</article>
  </section>
 </main>
}
