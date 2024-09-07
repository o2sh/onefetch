declare module '*/languages.yaml' {
  export interface LanguageColors {
    ansi: string[];
    hex?: string[];
    chip: string;
  }

  export interface Language {
    name: string;
    type: string;
    ascii: string;
    colors: LanguageColors;
    icon: string | null;
  }

  export type Languages = Record<string, Language>;
  export type Language = Language;
}
