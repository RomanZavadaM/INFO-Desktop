import { invoke } from "@tauri-apps/api/core";

export const PROTOCOL_ID="info-desktop-protocol/1";

export interface Handshake{protocol:string;server_name?:string;instance_name?:string;server_version?:string;timezone?:string;capabilities?:string[];languages?:string[]}
export interface Channel{code:string;name:string;language?:string;auto_publish?:boolean;pending?:number}
export type EditorialBucket="review"|"quick"|"weak"|"auto"|string;
export interface QueueItem{id:string|number;event_id?:string|number;channel:string;channel_name?:string;title?:string;created_at?:string;first_published_at?:string;state?:string;bucket?:EditorialBucket;quality_score?:number;auto_ok?:boolean;issues?:string[];language?:string;media_count?:number}
export interface QueuePage{items:QueueItem[];next_cursor?:string|null;counts?:Record<string,number>}

export interface MediaDescriptor{
  kind?:string;
  mime_type?:string;
  origin_url?:string;
  cached?:boolean;
  preview_available?:boolean;
  original_available?:boolean;
  usage?:string;
  policy?:"temporary_cache"|"external_only"|string;
  preview_url?:string;
  width?:number;
  height?:number;
  size?:number;
}
export interface SourceMaterial{id?:string|number;source?:string;source_name?:string;url?:string;title?:string;summary?:string;published_at?:string;original_title?:string;original_summary?:string;translation_state?:string;translated_title?:string;translated_summary?:string;media?:MediaDescriptor[]}
export interface WhyChannel{code:string;mode:string;ok:boolean;reason:string}
export interface WhyRules{channel:string;has_rules:boolean;ok:boolean|null;reason:string;lines:string[];truncated:boolean}
export interface Why{item_id:number;event_id:number;title:string;topics:string[];scope:string;channels:WhyChannel[];rules?:WhyRules}
export interface SettingsStatus{has_pending:boolean;can_rollback:boolean;pending_digest?:string;probe_matches_pending?:boolean}
export interface SettingsExport{filename:string;size:number;base64:string}
export interface SettingsCheck{ok:boolean;summary:string;pending_digest?:string}
export interface ProbeStatus{state:"none"|"queued"|"running"|"done"|"error";id?:string;hours?:number;created?:string;finished?:string;text?:string;plan_digest?:string}
export interface Theme{code:string;title:string;enabled:boolean;words:string[]}
export interface ThemeTest{matched:string[];words:number}
export interface ItemDetail extends QueueItem{draft_text?:string;winner_channel?:string;eligible_channels?:string[];event?:{url?:string;first_source?:string;first_published_at?:string;topics?:string[];sources?:string[]};materials?:SourceMaterial[]}

async function getJson<T>(url:string,deviceId:string,serverCaDer:string):Promise<T>{
  const r=await invoke<{status:number;body:string}>("device_get",{url,deviceId,serverCaDer});
  if(r.status<200||r.status>=300)throw new Error(`HTTP ${r.status}`);
  return JSON.parse(r.body) as T;
}

async function postJson<T>(url:string,deviceId:string,serverCaDer:string,payload:unknown):Promise<T>{
  const r=await invoke<{status:number;body:string}>("device_post",{url,deviceId,serverCaDer,body:JSON.stringify(payload??{})});
  if(r.status<200||r.status>=300){let detail="";try{const j=JSON.parse(r.body) as {error?:string;detail?:string};detail=String(j.detail??j.error??"")}catch{};throw new Error(detail?`HTTP ${r.status}: ${detail}`:`HTTP ${r.status}`)}
  return r.body?JSON.parse(r.body) as T:{} as T;
}
function baseUrl(server:string){return server.replace(/\/+$/,"")}
function absoluteUrl(server:string,path:string){return /^https?:\/\//i.test(path)?path:`${baseUrl(server)}${path.startsWith("/")?path:`/${path}`}`}

export class InfoServerClient{
  constructor(private readonly server:string,private readonly deviceId:string,private readonly serverCaDer:string){}
  async handshake(){const d=await getJson<Handshake>(`${baseUrl(this.server)}/api/info-desktop/v1/handshake`,this.deviceId,this.serverCaDer);if(d.protocol!==PROTOCOL_ID)throw new Error(`Unsupported protocol ${String(d.protocol)}; expected ${PROTOCOL_ID}`);return d}
  channels(){return getJson<Channel[]>(`${baseUrl(this.server)}/api/info-desktop/v1/channels`,this.deviceId,this.serverCaDer)}
  queue(limit=100,cursor?:string){
    const params=new URLSearchParams({limit:String(Math.max(1,Math.min(100,limit)))});
    if(cursor)params.set("cursor",cursor);
    return getJson<QueuePage>(`${baseUrl(this.server)}/api/info-desktop/v1/queue?${params.toString()}`,this.deviceId,this.serverCaDer)
  }
  async queueAll(pageSize=100,maxPages=100){
    const items:QueueItem[]=[];
    let cursor:string|undefined;
    let counts:Record<string,number>|undefined;
    const seen=new Set<string>();
    let finished=false;
    for(let pageNo=0;pageNo<maxPages;pageNo++){
      const page=await this.queue(pageSize,cursor);
      if(!counts&&page.counts)counts=page.counts;
      items.push(...page.items);
      const next=page.next_cursor??undefined;
      if(!next){finished=true;break}
      if(seen.has(next))throw new Error("Queue pagination cursor repeated");
      seen.add(next);
      cursor=next;
    }
    if(!finished)throw new Error("Queue pagination safety limit reached");
    return {items,next_cursor:null,counts} satisfies QueuePage;
  }
  item(id:string|number){return getJson<ItemDetail>(`${baseUrl(this.server)}/api/info-desktop/v1/queue/${encodeURIComponent(String(id))}`,this.deviceId,this.serverCaDer)}
  why(id:string|number,channel?:string,scope="route"){
    const params=new URLSearchParams({scope});
    if(channel)params.set("channel",channel);
    return getJson<Why>(`${baseUrl(this.server)}/api/info-desktop/v1/queue/${encodeURIComponent(String(id))}/why?${params.toString()}`,this.deviceId,this.serverCaDer)
  }
  settingsStatus(){return getJson<SettingsStatus>(`${baseUrl(this.server)}/api/info-desktop/v1/settings/status`,this.deviceId,this.serverCaDer)}
  settingsExport(){return getJson<SettingsExport>(`${baseUrl(this.server)}/api/info-desktop/v1/settings/xlsx`,this.deviceId,this.serverCaDer)}
  settingsCheck(base64:string){return postJson<SettingsCheck>(`${baseUrl(this.server)}/api/info-desktop/v1/settings/xlsx/check`,this.deviceId,this.serverCaDer,{base64})}
  settingsApply(expected_digest?:string){return postJson<{ok:boolean}>(`${baseUrl(this.server)}/api/info-desktop/v1/settings/xlsx/apply`,this.deviceId,this.serverCaDer,{expected_digest})}
  settingsRollback(){return postJson<{ok:boolean}>(`${baseUrl(this.server)}/api/info-desktop/v1/settings/xlsx/rollback`,this.deviceId,this.serverCaDer,{})}
  settingsProbe(hours=48){return postJson<{ok:boolean;id:string}>(`${baseUrl(this.server)}/api/info-desktop/v1/settings/probe`,this.deviceId,this.serverCaDer,{hours})}
  settingsProbeStatus(){return getJson<ProbeStatus>(`${baseUrl(this.server)}/api/info-desktop/v1/settings/probe`,this.deviceId,this.serverCaDer)}
  themes(){return getJson<{themes:Theme[]}>(`${baseUrl(this.server)}/api/info-desktop/v1/themes`,this.deviceId,this.serverCaDer)}
  themesTest(words:string[],text:string){return postJson<ThemeTest>(`${baseUrl(this.server)}/api/info-desktop/v1/themes/test`,this.deviceId,this.serverCaDer,{words,text})}
  themesSave(topic:string,words:string[]){return postJson<{ok:boolean;topic:string;words:number}>(`${baseUrl(this.server)}/api/info-desktop/v1/themes/save`,this.deviceId,this.serverCaDer,{topic,words})}
  save(id:string|number,payload:{title:string;draft_text:string;channel:string}){return postJson<{ok:boolean;item_id:number}>(`${baseUrl(this.server)}/api/info-desktop/v1/queue/${encodeURIComponent(String(id))}/save`,this.deviceId,this.serverCaDer,payload)}
  publish(id:string|number){return postJson<{ok:boolean}>(`${baseUrl(this.server)}/api/info-desktop/v1/queue/${encodeURIComponent(String(id))}/publish`,this.deviceId,this.serverCaDer,{})}
  skip(id:string|number){return postJson<{ok:boolean}>(`${baseUrl(this.server)}/api/info-desktop/v1/queue/${encodeURIComponent(String(id))}/skip`,this.deviceId,this.serverCaDer,{})}
  defer(id:string|number,minutes=60){return postJson<{ok:boolean}>(`${baseUrl(this.server)}/api/info-desktop/v1/queue/${encodeURIComponent(String(id))}/defer`,this.deviceId,this.serverCaDer,{minutes})}
  create(payload:{title:string;draft_text:string;channel:string;url?:string}){return postJson<{ok:boolean;item_id:number}>(`${baseUrl(this.server)}/api/info-desktop/v1/queue`,this.deviceId,this.serverCaDer,payload)}
  async preview(path:string){
    const r=await invoke<{status:number;content_type:string;body_base64:string}>("device_get_preview",{url:absoluteUrl(this.server,path),deviceId:this.deviceId,serverCaDer:this.serverCaDer});
    if(r.status<200||r.status>=300)throw new Error(`HTTP ${r.status}`);
    return `data:${r.content_type};base64,${r.body_base64}`;
  }
}
