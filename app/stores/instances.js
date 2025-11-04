import { defineStore } from 'pinia'

export const useInstancesStore = defineStore('Instances', () => {
  const instances = ref()
  const error = ref()
  const loading = ref()

  const fetchInstances = async () => {
    try {
      error.value = false
      loading.value = true

      let { data, code, message } = await useRequest('list_instances')
      if (code !== 200 && message) {
        error.value = message
        return
      }
      instances.value = data
    }
    catch (e) {
      error.value = e
    }
    finally {
      loading.value = false
    }
  }

  return { fetchInstances, instances, error, loading }
})
