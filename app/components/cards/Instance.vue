<script setup>
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

const props = defineProps({
  id: {
    type: String,
    default: '',
  },
  name: {
    type: String,
    default: '',
  },
  status: {
    type: String,
    default: '',
  },
  external_ips: {
    type: Array,
    default: () => [],
  },
  zone: {
    type: String,
    default: '',
  },
});

const store = useInstancesStore();

const mouseOver = ref(false);
const copied = ref(false);
const error = ref(false);

const trueIP = computed(() => props.external_ips?.[0]);

function onHoverIn() {
  mouseOver.value = true;
}

function onHoverOut() {
  mouseOver.value = false;
  copied.value = false;
  error.value = false;
}

async function onClick() {
  try {
    await writeText(trueIP.value);
    copied.value = true;
    setTimeout(() => (copied.value = false), 1000);
  } catch {
    error.value = true;
    setTimeout(() => (error.value = false), 1000);
  }
}

const ip = computed(() => {
  const label = mouseOver.value ? 'Copy IP:' : 'IP:';
  const value = error.value ? 'ERROR' : copied.value ? 'COPIED' : trueIP.value;

  return {
    label,
    value,
    trueValue: trueIP.value,
    onHoverIn,
    onHoverOut,
    onClick,
  };
});

const button = computed(() => {
  const status = props.status.toLowerCase();
  if (status === 'running') return { text: 'Stop', click: stop };
  if (status === 'terminated') return { text: 'Start', click: start };
  return { text: 'Busy', click: () => {} };

  async function stop() {
    store.updateInstanceStatus(props.id, 'STOPPING');
    const { error } = await store.stopInstance(props.id, props.zone);
    if (error) store.updateInstanceStatus(props.id, 'RUNNING');
  }

  async function start() {
    store.updateInstanceStatus(props.id, 'STAGING');
    const { error } = await store.startInstance(props.id, props.zone);
    if (error) store.updateInstanceStatus(props.id, 'TERMINATED');
  }
});

const parts = computed(() => [
  { label: 'Name:', value: props.name },
  ip.value,
  { label: 'Zone:', value: props.zone },
]);
</script>

<template>
  <UiUniversalCard :parts="parts" :ip="ip" :button="button" :status="status" />
</template>
