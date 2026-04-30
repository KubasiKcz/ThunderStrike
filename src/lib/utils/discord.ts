import { start, setActivity } from "tauri-plugin-drpc";
import { Activity, Assets } from "tauri-plugin-drpc/activity";
import { toastStore } from "../stores/toastStore";

export async function initDiscordRPC() {
  try {
    await start("1498385285866721521");
    await updateDiscordRPC("Idle", "In the manager...");
  } catch (e) {
    console.warn("Discord RPC Init Failed", e);
    toastStore.add("Discord RPC se nepodařilo inicializovat: " + e, "warning");
  }
}

export async function updateDiscordRPC(state: string, details: string) {
  try {
    let activity = new Activity()
      .setState(state)
      .setDetails(details)
      .setAssets(
        new Assets().setLargeImage("logo").setLargeText("ThunderStrike"),
      );
    await setActivity(activity);
  } catch (e) {
    console.warn("Failed to update Discord RPC", e);
    toastStore.add("Nepodařilo se aktualizovat Discord RPC: " + e, "warning");
  }
}
