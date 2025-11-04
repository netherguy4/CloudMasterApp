import { defineStore } from 'pinia'

export const useServersStore = defineStore('Servers', () => {
  const servers = ref()
  const error = ref()
  const loading = ref()

  const fetchServers = async () => {
    try {
      error.value = false
      loading.value = true

      let { data, code, message } = await useRequest('list_servers')
      if (code !== 200 && message) {
        error.value = message
        return
      }
      servers.value = data.map(serv => ({
        id: serv.name + serv.vm?.id,
        ...serv,
      }))
    }
    catch (e) {
      error.value = e
    }
    finally {
      loading.value = false
    }
  }

  return { fetchServers, servers, error, loading }
})
