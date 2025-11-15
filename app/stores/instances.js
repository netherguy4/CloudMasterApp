import { defineStore } from 'pinia';
import { instancesAdapter } from '@/adapters/instances';

export const useInstancesStore = defineStore('Instances', () => {
  const instances = ref(null);
  const isStarting = ref(false);
  const isStopping = ref(false);

  const updateListItem = (list, freshItem) => {
    const index = list.findIndex((i) => i.id === freshItem.id);
    if (index !== -1) {
      list[index] = freshItem;
    } else {
      list.push(freshItem);
    }
  };

  const fetchInstances = async () => {
    try {
      const response = await useRequest('instances_list');
      instances.value = instancesAdapter(response);
      return instances.value;
    } catch (error) {
      throw error;
    }
  };

  const updateInstanceStatus = (id, newStatus) => {
    if (!instances.value) return;

    const instanceIndex = instances.value.findIndex?.((i) => i.id === id);
    if (instanceIndex >= 0) {
      instances.value[instanceIndex].status = newStatus;
    }
  };

  const getInstance = async (instance, zone) => {
    try {
      const freshItem = await useRequest('instance_get', instance, zone);

      if (instances.value) {
        updateListItem(instances.value, freshItem);
      } else {
        instances.value[0] = freshItem;
      }

      return freshItem;
    } catch (e) {
      throw e;
    }
  };

  const startInstance = async (instance, zone) => {
    if (isStarting.value) throw new Error('Instance is starting');

    try {
      isStarting.value = true;
      return await useRequest('instance_start', instance, zone);
    } catch (e) {
      throw e;
    } finally {
      isStarting.value = false;
    }
  };

  const stopInstance = async (instance, zone) => {
    if (isStopping.value) throw new Error('Instance is stopping');

    try {
      isStopping.value = true;
      return await useRequest('instance_stop', instance, zone);
    } catch (e) {
      throw e;
    } finally {
      isStopping.value = false;
    }
  };

  return {
    instances,
    fetchInstances,
    startInstance,
    stopInstance,
    getInstance,
    updateInstanceStatus,
  };
});
