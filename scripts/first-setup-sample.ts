// Віртуальна перевірка форми: складає файл першого налаштування тим самим кодом, що й вікно,
// і таблицю відповідей перевірок формату. Запуск у CI: esbuild збирає цей файл, node виконує.
// Серверний тест INFO читає результат і порівнює з власними правилами (tests/test_desktop_form_parity.py).
import fs from "node:fs";
import {INSTANCE,ACCESS,checkInstance,checkAccess,checkChannel,buildSheets,templateChannels,templateDefault} from "../src/firstSetupSchema";
import type {Answers} from "../src/firstSetupSchema";
import {makeXlsx} from "../src/xlsxWriter";

const out=process.argv[2];
if(!out)throw new Error("потрібен каталог виводу");
fs.mkdirSync(out,{recursive:true});

const valid:Record<string,string>={project:"Моя редакція",network:"Мої новини",bot_title:"Моя редакція | INFO",bot_description:"Новини нашої редакції.",site:"https://example.org",site_label:"наш сайт",timezone:"Europe/Kyiv",language:"uk"};
const token="123456789:"+"A".repeat(35);
const access:Record<string,string>={"telegram.bot_token":token,"telegram.admin_chat_id":"123456789","userbot.api_id":"1234567","userbot.api_hash":"0123456789abcdef0123456789abcdef"};
const channels:Answers["channels"]={};
const list=templateChannels();
list.forEach((c,i)=>{channels[c.code]={title:c.title,telegram:"",join:"",enabled:false,auto:false}});
const first=list.find(c=>c.code==="test")??list[0];
channels[first.code]={title:"Тестовий канал",telegram:"@my_test_channel",join:"https://t.me/my_test_channel",enabled:true,auto:false};
const answers:Answers={instance:valid,access,channels};
fs.writeFileSync(`${out}/sample.xlsx`,makeXlsx(buildSheets(answers)));

const instanceValues:Record<string,string[]>={
 project:["Моя редакція","","   ","x".repeat(61)],
 bot_title:["Бот","x".repeat(65)],
 bot_description:["Опис.","x".repeat(513)],
 site:["","https://example.org","example.org","ftp://x.org","https://a b.org","https://"],
 site_label:["сайт",""],
 timezone:["Europe/Kyiv","America/Argentina/Buenos_Aires","Kyiv","europe kyiv","UTC"],
 language:["uk","UK","ukr","u","de"],
};
const cases:{kind:string;key:string;value:string;error:boolean}[]=[];
for(const f of INSTANCE){
 for(const v of instanceValues[f.key]??[]) cases.push({kind:"instance",key:f.key,value:v,error:checkInstance(f,v)!==""});
}
const accessValues:Record<string,string[]>={
 "telegram.bot_token":["",token,"123456789:short","abc","12345:"+"A".repeat(40),"1234567890123:"+"A".repeat(40)],
 "telegram.admin_chat_id":["","123456789","-1001234567890","12","abc","12345678901234567"],
 "userbot.api_id":["","1234567","12","abc","1234567890123"],
 "userbot.api_hash":["","0123456789abcdef0123456789abcdef","0123456789ABCDEF0123456789ABCDEF","xyz","0123456789abcdef"],
};
for(const a of ACCESS){
 for(const v of accessValues[a.key]??[]) cases.push({kind:"access",key:a.key,value:v,error:checkAccess(a.key,v)!==""});
}
const channelCases:{enabled:boolean;telegram:string;join:string;auto:boolean}[]=[];
for(const telegram of ["","@my_test_channel","@ab","my_channel","-1001234567890","@1abcde","123"])
 for(const join of ["","https://t.me/my_channel","https://t.me/+AbCdEf12345","http://t.me/x_channel","t.me/my_channel"])
  for(const enabled of [false,true])
   for(const auto of [false,true]) channelCases.push({enabled,telegram,join,auto});
const channelOut=channelCases.map(c=>{
 const e=checkChannel(c.enabled,c.telegram,c.join,c.auto,"test");
 return {...c,errors:{telegram:!!e.telegram,join:!!e.join,auto:!!e.auto}};
});
fs.writeFileSync(`${out}/parity.json`,JSON.stringify({cases,channels:channelOut,defaults:INSTANCE.map(f=>[f.key,templateDefault(f)])},null,1));
console.log(`sample.xlsx і parity.json: ${cases.length} полів, ${channelOut.length} комбінацій каналу`);
