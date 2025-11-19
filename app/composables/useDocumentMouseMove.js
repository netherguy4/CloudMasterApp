export function useDocumentMouseMove(callback) {
  const handler = (e) => callback(e);

  onMounted(() => {
    document.addEventListener('mousemove', handler);
  });

  onBeforeUnmount(() => {
    document.removeEventListener('mousemove', handler);
  });
}
