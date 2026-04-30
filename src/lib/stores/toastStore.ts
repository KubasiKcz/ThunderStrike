import { writable } from "svelte/store";

export type ToastType = "info" | "success" | "warning" | "error" | "loading" | string;

export interface Toast {
  id: string;
  message: string;
  type: ToastType;
}

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);

  return {
    subscribe,
    
    add: (message: string, type: ToastType = "info", duration: number = 2500) => {
      const id = crypto.randomUUID();
      
      // Chybové toasty zůstávají 5 minut (300 000 ms) podle požadavku uživatele
      let finalDuration = duration;
      if (type === "error" && duration === 2500) {
        finalDuration = 300000;
      }

      update((all) => [...all, { id, message, type }]);

      if (finalDuration > 0) {
        setTimeout(() => {
          update((all) => all.filter((t) => t.id !== id));
        }, finalDuration);
      }
      return id;
    },
    
    updateToast: (id: string, message: string, type: ToastType = "info", duration: number = 2500) => {
      let finalDuration = duration;
      if (type === "error" && duration === 2500) {
        finalDuration = 300000;
      }

      update((all) =>
        all.map((t) => (t.id === id ? { ...t, message, type } : t))
      );
      
      if (finalDuration > 0) {
        setTimeout(() => {
          update((all) => all.filter((t) => t.id !== id));
        }, finalDuration);
      }
    },
    
    remove: (id: string) => {
      update((all) => all.filter((t) => t.id !== id));
    }
  };
}

export const toastStore = createToastStore();