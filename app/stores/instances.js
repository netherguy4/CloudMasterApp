import { defineStore } from 'pinia';

const mapInstances = (data) =>
  data.map(({ zone, ...inst }) => ({
    ...inst,
    zone: zone.split('/')[1],
  }));

const errorMessage = (err, code = 418) => ({
  data: null,
  error: { code: code, message: err },
});

export const useInstancesStore = defineStore('Instances', () => {
  const loading = ref(false);
  const instances = ref([]);

  const fetchInstances = async () => {
    try {
      if (!instances.value.length) loading.value = true;
      let { data, error } = await useRequest('instances_list');
      data = mapInstances(data);
      instances.value = data;
      return { data, error };
    } catch (e) {
      return errorMessage(e);
    } finally {
      loading.value = false;
    }
  };

  const getInstance = async (instance, zone) => {
    try {
      if (!instances.value.length) loading.value = true;
      let { data, error } = await useRequest('instance_get', instance, zone);
      data = mapInstances(data);
      return { data, error };
    } catch (e) {
      return errorMessage(e);
    } finally {
      loading.value = false;
    }
  };

  const startInstance = async (instance, zone) => {
    try {
      let { data, error } = await useRequest('instance_start', instance, zone);
      return { data, error };
    } catch (e) {
      return errorMessage(e);
    }
  };

  const stopInstance = async (instance, zone) => {
    try {
      let { data, error } = await useRequest('instance_stop', instance, zone);
      return { data, error };
    } catch (e) {
      return errorMessage(e);
    }
  };

  return {
    fetchInstances,
    startInstance,
    stopInstance,
    getInstance,
    instances,
    loading,
  };
});
