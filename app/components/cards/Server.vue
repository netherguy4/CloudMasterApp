<script setup>
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

const props = defineProps({
  name: {
    type: String,
    default: '',
  },
  preset: {
    type: String,
    default: '',
  },
  port: {
    type: String,
    default: '',
  },
  vm: {
    type: Object,
    default: () => ({}),
  },
  status: {
    type: String,
    default: '',
  },
});

const mouseOver = ref(false);
const copied = ref(false);
const error = ref(false);

const trueIP = computed(() => props.vm?.external_ips?.[0] + ':' + props.port);

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
  if (status === 'running') return { text: 'Stop' };
  if (status === 'terminated') return { text: 'Start' };
  return { text: 'Busy' };
});

const parts = computed(() => [
  { label: 'Name:', value: props.name },
  ip.value,
  { label: 'Preset:', value: props.preset },
  { label: 'Instance Name:', value: props.vm?.name },
]);
</script>

<template>
  <UiUniversalCard :parts="parts" :ip="ip" :button="button" :status="status" />
</template>
