import { writable, derived, get } from 'svelte/store';
import en from './locales/en.json';
import cs from './locales/cs.json';

export type Locale = 'en' | 'cs' | string;

export interface LocaleOption {
  code: string;
  name: string;
}

export const availableLocales: LocaleOption[] = [
  { code: 'en', name: 'English' },
  { code: 'cs', name: 'Čeština' }
];

export const defaultLocale = 'en';

const translations: Record<string, Record<string, any>> = {
  en,
  cs
};

/**
 * Register a new locale dynamically if needed
 */
export function registerLocale(code: string, name: string, data: Record<string, any>) {
  translations[code] = data;
  if (!availableLocales.find((l) => l.code === code)) {
    availableLocales.push({ code, name });
  }
}

// Helper to access nested property via dot notation (e.g. "header.checkGameFolder")
function getNestedValue(obj: Record<string, any> | undefined, path: string): string | undefined {
  if (!obj || typeof obj !== 'object') return undefined;
  const parts = path.split('.');
  let curr: any = obj;
  for (const part of parts) {
    if (curr && typeof curr === 'object' && part in curr) {
      curr = curr[part];
    } else {
      return undefined;
    }
  }
  return typeof curr === 'string' ? curr : undefined;
}

// Helper to replace {param} placeholders with values
function interpolate(template: string, params?: Record<string, string | number>): string {
  if (!params) return template;
  return template.replace(/\{(\w+)\}/g, (match, key) => {
    return key in params ? String(params[key]) : match;
  });
}

function getInitialLocale(): string {
  if (typeof window !== 'undefined' && window.localStorage) {
    const saved = localStorage.getItem('thunderstrike_locale');
    if (saved && saved in translations) {
      return saved;
    }
  }
  return defaultLocale;
}

export const locale = writable<string>(getInitialLocale());

export function setLocale(newLocale: string) {
  if (newLocale in translations) {
    locale.set(newLocale);
    if (typeof window !== 'undefined' && window.localStorage) {
      localStorage.setItem('thunderstrike_locale', newLocale);
    }
  }
}

// Generate strongly-typed translation keys based on the default en.json
type NestedKeyOf<ObjectType extends object> = {
  [Key in keyof ObjectType & (string | number)]: ObjectType[Key] extends object
    ? `${Key}.${NestedKeyOf<ObjectType[Key]>}`
    : `${Key}`;
}[keyof ObjectType & (string | number)];

export type TranslationKey = NestedKeyOf<typeof en>;

export function translate(
  currentLocale: string,
  key: string,
  params?: Record<string, string | number>
): string {
  const currentDict = translations[currentLocale];
  let val = getNestedValue(currentDict, key);

  // Fallback to defaultLocale (en) if missing in current dictionary
  if (val === undefined && currentLocale !== defaultLocale) {
    val = getNestedValue(translations[defaultLocale], key);
  }

  // Fallback to key itself if not found anywhere
  if (val === undefined) {
    return key;
  }

  return interpolate(val, params);
}

export function tCurrent(key: TranslationKey | string, params?: Record<string, string | number>): string {
  return translate(get(locale), key, params);
}

// Reactive store for use with $t('key', { params })
export const t = derived(locale, ($locale) => {
  return (key: TranslationKey | string, params?: Record<string, string | number>): string => {
    return translate($locale, key, params);
  };
});
