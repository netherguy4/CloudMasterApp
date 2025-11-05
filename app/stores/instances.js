import { defineStore } from 'pinia';

export const useInstancesStore = defineStore('Instances', () => {
  const instances = ref();
  const error = ref();
  const loading = ref();

  const fetchInstances = async () => {
    try {
      error.value = false;
      loading.value = true;
      let { data, code, message } = await useRequest('instances_list');
      if (code !== 200 && message) {
        error.value = message;
        return;
      }
      data = data.map(({ zone, ...inst }) => ({
        ...inst,
        zone: zone.split('/')[1],
      }));
      instances.value = data;
    } catch (e) {
      error.value = e;
    } finally {
      loading.value = false;
    }
  };

  const startInstance = async (instance, zone) => {
    try {
      let { code, message } = await useRequest(
        'instance_start',
        instance,
        zone,
      );
      if (code === 200) {
        return 'Instance start scedulled successfuly';
      } else if (message) return message;
      else return 'Something went wrong';
    } catch (e) {
      return `Something went wrong: ${e}`;
    }
  };

  const stopInstance = async (instance, zone) => {
    try {
      let { code, message } = await useRequest('instance_stop', instance, zone);
      if (code === 200) {
        return 'Instance stop scedulled successfuly';
      } else if (message) return message;
      else return 'Something went wrong';
    } catch (e) {
      return `Something went wrong: ${e}`;
    }
  };

  const getInstance = async (instance, zone) => {
    try {
      let { data, code, message } = await useRequest(
        'instance_get',
        instance,
        zone,
      );
      if (code === 200 && data) return data;
      else if (message) return message;
      else return 'Something went wrong';
    } catch (e) {
      return `Something went wrong: ${e}`;
    }
  };

  return {
    fetchInstances,
    startInstance,
    stopInstance,
    getInstance,
    instances,
    error,
    loading,
  };
});
