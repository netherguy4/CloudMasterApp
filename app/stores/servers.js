import { defineStore } from 'pinia';

export const useServersStore = defineStore('Servers', () => {
  const servers = ref();
  const fetchServers = async () => {
    try {
      let { data, code, message } = await useRequest('list_servers');
      if (code !== 200 && message) console.log(message);
      servers.value = data;
    } catch (e) {
      console.log(e);
    }
  };

  return { fetchServers, servers };
});
