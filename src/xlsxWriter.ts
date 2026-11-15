// Мінімальний запис XLSX без залежностей: ZIP без стиснення, рядки як inlineStr.
// Формат той самий, що робить сервер INFO (make_xlsx), тож `info menu` читає файл так само.
const enc=new TextEncoder();

function crc32(bytes:Uint8Array){
 let crc=0xFFFFFFFF;
 for(let i=0;i<bytes.length;i++){
  let c=(crc^bytes[i])&0xFF;
  for(let k=0;k<8;k++)c=c&1?(0xEDB88320^(c>>>1)):(c>>>1);
  crc=(crc>>>8)^c;
 }
 return (crc^0xFFFFFFFF)>>>0;
}

function colName(n:number){let s="";while(n>0){const r=(n-1)%26;s=String.fromCharCode(65+r)+s;n=Math.floor((n-1)/26)}return s}
function esc(t:string){return t.replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;")}

function sheetXml(rows:string[][]){
 const out=['<?xml version="1.0" encoding="UTF-8" standalone="yes"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>'];
 rows.forEach((row,ri)=>{
  out.push(`<row r="${ri+1}">`);
  row.forEach((v,ci)=>{
   const text=String(v??"");
   const space=text!==text.trim()?' xml:space="preserve"':"";
   out.push(`<c r="${colName(ci+1)}${ri+1}" t="inlineStr"><is><t${space}>${esc(text)}</t></is></c>`);
  });
  out.push("</row>");
 });
 out.push("</sheetData></worksheet>");
 return out.join("");
}

function zipStored(files:{name:string;data:Uint8Array}[]){
 const chunks:Uint8Array[]=[];const central:Uint8Array[]=[];let offset=0;
 for(const f of files){
  const name=enc.encode(f.name);const crc=crc32(f.data);
  const local=new Uint8Array(30+name.length);const lv=new DataView(local.buffer);
  lv.setUint32(0,0x04034b50,true);lv.setUint16(4,20,true);lv.setUint16(6,0x0800,true);lv.setUint16(8,0,true);
  lv.setUint16(10,0,true);lv.setUint16(12,0x21,true);lv.setUint32(14,crc,true);lv.setUint32(18,f.data.length,true);
  lv.setUint32(22,f.data.length,true);lv.setUint16(26,name.length,true);lv.setUint16(28,0,true);local.set(name,30);
  chunks.push(local,f.data);
  const cen=new Uint8Array(46+name.length);const cv=new DataView(cen.buffer);
  cv.setUint32(0,0x02014b50,true);cv.setUint16(4,20,true);cv.setUint16(6,20,true);cv.setUint16(8,0x0800,true);
  cv.setUint16(10,0,true);cv.setUint16(12,0,true);cv.setUint16(14,0x21,true);cv.setUint32(16,crc,true);
  cv.setUint32(20,f.data.length,true);cv.setUint32(24,f.data.length,true);cv.setUint16(28,name.length,true);
  cv.setUint32(42,offset,true);cen.set(name,46);central.push(cen);
  offset+=local.length+f.data.length;
 }
 const cdSize=central.reduce((a,c)=>a+c.length,0);
 const end=new Uint8Array(22);const ev=new DataView(end.buffer);
 ev.setUint32(0,0x06054b50,true);ev.setUint16(8,files.length,true);ev.setUint16(10,files.length,true);
 ev.setUint32(12,cdSize,true);ev.setUint32(16,offset,true);
 const all=[...chunks,...central,end];const total=all.reduce((a,c)=>a+c.length,0);
 const result=new Uint8Array(total);let pos=0;for(const c of all){result.set(c,pos);pos+=c.length}
 return result;
}

export function makeXlsx(sheets:{name:string;rows:string[][]}[]):Uint8Array{
 const text=(s:string)=>enc.encode(s);
 const ns="http://schemas.openxmlformats.org/spreadsheetml/2006/main";
 const rel="http://schemas.openxmlformats.org/officeDocument/2006/relationships";
 const pkg="http://schemas.openxmlformats.org/package/2006/relationships";
 const head='<?xml version="1.0" encoding="UTF-8" standalone="yes"?>';
 const overrides=sheets.map((_s,i)=>`<Override PartName="/xl/worksheets/sheet${i+1}.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>`).join("");
 const files=[
  {name:"[Content_Types].xml",data:text(`${head}<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"/>${overrides}</Types>`)},
  {name:"_rels/.rels",data:text(`${head}<Relationships xmlns="${pkg}"><Relationship Id="rId1" Type="${rel}/officeDocument" Target="xl/workbook.xml"/></Relationships>`)},
  {name:"xl/workbook.xml",data:text(`${head}<workbook xmlns="${ns}" xmlns:r="${rel}"><sheets>${sheets.map((s,i)=>`<sheet name="${esc(s.name)}" sheetId="${i+1}" r:id="rId${i+1}"/>`).join("")}</sheets></workbook>`)},
  {name:"xl/_rels/workbook.xml.rels",data:text(`${head}<Relationships xmlns="${pkg}">${sheets.map((_s,i)=>`<Relationship Id="rId${i+1}" Type="${rel}/worksheet" Target="worksheets/sheet${i+1}.xml"/>`).join("")}<Relationship Id="rId${sheets.length+1}" Type="${rel}/styles" Target="styles.xml"/></Relationships>`)},
  {name:"xl/styles.xml",data:text(`${head}<styleSheet xmlns="${ns}"><fonts count="1"><font><sz val="11"/><name val="Calibri"/></font></fonts><fills count="2"><fill><patternFill patternType="none"/></fill><fill><patternFill patternType="gray125"/></fill></fills><borders count="1"><border><left/><right/><top/><bottom/><diagonal/></border></borders><cellStyleXfs count="1"><xf numFmtId="0" fontId="0" fillId="0" borderId="0"/></cellStyleXfs><cellXfs count="1"><xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"/></cellXfs></styleSheet>`)},
  ...sheets.map((s,i)=>({name:`xl/worksheets/sheet${i+1}.xml`,data:text(sheetXml(s.rows))})),
 ];
 return zipStored(files);
}
