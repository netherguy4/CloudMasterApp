import { defineStore } from 'pinia';

export const useServersStore = defineStore('Servers', () => {
  const servers = ref();

  const fetchServers = async () => {
    try {
      const response = await useRequest('list_servers');
      servers.value = response;
      return servers.value;
    } catch (error) {
      throw error;
    }
  };

  return { fetchServers, servers };
});
