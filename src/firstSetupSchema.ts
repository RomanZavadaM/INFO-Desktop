// Поля першого налаштування й перевірки формату. Ті самі правила на сервері: app/infolib/first_setup.py та access.py.
// Сервер перевіряє файл ще раз під час завантаження, тут лише швидка підказка до збереження.
import template from "./firstSetupTemplate.json";

export interface FieldDef{key:string;column:string;label:string;hint:string;example:string;required:boolean;max:number}
export const INSTANCE:FieldDef[]=[
 {key:"project",column:"Назва проєкту",label:"Назва проєкту",hint:"Як називається ваша редакція. Бачать у службових повідомленнях.",example:"Моя редакція",required:true,max:60},
 {key:"network",column:"Назва мережі",label:"Назва мережі каналів",hint:"Спільна назва всіх ваших каналів (підпис у шапках).",example:"Мої новини",required:true,max:60},
 {key:"bot_title",column:"Заголовок бота",label:"Заголовок бота",hint:"Коротка назва бота в Telegram (до 64 знаків).",example:"Моя редакція | INFO",required:true,max:64},
 {key:"bot_description",column:"Опис бота",label:"Опис бота",hint:"Одне-два речення, що робить бот (до 512 знаків).",example:"Новини нашої редакції.",required:true,max:512},
 {key:"site",column:"Сайт",label:"Сайт (необов'язково)",hint:"Адреса вашого сайту разом із https://. Можна лишити порожнім.",example:"https://example.org",required:false,max:200},
 {key:"site_label",column:"Підпис сайту",label:"Підпис сайту",hint:"Короткий текст посилання на сайт.",example:"наш сайт",required:true,max:40},
 {key:"timezone",column:"Часовий пояс",label:"Часовий пояс",hint:"Назва за списком IANA; від нього залежить час дайджестів.",example:"Europe/Kyiv",required:true,max:40},
 {key:"language",column:"Мова",label:"Мова за замовчуванням",hint:"Дві літери: uk, en, de ...",example:"uk",required:true,max:2},
];
export interface AccessDef{key:string;label:string;hint:string;example:string;secret:boolean}
export const ACCESS:AccessDef[]=[
 {key:"telegram.bot_token",label:"Токен бота",hint:"Видає @BotFather після команди /newbot. Вигляд: 123456789:AAE... (цифри, двокрапка, довгий набір літер). Секрет: після завантаження файлу лишається лише на сервері.",example:"123456789:AAExampleExampleExampleExample12345",secret:true},
 {key:"telegram.admin_chat_id",label:"Telegram-id першого адміна",hint:"Ціле число вашого акаунта в Telegram: напишіть боту @userinfobot і скопіюйте «Id». Лише цифри.",example:"123456789",secret:false},
 {key:"userbot.api_id",label:"api_id акаунта зчитування",hint:"Для зчитування каналів-джерел. Беруть на my.telegram.org у розділі API development tools. Лише цифри. Якщо зчитування не потрібне, лишіть порожнім.",example:"1234567",secret:true},
 {key:"userbot.api_hash",label:"api_hash акаунта зчитування",hint:"Там само: 32 символи з цифр і літер a-f. Якщо зчитування не потрібне, лишіть порожнім.",example:"0123456789abcdef0123456789abcdef",secret:true},
];
export const CHANNEL_HINT_TELEGRAM="Адреса каналу: @username (5-32 знаки, лише латиниця, цифри, _) або числовий id. Бот має бути адміністратором каналу.";
export const CHANNEL_HINT_JOIN="Посилання для читачів: https://t.me/імя або запрошення https://t.me/+код. Необов'язково.";

const TG=/^(@[A-Za-z][A-Za-z0-9_]{4,31}|-?\d{5,16})$/;
const JOIN=/^(https:\/\/t\.me\/[A-Za-z][A-Za-z0-9_]{4,31}\/?|https:\/\/t\.me\/\+[A-Za-z0-9_-]{8,}\/?)$/;
const SITE=/^https?:\/\/[^\s<>"]{3,200}$/;
const TZ=/^[A-Za-z]+(?:\/[A-Za-z_+-]+){1,2}$/;
const LANG=/^[a-z]{2}$/;

export function checkInstance(f:FieldDef,raw:string):string{
 const v=raw.trim().replace(/\s+/g," ");
 if(!v)return f.required?`«${f.label}»: заповніть`:"";
 if(v.length>f.max)return `«${f.label}»: не більше ${f.max} знаків (зараз ${v.length})`;
 if(f.key==="site"&&!SITE.test(v))return "Сайт: адреса має починатися з https:// (приклад: https://example.org)";
 if(f.key==="timezone"&&!TZ.test(v))return "Часовий пояс: формат Регіон/Місто, наприклад Europe/Kyiv";
 if(f.key==="language"&&!LANG.test(v))return "Мова: дві малі літери, наприклад uk";
 return "";
}
export function checkAccess(key:string,raw:string):string{
 const v=raw.trim();if(!v)return "";
 if(key==="telegram.bot_token"&&!/^\d{6,12}:[A-Za-z0-9_-]{30,}$/.test(v))return `токен бота має вигляд 123456789:ABC... (введено символів: ${v.length})`;
 if(key==="telegram.admin_chat_id"&&!/^-?\d{4,16}$/.test(v))return `id адміна - ціле число (введено символів: ${v.length})`;
 if(key==="userbot.api_id"&&!(/^\d+$/.test(v)&&v.length>=4&&v.length<=12))return `api_id - лише цифри (введено символів: ${v.length})`;
 if(key==="userbot.api_hash"&&!/^[0-9a-fA-F]{32}$/.test(v))return `api_hash - 32 символи 0-9 a-f (введено символів: ${v.length})`;
 return "";
}
export function checkChannel(enabled:boolean,telegram:string,join:string,auto:boolean,code:string):{telegram?:string;join?:string;auto?:string}{
 const e:{telegram?:string;join?:string;auto?:string}={};
 const t=telegram.trim(),j=join.trim();
 if(t&&!TG.test(t))e.telegram=`Канал ${code}: адреса має вигляд @username або числовий id`;
 if(j&&!JOIN.test(j))e.join=`Канал ${code}: посилання має вигляд https://t.me/імя або https://t.me/+код`;
 if(enabled&&!t&&!e.telegram)e.telegram=`Канал ${code}: щоб увімкнути, вкажіть адресу каналу`;
 if(auto&&!enabled)e.auto=`Канал ${code}: автопублікація можлива лише для увімкненого каналу`;
 return e;
}

type Sheet={name:string;rows:string[][]};
export const SHEETS=(template as {sheets:Sheet[]}).sheets;
const sheet=(name:string)=>SHEETS.find(s=>s.name===name);
const INSTANCE_SHEET="Інстанс",ACCESS_SHEET="Доступи",CHANNEL_SHEET="Канали публікації";
function col(rows:string[][],name:string){return rows[0].map(x=>x.trim()).indexOf(name)}

export interface TemplateChannel{code:string;title:string}
export function templateChannels():TemplateChannel[]{
 const s=sheet(CHANNEL_SHEET);if(!s)return [];
 const c=col(s.rows,"Код"),n=col(s.rows,"Назва");
 return s.rows.slice(1).map(r=>({code:r[c],title:r[n]}));
}
export function templateDefault(f:FieldDef):string{
 const s=sheet(INSTANCE_SHEET);if(!s)return "";const i=col(s.rows,f.column);return i<0?"":(s.rows[1][i]??"");
}

export interface Answers{instance:Record<string,string>;access:Record<string,string>;channels:Record<string,{title:string;telegram:string;join:string;enabled:boolean;auto:boolean}>}

export function buildSheets(a:Answers):Sheet[]{
 const copy:Sheet[]=SHEETS.map(s=>({name:s.name,rows:s.rows.map(r=>[...r])}));
 const get=(n:string)=>copy.find(s=>s.name===n)!;
 const inst=get(INSTANCE_SHEET);
 for(const f of INSTANCE){const i=col(inst.rows,f.column);if(i>=0)inst.rows[1][i]=(a.instance[f.key]??"").trim().replace(/\s+/g," ")}
 const acc=get(ACCESS_SHEET);const vi=col(acc.rows,"Значення");
 for(const r of acc.rows.slice(1))r[vi]=(a.access[r[0].trim()]??"").trim();
 const ch=get(CHANNEL_SHEET);
 const ci={code:col(ch.rows,"Код"),title:col(ch.rows,"Назва"),tg:col(ch.rows,"Telegram"),join:col(ch.rows,"Посилання для читача"),on:col(ch.rows,"Увімкнено"),auto:col(ch.rows,"Автопублікація")};
 for(const r of ch.rows.slice(1)){
  const c=a.channels[r[ci.code]];if(!c)continue;
  r[ci.title]=c.title.trim();r[ci.tg]=c.telegram.trim();r[ci.join]=c.join.trim();
  r[ci.on]=c.enabled?"так":"ні";r[ci.auto]=c.enabled&&c.auto?"так":"ні";
 }
 return copy;
}
