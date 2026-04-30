const translations = {
  cs: {
    "Welcome to ThunderStrike!": "Vítejte v ThunderStrike!",
    "Step 1 of 4: App Language": "Krok 1 ze 4: Jazyk aplikace",
    "Which language should the application UI use?": "Jaký jazyk má používat rozhraní aplikace?",
    "Step 2 of 4: Game Location": "Krok 2 ze 4: Umístění hry",
    "The disk location where the game installation is found:": "Umístění na disku, kde je nainstalována hra:",
    "Step 3 of 4: In-Game Language": "Krok 3 ze 4: Jazyk ve hře",
    "Which language should be used inside the game?": "Jaký jazyk se má používat přímo ve hře?",
    "Step 4 of 4: War Thunder Profile": "Krok 4 ze 4: War Thunder Profil",
    "Enter your exact War Thunder nickname (Warning: It is Case-Sensitive!).": "Zadejte přesnou War Thunder přezdívku (Pozor: Záleží na malých/velkých písmenech!).",
    "Home Dashboard": "Dashboard",
    "WT Text Editor": "Textový Editor",
    "Software Info": "Informace",
    "Level": "Úroveň",
    "Registered:": "Registrován:",
    "Registered": "Registrován",
    "Sync Profile": "Sync Profil",
    "Updating...": "Aktualizuji...",
    "Arcade": "Arkáda",
    "Realistic": "Realistické",
    "Simulator": "Simulátor",
    "Victories": "Vítězství",
    "Missions": "Mise",
    "Win Ratio": "Poměr výher",
    "Deaths": "Úmrtí",
    "Lions": "Stříbrní lvi",
    "Play Time": "Doba hraní",
    "Air Targets": "Vzdušné cíle",
    "Ground Targets": "Pozemní cíle",
    "Naval Targets": "Námořní cíle",
    "Game Mods": "Herní módy",
    "Localization": "Lokalizace",
    "Sound Mods": "Zvukové módy",
    "Custom Hangar": "Vlastní hangár",
    "Configure custom game modifications.": "Nastavení vlastních úprav hry.",
    "Restart game": "Restartujte hru",
    "No statistics available for this mode.": "Pro tento mód nejsou k dispozici žádné statistiky.",
    "Player Profile": "Hráčský profil",
    "Last updated:": "Naposledy aktualizováno:",
    "Update Profile": "Aktualizovat profil",
    "Czech": "Čeština",
    "English": "Angličtina",
    "Continue": "Pokračovat",
    "Back": "Zpět",
    "Finish Setup": "Dokončit nastavení",
    "No nickname found. Please complete onboarding.": "Přezdívka nebyla nalezena. Dokončete prosím uvítací proces.",
    "Waiting for profile data...": "Čekám na data z profilu...",
    "Fetch Profile Now": "Načíst profil",
    "Dark Mode": "Tmavý režim",
    "App Language": "Jazyk aplikace",
    "Game language": "Jazyk hry",
    "WarThunder username": "Uživatelské jméno WarThunder",
    "Steam username": "Uživatelské jméno Steam",
    "Game location": "Umístění hry",
    "Save settings": "Uložit nastavení",
    "Save Settings": "Uložit nastavení",
    "Dev Testing": "Dev Testování",
    "Toast Test": "Toast Test",
    "Test Toasts": "Testovat Toasty",
    "Factory Reset": "Tovární nastavení",
    "Reset program (Hard Reset)": "Resetovat program (Tvrdý Reset)",
    "Launch WT": "Spustit WT",
    "Running...": "Běží...",
    "Settings": "Nastavení",
    "General": "Obecné",
    "Search key or translation...": "Hledat klíč nebo překlad...",
    "Show only recommended G-Keys (QSelect)": "Zobrazit pouze doporučené G-Keys (QSelect)",
    "Restore original (Delete local modification)": "Obnovit originál (Smazat místní úpravu)",
    "No data loaded or filter found nothing...": "Žádná otevřená data nebo filtr nic nenašel...",
    "Key": "Klíč",
    "Original Reference (EN)": "Originální Reference (AJ)",
    "Your Translation": "Tvůj Překlad",
    "Deleted": "Smazáno",
    "The game will generate the original.": "Hra vygeneruje původní originál.",
    "Deletion failed": "Smazání selhalo",
    "Saved": "Uloženo",
    "Error writing data": "Chyba při zápisu dat",
    "New War Thunder version detected. Applying modifications...": "Byla detekována nová verze War Thunderu. Aplikuji modifikace...",
    "Error loading": "Chyba načítání",
    "Failed to save translation": "Nepodařilo se uložit překlad"
  }
};

export type Language = "en" | "cs";

export const AVAILABLE_LANGUAGES = [
  { code: "en", name: "English" },
  { code: "cs", name: "Čeština" }
];

class I18nStore {
  currentLanguage = $state<Language>("en");

  t(key: string) {
    // @ts-ignore
    if (this.currentLanguage === "en") return key;
    // @ts-ignore
    return translations[this.currentLanguage]?.[key] || key;
  }
}

export const i18n = new I18nStore();
