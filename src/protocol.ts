import { invoke } from "@tauri-apps/api/core";
export const PROTOCOL_ID="info-desktop-protocol/1";
export interface Handshake{protocol:string;server_name?:string;instance_name?:string;server_version?:string;timezone?:string;capabilities?:string[];languages?:string[]}
export interface Channel{id?:string|number;code?:string;name?:string;pending?:number}
export interface QueueItem{id:string|number;channel?:string;channel_name?:string;title?:string;created_at?:string;published_at?:string;state?:string;language?:string;media_count?:number;quality_score?:number;quality_label?:string}
export interface QueuePage{items:QueueItem[];next_cursor?:string|null}
export interface ItemDetail extends QueueItem{text?:string;prepared_text?:string;original_text?:string;source?:string;source_name?:string;source_url?:string;media?:Array<{id?:string|number;kind?:string;url?:string;preview_url?:string;mime_type?:string;width?:number;height?:number;size?:number}>;related?:QueueItem[];warnings?:string[]}
async function getJson<T>(url:string,token:string):Promise<T>{const r=await invoke<{status:number;body:string}>("readonly_get",{url,token});if(r.status<200||r.status>=300)throw new Error(`HTTP ${r.status}`);return JSON.parse(r.body) as T}
function baseUrl(server:string){return server.replace(/\/+$/,"")}
export class InfoServerClient{
 constructor(private readonly server:string,private readonly token:string){}
 async handshake(){const d=await getJson<Handshake>(`${baseUrl(this.server)}/api/info-desktop/v1/handshake`,this.token);if(d.protocol!==PROTOCOL_ID)throw new Error(`Unsupported protocol ${String(d.protocol)}; expected ${PROTOCOL_ID}`);return d}
 channels(){return getJson<Channel[]>(`${baseUrl(this.server)}/api/info-desktop/v1/channels`,this.token)}
 queue(limit=200){return getJson<QueuePage>(`${baseUrl(this.server)}/api/info-desktop/v1/queue?limit=${limit}`,this.token)}
 item(id:string|number){return getJson<ItemDetail>(`${baseUrl(this.server)}/api/info-desktop/v1/queue/${encodeURIComponent(String(id))}`,this.token)}
}
