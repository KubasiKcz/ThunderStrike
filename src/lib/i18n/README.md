# ThunderStrike i18n (Internationalization)

This directory contains the translation management system for the application UI.

## Structure
- `index.ts`: Reactive i18n store (`$t`), language switcher (`setLocale`), key resolution with fallback to English, and parameter interpolation.
- `locales/en.json`: Primary language file (English).
- `locales/cs.json`: Czech language file.

## How to add a new language

1. Create a new file in `locales/`, e.g. `locales/de.json`.
2. Copy the keys from `locales/en.json` and translate the values:
   ```json
   {
     "app": { "title": "ThunderStrike" },
     "header": {
       "activateTestLocalization": "testLocalization in config.blk aktivieren",
       "checkGameFolder": "Spielordner prüfen",
       "language": "Sprache"
     }
     ...
   }
   ```
3. Register the new locale in `src/lib/i18n/index.ts`:
   ```ts
   import de from './locales/de.json';

   export const availableLocales: LocaleOption[] = [
     { code: 'en', name: 'English' },
     { code: 'cs', name: 'Čeština' },
     { code: 'de', name: 'Deutsch' }
   ];

   const translations: Record<string, Record<string, any>> = {
     en,
     cs,
     de
   };
   ```

## Usage in Svelte components

```svelte
<script lang="ts">
  import { t, locale, setLocale, availableLocales } from '$lib/i18n';
</script>

<h1>{$t('app.title')}</h1>
<p>{$t('status.diffSavedSuccess', { path: 'data/diffs/units.json' })}</p>
```
