import uk from "./locales/uk.json";import en from "./locales/en.json";import de from "./locales/de.json";import es from "./locales/es.json";import fr from "./locales/fr.json";import ko from "./locales/ko.json";import ja from "./locales/ja.json";
export const languages={uk:{label:"Українська",messages:uk},en:{label:"English",messages:en},de:{label:"Deutsch",messages:de},es:{label:"Español",messages:es},fr:{label:"Français",messages:fr},ko:{label:"한국어",messages:ko},ja:{label:"日本語",messages:ja}} as const;
export type Language=keyof typeof languages;
export function defaultLanguage():Language{const s=localStorage.getItem("info-desktop-language");if(s&&s in languages)return s as Language;const b=navigator.language.toLowerCase().split("-")[0];return b in languages?b as Language:"uk"}
export function translator(l:Language){const a=languages[l].messages as Record<string,string>,f=uk as Record<string,string>;return(k:string)=>a[k]??f[k]??k}
