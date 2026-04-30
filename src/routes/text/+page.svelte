<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { toastStore } from "../../lib/stores/toastStore";
  import { i18n } from "../../lib/stores/i18n.svelte";
  import { navStore } from "../../lib/stores/navStore.svelte";

  let gameLocation = $state("");
  let gameLang = $state("");
  let availableFiles = $state<string[]>([]);
  let selectedFile = $state("main.csv");
  let appPath = $state("");
  
  let qSelectIds = $state<Set<string>>(new Set());
  let csvDataMode = $state<any[]>([]);
  let tableRows = $state<any[]>([]);
  let visibleLimit = $state(50);
  let visibleRows = $derived(tableRows.slice(0, visibleLimit));
  let delimiter = $state(";");
  let colIdIdx = $state(0);
  let colTargetIdx = $state(-1);
  let colRefIdx = $state(1);
  
  let searchQuery = $state("");
  let showOnlyQSelect = $state(true);
  
  onMount(async () => {
    try {
      gameLocation = await invoke("get_setting", { key: "gameLocation" }) as string;
      gameLang = await invoke("get_setting", { key: "gameLang" }) as string;
      appPath = await invoke("get_app_path");
      
      if (!gameLocation || !gameLang) {
        toastStore.add("Nastavení chybí. Prosím, projděte Onboardingem znovu.", "error");
        return;
      }

      await checkAndApplyWTUpdates();
      
      try {
        availableFiles = await invoke("list_csv_files", { path: gameLocation }) as string[];
      } catch (e) {
        toastStore.add("Nelze načíst složku lang.", "error");
        return;
      }
      if (availableFiles.length > 0) {
        let defFile = availableFiles.includes("menu.csv") ? "menu.csv" : availableFiles[0];
        await loadCsvFile(defFile);
      }
    } catch (e) {
      toastStore.add(`${i18n.t("Error loading")}: ${e}`, "error");
    }
  });
  
  async function checkAndApplyWTUpdates() {
     try {
       const currentVer = await invoke("get_wt_version", { wtPath: gameLocation });
       const savedVer = await invoke("get_setting", { key: "lastWtVersion" }).catch(()=>"");
       if (currentVer !== savedVer) {
           await invoke("save_setting", { key: "lastWtVersion", value: currentVer as string });
           toastStore.add(i18n.t("New War Thunder version detected. Applying modifications..."), "info");
       }
     } catch (e) {}
  }
  
  import qSelectRaw from '../../lib/data/QSelect.csv?raw';
  onMount(() => {
    let ids = qSelectRaw.split('\n').filter(line => line && !line.startsWith('<ID')).map(l => l.trim());
    qSelectIds = new Set(ids);
  });

  function autoResize(node: HTMLTextAreaElement) {
      const resize = () => {
          node.style.height = 'auto';
          node.style.height = (node.scrollHeight + 2) + 'px';
      };
      node.addEventListener('input', resize);
      setTimeout(resize, 0);
      return {
          destroy() {
              node.removeEventListener('input', resize);
          }
      };
  }

  function parseCsvString(csvText: string) {
    let firstLineEnd = csvText.indexOf('\n');
    let firstLine = firstLineEnd !== -1 ? csvText.substring(0, firstLineEnd) : csvText;
    let delim = firstLine.includes(';') ? ';' : ',';
    
    let matrix = [];
    let currentRow = [];
    let currentCell = "";
    let inQuotes = false;
    
    for (let i = 0; i < csvText.length; i++) {
        let char = csvText[i];
        let nextChar = csvText[i+1];
        
        if (char === '"') {
            if (inQuotes && nextChar === '"') {
                currentCell += '"';
                i++;
            } else {
                inQuotes = !inQuotes;
            }
        } else if (char === delim && !inQuotes) {
            currentRow.push(currentCell);
            currentCell = "";
        } else if ((char === '\n' || char === '\r') && !inQuotes) {
            if (char === '\r' && nextChar === '\n') i++;
            
            currentRow.push(currentCell);
            if (currentRow.length > 0) {
                matrix.push(currentRow);
            }
            currentRow = [];
            currentCell = "";
        } else {
            currentCell += char;
        }
    }
    
    if (currentRow.length > 0 || currentCell !== "") {
        currentRow.push(currentCell);
        matrix.push(currentRow);
    }
    
    return { matrix, delim };
  }

  function encodeCsvString(matrix: any[][], delim: string) {
    return matrix.map(row => row.map(val => {
        if (typeof val === 'string' && (val.includes(delim) || val.includes('"') || val.includes('\n') || val.includes('\r'))) {
            return `"${val.replace(/"/g, '""')}"`;
        }
        return val;
    }).join(delim)).join('\n');
  }

  async function loadCsvFile(filename: string) {
    selectedFile = filename;
    try {
      const fileData = await invoke("read_text_file", { path: `${gameLocation}\\${filename}` }) as string;
      const { matrix, delim } = parseCsvString(fileData);
      csvDataMode = matrix;
      delimiter = delim;
      
      let header = matrix[0];
      colIdIdx = 0;
      
      colTargetIdx = header.findIndex((h: string) => h.includes(`<${gameLang}>`) || h.toLowerCase() === gameLang.toLowerCase() || h.includes(gameLang));
      if (colTargetIdx === -1) {
          header.push(gameLang);
          colTargetIdx = header.length - 1;
          for (let i=1; i<matrix.length; i++) matrix[i].push("");
      }
      
      let rIdx = header.findIndex((h: string) => h.includes("English"));
      colRefIdx = rIdx !== -1 ? rIdx : 1;
      
      rebuildTable();
    } catch(e) {
      toastStore.add("Chyba při otevírání CSV (možná neexistuje): " + e, "error");
    }
  }

  function rebuildTable() {
    let rows = [];
    for (let i = 1; i < csvDataMode.length; i++) {
        let cols = csvDataMode[i];
        let key = cols[colIdIdx];
        let target = cols[colTargetIdx] || "";
        let ref = cols[colRefIdx] || "";
        let inQSelect = qSelectIds.has(key);
        
        let matchFilter = true;
        if (showOnlyQSelect && !inQSelect) matchFilter = false;
        if (searchQuery && !key.toLowerCase().includes(searchQuery.toLowerCase()) && !target.toLowerCase().includes(searchQuery.toLowerCase())) matchFilter = false;
        
        if (matchFilter) {
           rows.push({ _rowIndex: i, key, ref, target, inQSelect });
        }
    }
    tableRows = rows;
    visibleLimit = 50;
  }
  
  $effect(() => {
     let _ = searchQuery; let __ = showOnlyQSelect;
     rebuildTable();
  });

  async function saveCell(rowObj: any, newVal: string) {
      rowObj.target = newVal;
      csvDataMode[rowObj._rowIndex][colTargetIdx] = newVal;
      
      let newCsvText = encodeCsvString(csvDataMode, delimiter);
      try {
          await invoke("write_text_file", { path: `${gameLocation}\\${selectedFile}`, content: newCsvText });
          let edits: any = {};
          try {
             edits = JSON.parse(await invoke("get_custom_edits") as string);
          } catch(e) {}
          if (!edits[selectedFile]) edits[selectedFile] = {};
          edits[selectedFile][rowObj.key] = newVal;
          await invoke("save_custom_edits", { content: JSON.stringify(edits, null, 2) });
          
          toastStore.add(`${i18n.t("Saved")}: ${rowObj.key}`, "success", 1000);
      } catch (e) {
          toastStore.add(i18n.t("Failed to save translation"), "error");
      }
  }

  async function restoreOriginal() {
      try {
          await invoke("delete_file", { path: `${gameLocation}\\${selectedFile}` });
          toastStore.add(`${i18n.t("Deleted")}: ${selectedFile}. ${i18n.t("The game will generate the original.")}`, "success");
          csvDataMode = [];
          tableRows = [];
      } catch (e) {
          toastStore.add(i18n.t("Deletion failed"), "error");
      }
  }
</script>

<div class="content-wrapper">
  
  <div class="toolbar">
     <select bind:value={selectedFile} onchange={() => loadCsvFile(selectedFile)}>
       {#each availableFiles as fn}
         <option value={fn}>{fn}</option>
       {/each}
     </select>
     
     <input type="text" placeholder={i18n.t("Search key or translation...")} bind:value={searchQuery}>
     
     <label>
       <input type="checkbox" bind:checked={showOnlyQSelect}> {i18n.t("Show only recommended G-Keys (QSelect)")}
     </label>
     
      <button class="button restore-btn" onclick={restoreOriginal}>
          {i18n.t("Restore original (Delete local modification)")}
      </button>
   </div>

   <div class="table-container" onscroll={(e) => {
       const target = e.currentTarget;
       if (target.scrollTop + target.clientHeight >= target.scrollHeight - 100) {
           if (visibleLimit < tableRows.length) visibleLimit += 50;
       }
   }}>
     {#if tableRows.length === 0}
       <p>{i18n.t("No data loaded or filter found nothing...")}</p>
     {:else}
       <table>
         <thead>
           <tr>
             <th class="key-col">{i18n.t("Key")}</th>
             <th class="ref-col">{i18n.t("Original Reference (EN)")}</th>
             <th class="input-col">{i18n.t("Your Translation")} ({gameLang})</th>
           </tr>
         </thead>
         <tbody>
           {#each visibleRows as row (row.key)}
             <tr class:highlighted={row.inQSelect}>
               <td class="key-col">{row.key}</td>
               <td class="ref-col">{row.ref}</td>
               <td class="input-col">
                 <textarea use:autoResize value={row.target} 
                        onchange={(e) => saveCell(row, (e.currentTarget as HTMLTextAreaElement).value)} 
                        onkeydown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); e.currentTarget.blur(); } }} 
                        rows="1" spellcheck="false"></textarea>
               </td>
             </tr>
           {/each}
         </tbody>
       </table>
     {/if}
  </div>
</div>

<style>
  .content-wrapper {
    animation: fadein 0.3s;
    display: flex;
    flex-direction: column;
    gap: 20px;
    height: 100%;
    margin: 0;
    margin-top: 15px;
    padding: 0;
    box-sizing: border-box;
    overflow: hidden;
  }

  .toolbar {
    display: flex;
    gap: 15px;
    align-items: center;
    flex-wrap: wrap;
    flex-shrink: 0;
    margin-bottom: 25px;
  }
  
  .restore-btn {
      background: #c0392b;
      margin-left: auto;
  }
  
  .table-container {
    overflow-y: auto;
    overflow-x: hidden; 
    flex: 1;
    min-height: 0; 
    border: 1px solid var(--border-color);
    border-radius: 8px;
    background: var(--bg-color);
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
    table-layout: fixed; 
  }
  
  th.key-col { width: 25%; }
  th.ref-col { width: 35%; }
  th.input-col { width: 40%; }
  
  th, td {
      border: 1px solid var(--border-color);
      padding: 8px;
      word-wrap: break-word;
      overflow-wrap: break-word;
      white-space: pre-wrap;
  }
  
  th {
      background: var(--bg-color);
      position: sticky;
      top: 0;
      z-index: 10;
  }
  
  tr.highlighted td.key-col {
      color: #ffd700;
  }
  
  :global(.light) tr.highlighted td.key-col {
      color: #d97706;
      font-weight: 600;
  }

  .input-col textarea {
      width: 100%;
      background: transparent;
      border: 1px solid transparent;
      color: var(--text-color);
      padding: 5px;
      outline: none;
      resize: none;
      min-height: 24px;
      font-family: inherit;
      box-sizing: border-box;
      overflow: hidden;
  }

  .input-col textarea:focus {
      border-bottom: 1px solid #5b9bd5;
  }

  @keyframes fadein {
    from { opacity: 0; transform: translateY(5px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
