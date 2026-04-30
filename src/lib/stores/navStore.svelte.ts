import { i18n } from "./i18n.svelte";

class NavStore {
  pageTitle = $state("ThunderStrike");

  setTitle(key: string) {
    this.pageTitle = i18n.t(key);
  }
}

export const navStore = new NavStore();
