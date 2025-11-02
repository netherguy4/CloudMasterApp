import { defineStore } from 'pinia';

export const useInstancesStore = defineStore('Instances', () => {
  const instances = ref();
  const fetchInstances = async () => {
    try {
      let { data, code, message } = await useRequest('list');
      if (code !== 200 && message) console.log(message);
      instances.value = data;
    } catch (e) {
      console.log(e);
    }
  };

  return { fetchInstances, instances };
});
