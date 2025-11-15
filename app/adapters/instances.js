export const instancesAdapter = (data) =>
  data.map(({ zone, ...inst }) => ({
    ...inst,
    zone: zone.split('/')[1],
  }));
