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
export interface ItemDetail extends QueueItem{draft_text?:string;winner_channel?:string;eligible_channels?:string[];event?:{url?:string;first_source?:string;first_published_at?:string;topics?:string[];sources?:string[]};materials?:SourceMaterial[]}

async function getJson<T>(url:string,token:string):Promise<T>{
  const r=await invoke<{status:number;body:string}>("readonly_get",{url,token});
  if(r.status<200||r.status>=300)throw new Error(`HTTP ${r.status}`);
  return JSON.parse(r.body) as T;
}
function baseUrl(server:string){return server.replace(/\/+$/,"")}
function absoluteUrl(server:string,path:string){return /^https?:\/\//i.test(path)?path:`${baseUrl(server)}${path.startsWith("/")?path:`/${path}`}`}

export class InfoServerClient{
  constructor(private readonly server:string,private readonly token:string){}
  async handshake(){const d=await getJson<Handshake>(`${baseUrl(this.server)}/api/info-desktop/v1/handshake`,this.token);if(d.protocol!==PROTOCOL_ID)throw new Error(`Unsupported protocol ${String(d.protocol)}; expected ${PROTOCOL_ID}`);return d}
  channels(){return getJson<Channel[]>(`${baseUrl(this.server)}/api/info-desktop/v1/channels`,this.token)}
  queue(limit=200){return getJson<QueuePage>(`${baseUrl(this.server)}/api/info-desktop/v1/queue?limit=${limit}`,this.token)}
  item(id:string|number){return getJson<ItemDetail>(`${baseUrl(this.server)}/api/info-desktop/v1/queue/${encodeURIComponent(String(id))}`,this.token)}
  async preview(path:string){
    const r=await invoke<{status:number;content_type:string;body_base64:string}>("readonly_get_preview",{url:absoluteUrl(this.server,path),token:this.token});
    if(r.status<200||r.status>=300)throw new Error(`HTTP ${r.status}`);
    return `data:${r.content_type};base64,${r.body_base64}`;
  }
}
