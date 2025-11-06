import { defineStore } from 'pinia';

const mapInstances = (data) =>
  data.map(({ zone, ...inst }) => ({
    ...inst,
    zone: zone.split('/')[1],
  }));

const errorMessage = (err, code = 418) => ({
  data: null,
  error: { code: code, status: 'Internal Error', message: err },
});

export const useInstancesStore = defineStore('Instances', () => {
  const localData = ref(null);

  function setInstances(newData) {
    localData.value = newData;
  }
  // function clearInstances() {
  //   localData.value = null;
  // }

  const {
    data,
    refresh: fetchInstances,
    error: errorInstances,
  } = useAsyncData(
    'instances',
    async () => {
      const response = await useRequest('instances_list');
      const newData = mapInstances(response);
      setInstances(newData);
      return newData;
    },
    { dedupe: 'cancel' },
  );

  const instances = computed(() => localData.value ?? data.value ?? null);

  function updateInstanceStatus(id, newStatus) {
    const list = localData.value ?? data.value;
    if (!list) return;

    const instance = list.find((i) => i.id === id);
    if (instance) {
      instance.status = newStatus;
      if (localData.value) {
        localData.value = [...localData.value];
      } else if (data.value) {
        data.value = [...data.value];
      }
    }
  }

  async function getInstance(instance, zone) {
    try {
      const updated = await useRequest('instance_get', instance, zone);

      if (localData.value) {
        const index = localData.value.findIndex((i) => i.id === updated.id);
        if (index !== -1) {
          localData.value[index] = updated;
        } else {
          localData.value.push(updated);
        }
      } else if (data.value) {
        const index = data.value.findIndex((i) => i.id === updated.id);
        if (index !== -1) {
          data.value[index] = updated;
        } else {
          data.value.push(updated);
        }
      } else {
        localData.value = [updated];
      }

      return updated;
    } catch (e) {
      console.error('Failed to refresh instance:', e);
      throw e;
    }
  }

  const isStarting = ref(false);
  const startInstance = async (instance, zone) => {
    if (isStarting.value) return errorMessage('Instance is starting');
    try {
      isStarting.value = true;
      let { data, error } = await useRequest('instance_start', instance, zone);
      return { data, error };
    } catch (e) {
      return errorMessage(e);
    } finally {
      isStarting.value = false;
    }
  };

  const isStopping = ref(false);
  const stopInstance = async (instance, zone) => {
    if (isStopping.value) return errorMessage('Instance is starting');
    try {
      isStopping.value = true;
      let { data, error } = await useRequest('instance_stop', instance, zone);
      return { data, error };
    } catch (e) {
      return errorMessage(e);
    } finally {
      isStopping.value = false;
    }
  };

  return {
    fetchInstances,
    startInstance,
    stopInstance,
    getInstance,
    updateInstanceStatus,
    instances,
    errorInstances,
  };
});
