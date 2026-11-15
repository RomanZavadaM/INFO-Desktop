import {useMemo,useState} from "react";
import {ACCESS,Answers,CHANNEL_HINT_JOIN,CHANNEL_HINT_TELEGRAM,INSTANCE,buildSheets,checkAccess,checkChannel,checkInstance,templateChannels,templateDefault} from "./firstSetupSchema";
import {makeXlsx} from "./xlsxWriter";

type T=(k:string)=>string;

function Line({label,hint,example,value,error,onChange,t,password,required,set}:{label:string;hint:string;example:string;value:string;error?:string;onChange:(v:string)=>void;t:T;password?:boolean;required?:boolean;set?:boolean}){
 return <label className={error?"fs-field fs-bad":"fs-field"}>
  <strong>{label}{required?` (${t("firstsetup.required")})`:""}{set?` \u00b7 ${t("firstsetup.set")}`:""}</strong>
  <input type={password?"password":"text"} autoComplete="off" spellCheck={false} value={value} placeholder={example} onChange={e=>onChange(e.target.value)}/>
  <small>{hint}</small>
  <small>{t("firstsetup.example")}: {example}</small>
  {error&&<span className="fs-error">{error}</span>}
 </label>
}

export function FirstSetupPanel({t,onError,onSaved}:{t:T;onError:(m:string)=>void;onSaved:()=>void}){
 const channels=useMemo(()=>templateChannels(),[]);
 const[inst,setInst]=useState<Record<string,string>>(()=>Object.fromEntries(INSTANCE.map(f=>[f.key,templateDefault(f)])));
 const[acc,setAcc]=useState<Record<string,string>>({});
 const[chans,setChans]=useState<Answers["channels"]>(()=>Object.fromEntries(channels.map(c=>[c.code,{title:c.title,telegram:"",join:"",enabled:false,auto:false}])));
 const[errors,setErrors]=useState<Record<string,string>>({});const[saved,setSaved]=useState(false);
 const patch=(code:string,key:string,v:string|boolean)=>setChans(s=>({...s,[code]:{...s[code],[key]:v}}));
 function validate(){
  const e:Record<string,string>={};
  for(const f of INSTANCE){const m=checkInstance(f,inst[f.key]??"");if(m)e["instance."+f.key]=m}
  for(const f of ACCESS){const m=checkAccess(f.key,acc[f.key]??"");if(m)e["access."+f.key]=m}
  for(const c of channels){const v=chans[c.code];const m=checkChannel(v.enabled,v.telegram,v.join,v.auto,c.code);
   if(m.telegram)e[`channels.${c.code}.telegram`]=m.telegram;if(m.join)e[`channels.${c.code}.join`]=m.join;if(m.auto)e[`channels.${c.code}.auto`]=m.auto;
   if(!v.title.trim())e[`channels.${c.code}.title`]=`Канал ${c.code}: потрібна назва`}
  return e;
 }
 function save(){
  setSaved(false);const e=validate();setErrors(e);if(Object.keys(e).length)return;
  try{
   const bytes=makeXlsx(buildSheets({instance:inst,access:acc,channels:chans}));
   const buf=new ArrayBuffer(bytes.length);new Uint8Array(buf).set(bytes);
   const url=URL.createObjectURL(new Blob([buf],{type:"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"}));
   const a=document.createElement("a");a.href=url;a.download="info-first-setup.xlsx";a.click();URL.revokeObjectURL(url);
   setSaved(true);onSaved();
  }catch(err){onError(err instanceof Error?err.message:String(err))}
 }
 return <section className="settings-panel first-setup">
  <h2>{t("firstsetup.title")}</h2><p>{t("firstsetup.intro")}</p>
  {Object.keys(errors).length>0&&<div className="error-banner">{t("firstsetup.fix")}</div>}
  <h3>{t("firstsetup.instance")}</h3>
  {INSTANCE.map(f=><Line key={f.key} label={f.label} hint={f.hint} example={f.example} required={f.required} value={inst[f.key]??""} error={errors["instance."+f.key]} onChange={v=>setInst(s=>({...s,[f.key]:v}))} t={t}/>)}
  <h3>{t("firstsetup.access")}</h3>
  {ACCESS.map(f=><Line key={f.key} label={f.label} hint={f.hint} example={f.example} value={acc[f.key]??""} error={errors["access."+f.key]} onChange={v=>setAcc(s=>({...s,[f.key]:v}))} t={t} password={f.secret}/>)}
  <small>{t("firstsetup.keep")}</small>
  <h3>{t("firstsetup.channels")}</h3>
  <small>{CHANNEL_HINT_TELEGRAM}</small>
  {channels.map(c=>{const v=chans[c.code];return <div className="fs-channel" key={c.code}>
   <strong>{c.code}</strong>
   <label>{t("firstsetup.name")}<input value={v.title} onChange={e=>patch(c.code,"title",e.target.value)}/></label>
   <label className={errors[`channels.${c.code}.telegram`]?"fs-bad":""}>{t("firstsetup.telegram")}<input value={v.telegram} placeholder="@my_channel" onChange={e=>patch(c.code,"telegram",e.target.value)}/></label>
   <label className={errors[`channels.${c.code}.join`]?"fs-bad":""}>{t("firstsetup.join")}<input value={v.join} placeholder="https://t.me/my_channel" title={CHANNEL_HINT_JOIN} onChange={e=>patch(c.code,"join",e.target.value)}/></label>
   <label><input type="checkbox" checked={v.enabled} onChange={e=>patch(c.code,"enabled",e.target.checked)}/> {t("firstsetup.enabled")}</label>
   <label><input type="checkbox" checked={v.auto} disabled={!v.enabled} onChange={e=>patch(c.code,"auto",e.target.checked)}/> {t("firstsetup.auto")}</label>
   {["telegram","join","title","auto"].map(k=>errors[`channels.${c.code}.${k}`]&&<span className="fs-error" key={k}>{errors[`channels.${c.code}.${k}`]}</span>)}
  </div>})}
  <div className="settings-actions"><button className="publish" onClick={save}>{t("firstsetup.submit")}</button></div>
  {saved&&<div className="notice-banner">{t("firstsetup.ready")}</div>}
 </section>
}
