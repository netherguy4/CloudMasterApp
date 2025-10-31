import { invoke } from '@tauri-apps/api/core';

export async function initGcloud() {
  let message, error, data;

  try {
    data = await invoke('gcloud_init');
    message = 'Successfuli signed in';
  } catch (e) {
    error = 'Failed to initialize gcloud: ' + e;
  }

  return { message, error, data };
}
