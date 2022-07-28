declare module '*/languages.yaml' {
  export interface LanguageColors {
    ansi: string[];
    hex?: string[];
    chip: string;
  }

  export interface Language {
    type: string;
    ascii: string;
    colors: LanguageColors;
  }

  export type Languages = Record<string, Language>;
}
