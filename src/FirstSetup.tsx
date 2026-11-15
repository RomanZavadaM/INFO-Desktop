import {useEffect,useState} from "react";
import {FirstSetupAnswers,FirstSetupField,FirstSetupForm,InfoServerClient} from "./protocol";

type T=(k:string)=>string;

function Field({f,value,error,onChange,t,password}:{f:FirstSetupField;value:string;error?:string;onChange:(v:string)=>void;t:T;password?:boolean}){
 return <label className={error?"fs-field fs-bad":"fs-field"}>
  <strong>{f.label}{f.required?` (${t("firstsetup.required")})`:""}{f.set!==undefined?` \u00b7 ${f.set?t("firstsetup.set"):t("firstsetup.unset")}`:""}</strong>
  <input type={password?"password":"text"} autoComplete="off" spellCheck={false} maxLength={f.max} value={value} placeholder={f.example} onChange={e=>onChange(e.target.value)}/>
  <small>{f.hint}</small>
  <small>{t("firstsetup.example")}: {f.example}{f.set?` \u00b7 ${t("firstsetup.keep")}`:""}</small>
  {error&&<span className="fs-error">{error}</span>}
 </label>
}

export function FirstSetupPanel({client,t,onError,onReady}:{client:InfoServerClient;t:T;onError:(m:string)=>void;onReady:()=>void}){
 const[form,setForm]=useState<FirstSetupForm|null>(null);const[unavailable,setUnavailable]=useState(false);
 const[inst,setInst]=useState<Record<string,string>>({});const[acc,setAcc]=useState<Record<string,string>>({});
 const[chans,setChans]=useState<Record<string,{title:string;telegram:string;join_url:string;enabled:boolean;auto:boolean}>>({});
 const[errors,setErrors]=useState<Record<string,string>>({});const[busy,setBusy]=useState(false);const[ready,setReady]=useState("");
 useEffect(()=>{let alive=true;(async()=>{try{const f=await client.firstSetupForm();if(!alive)return;setForm(f);
  setInst(Object.fromEntries(f.instance.map(x=>[x.key,x.default??""])));
  setChans(Object.fromEntries(f.channels.map(c=>[c.code,{title:c.title,telegram:"",join_url:"",enabled:false,auto:false}])))}
  catch(e){const m=e instanceof Error?e.message:String(e);if(alive){if(m.includes("409"))setUnavailable(true);else onError(m)}}})();return()=>{alive=false}},[client,onError]);
 async function submit(){setBusy(true);setReady("");setErrors({});try{
  const answers:FirstSetupAnswers={instance:inst,access:acc,channels:Object.entries(chans).map(([code,c])=>({code,...c}))};
  const r=await client.firstSetupSubmit(answers);
  if(r.ok){setReady(r.summary);onReady()}else setErrors(r.errors)
 }catch(e){onError(e instanceof Error?e.message:String(e))}finally{setBusy(false)}}
 if(unavailable)return <section className="settings-panel"><h2>{t("firstsetup.title")}</h2><div className="warning-banner">{t("firstsetup.unavailable")}</div></section>;
 if(!form)return <section className="settings-panel"><h2>{t("firstsetup.title")}</h2></section>;
 const patch=(code:string,key:string,v:string|boolean)=>setChans(s=>({...s,[code]:{...s[code],[key]:v}}));
 return <section className="settings-panel first-setup">
  <h2>{t("firstsetup.title")}</h2><p>{t("firstsetup.intro")}</p><small>{form.note}</small>
  {Object.keys(errors).length>0&&<div className="error-banner">{t("firstsetup.fix")}</div>}
  <h3>{t("firstsetup.instance")}</h3>
  {form.instance.map(f=><Field key={f.key} f={f} value={inst[f.key]??""} error={errors["instance."+f.key]} onChange={v=>setInst(s=>({...s,[f.key]:v}))} t={t}/>)}
  <h3>{t("firstsetup.access")}</h3>
  {form.access.map(f=><Field key={f.key} f={f} value={acc[f.key]??""} error={errors["access."+f.key]} onChange={v=>setAcc(s=>({...s,[f.key]:v}))} t={t} password={f.secret}/>)}
  <h3>{t("firstsetup.channels")}</h3>
  <small>{form.channel_hints.telegram}</small>
  {form.channels.map(c=>{const v=chans[c.code];if(!v)return null;return <div className="fs-channel" key={c.code}>
   <strong>{c.code}</strong>
   <label>{t("firstsetup.name")}<input value={v.title} onChange={e=>patch(c.code,"title",e.target.value)}/></label>
   <label className={errors[`channels.${c.code}.telegram`]?"fs-bad":""}>{t("firstsetup.telegram")}<input value={v.telegram} placeholder="@my_channel" onChange={e=>patch(c.code,"telegram",e.target.value)}/></label>
   <label className={errors[`channels.${c.code}.join_url`]?"fs-bad":""}>{t("firstsetup.join")}<input value={v.join_url} placeholder="https://t.me/my_channel" onChange={e=>patch(c.code,"join_url",e.target.value)}/></label>
   <label><input type="checkbox" checked={v.enabled} onChange={e=>patch(c.code,"enabled",e.target.checked)}/> {t("firstsetup.enabled")}</label>
   <label><input type="checkbox" checked={v.auto} disabled={!v.enabled} onChange={e=>patch(c.code,"auto",e.target.checked)}/> {t("firstsetup.auto")}</label>
   {["telegram","join_url","title","auto"].map(k=>errors[`channels.${c.code}.${k}`]&&<span className="fs-error" key={k}>{errors[`channels.${c.code}.${k}`]}</span>)}
  </div>})}
  <div className="settings-actions"><button className="publish" onClick={submit} disabled={busy}>{t("firstsetup.submit")}</button></div>
  {ready&&<><div className="notice-banner">{t("firstsetup.ready")}</div><pre className="why-lines">{ready}</pre></>}
 </section>
}
