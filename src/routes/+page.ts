import { invoke } from '@tauri-apps/api/tauri';

export async function load() {
   return await invoke('fetch_data', {shift:0}) 
}
