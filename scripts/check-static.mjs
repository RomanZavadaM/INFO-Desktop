import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const localeDir = path.join(root, "src", "locales");
const languages = ["uk", "en", "de", "es", "fr", "ko", "ja"];

const read = (p) => fs.readFileSync(p, "utf8");
const base = JSON.parse(read(path.join(localeDir, "uk.json")));
const baseKeys = Object.keys(base).sort();

for (const lang of languages) {
  const file = path.join(localeDir, `${lang}.json`);
  const data = JSON.parse(read(file));
  const keys = Object.keys(data).sort();
  if (JSON.stringify(keys) !== JSON.stringify(baseKeys)) {
    throw new Error(`Locale key mismatch: ${lang}`);
  }
  for (const [key, value] of Object.entries(data)) {
    if (typeof value !== "string" || value.trim() === "") {
      throw new Error(`Empty locale value: ${lang}:${key}`);
    }
  }
}

const native = read(path.join(root, "src-tauri", "src", "lib.rs"));
for (const forbidden of [".put(", ".patch(", ".delete("]) {
  if (native.includes(forbidden)) {
    throw new Error(`Native layer contains forbidden method: ${forbidden}`);
  }
}
if (native.includes(".post(")) {
  throw new Error("Native layer must use explicit Method::POST only for enrollment");
}
const postCount=(native.match(/Method::POST/g)||[]).length;
if(postCount!==1){
  throw new Error(`Expected exactly one enrollment POST, found ${postCount}`);
}
if (!native.includes(".get(")) {
  throw new Error("Read-only native layer has no GET transport");
}

console.log("Static contract checks: OK");
