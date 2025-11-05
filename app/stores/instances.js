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
  const instances = ref([]);

  const fetchInstances = async () => {
    try {
      let { data, error } = await useRequest('instances_list');
      if (error) return { data, error };
      data = mapInstances(data);
      instances.value = data;
      return { data, error };
    } catch (e) {
      return errorMessage(e);
    }
  };

  const getInstance = async (instance, zone) => {
    try {
      let { data, error } = await useRequest('instance_get', instance, zone);
      if (error) return { data, error };
      data = mapInstances(data);
      return { data, error };
    } catch (e) {
      return errorMessage(e);
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
  };
});
