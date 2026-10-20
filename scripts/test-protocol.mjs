import fs from "node:fs";

const source=fs.readFileSync(new URL("../src/protocol.ts",import.meta.url),"utf8");
for(const fragment of ["queueAll(","next_cursor","Queue pagination cursor repeated","maxPages"]){
  if(!source.includes(fragment))throw new Error(`pagination guard missing: ${fragment}`);
}
console.log("queue pagination contract present");
