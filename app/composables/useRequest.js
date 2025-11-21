import { invoke } from '@tauri-apps/api/core';

const projectId = 'leafy-container-473321-k7';

export const useRequest = async (command, arg1, arg2) => {
  switch (command) {
    case 'instances_list': {
      const data = await invoke('instances_list', {
        projectId,
      });
      return data;
    }
    case 'instance_get': {
      const data = await invoke('instance_get', {
        projectId,
        instance: arg1,
        zone: arg2,
      });
      return data;
    }
    case 'instance_start': {
      const data = await invoke('instance_start', {
        projectId,
        instance: arg1,
        zone: arg2,
      });
      return data;
    }
    case 'instance_stop': {
      const data = await invoke('instance_stop', {
        projectId,
        instance: arg1,
        zone: arg2,
      });
      return data;
    }
    case 'list_servers':
      await new Promise((resolve) => setTimeout(resolve, 1000));
      return [
        {
          name: 'minecraft-eu',
          status: 'RUNNING',
          port: '25565',
          preset: 'minecraft',
          vm: {
            name: 'vm-minecraft',
            id: '7824338635686187217',
            status: 'RUNNING',
            machine_type: 'e2-medium',
            internal_ips: ['10.156.0.12'],
            external_ips: ['34.185.100.42'],
            zone: 'europe-west3-c',
          },
        },
        {
          name: 'cs2-frankfurt',
          status: 'RUNNING',
          port: '27015',
          preset: 'steamcmd-cs2',
          vm: {
            name: 'vm-cs2',
            id: '3345882337651234411',
            status: 'RUNNING',
            machine_type: 'n2-standard-4',
            internal_ips: ['10.156.0.8'],
            external_ips: ['35.198.210.17'],
            zone: 'europe-west3-b',
          },
        },
        {
          name: 'valheim-nordic',
          status: 'TERMINATED',
          port: '2456',
          preset: 'steamcmd-valheim',
          vm: {
            name: 'vm-valheim',
            id: '9192338641236187217',
            status: 'TERMINATED',
            machine_type: 'e2-highmem-2',
            internal_ips: ['10.156.0.15'],
            external_ips: ['34.185.101.11'],
            zone: 'europe-west3-a',
          },
        },
        {
          name: 'ark-survival',
          status: 'RUNNING',
          port: '7777',
          preset: 'steamcmd-ark',
          vm: {
            name: 'vm-ark',
            id: '7824338639991187217',
            status: 'RUNNING',
            machine_type: 'n2-highmem-4',
            internal_ips: ['10.156.0.20'],
            external_ips: ['34.185.120.15'],
            zone: 'europe-west3-b',
          },
        },
        {
          name: 'rust-pve',
          status: 'RUNNING',
          port: '28015',
          preset: 'steamcmd-rust',
          vm: {
            name: 'vm-rust',
            id: '1824338635686187217',
            status: 'RUNNING',
            machine_type: 'e2-medium',
            internal_ips: ['10.156.0.21'],
            external_ips: ['34.185.120.16'],
            zone: 'europe-west3-c',
          },
        },
        {
          name: 'terraria-classic',
          status: 'TERMINATED',
          port: '7777',
          preset: 'steamcmd-terraria',
          vm: {
            name: 'vm-terraria',
            id: '4824338635686187217',
            status: 'TERMINATED',
            machine_type: 'e2-small',
            internal_ips: ['10.156.0.25'],
            external_ips: ['34.185.122.33'],
            zone: 'europe-west3-a',
          },
        },
        {
          name: 'factorio-industrial',
          status: 'RUNNING',
          port: '34197',
          preset: 'steamcmd-factorio',
          vm: {
            name: 'vm-factorio',
            id: '5524338635686187217',
            status: 'RUNNING',
            machine_type: 'c2-standard-4',
            internal_ips: ['10.156.0.5'],
            external_ips: ['34.185.130.50'],
            zone: 'europe-west3-c',
          },
        },
        {
          name: 'palworld-beta',
          status: 'REPAIRING',
          port: '8211',
          preset: 'steamcmd-palworld',
          vm: {
            name: 'vm-palworld',
            id: '6624338635686187217',
            status: 'REPAIRING',
            machine_type: 'e2-micro',
            internal_ips: ['10.156.0.30'],
            external_ips: ['34.185.125.10'],
            zone: 'europe-west3-b',
          },
        },
        {
          name: 'dayz-expedition',
          status: 'TERMINATED',
          port: '2302',
          preset: 'steamcmd-dayz',
          vm: {
            name: 'vm-dayz',
            id: '7324338635686187217',
            status: 'TERMINATED',
            machine_type: 'n2-standard-2',
            internal_ips: ['10.156.0.35'],
            external_ips: ['34.185.140.12'],
            zone: 'europe-west3-a',
          },
        },
        {
          name: '7-days-2-die-europe',
          status: 'RUNNING',
          port: '26900',
          preset: 'steamcmd-7days2die',
          vm: {
            name: 'vm-7days2die',
            id: '9924338635686187217',
            status: 'RUNNING',
            machine_type: 'c2-standard-8',
            internal_ips: ['10.156.0.40'],
            external_ips: ['34.185.150.99'],
            zone: 'europe-west3-c',
          },
        },
        {
          name: 'minecraft-eu',
          status: 'RUNNING',
          port: '25565',
          preset: 'minecraft',
          vm: {
            name: 'vm-minecraft',
            id: '7824338635686187217',
            status: 'RUNNING',
            machine_type: 'e2-medium',
            internal_ips: ['10.156.0.12'],
            external_ips: ['34.185.100.42'],
            zone: 'europe-west3-c',
          },
        },
        {
          name: 'cs2-frankfurt',
          status: 'RUNNING',
          port: '27015',
          preset: 'steamcmd-cs2',
          vm: {
            name: 'vm-cs2',
            id: '3345882337651234411',
            status: 'RUNNING',
            machine_type: 'n2-standard-4',
            internal_ips: ['10.156.0.8'],
            external_ips: ['35.198.210.17'],
            zone: 'europe-west3-b',
          },
        },
        {
          name: 'valheim-nordic',
          status: 'TERMINATED',
          port: '2456',
          preset: 'steamcmd-valheim',
          vm: {
            name: 'vm-valheim',
            id: '9192338641236187217',
            status: 'TERMINATED',
            machine_type: 'e2-highmem-2',
            internal_ips: ['10.156.0.15'],
            external_ips: ['34.185.101.11'],
            zone: 'europe-west3-a',
          },
        },
        {
          name: 'ark-survival',
          status: 'RUNNING',
          port: '7777',
          preset: 'steamcmd-ark',
          vm: {
            name: 'vm-ark',
            id: '7824338639991187217',
            status: 'RUNNING',
            machine_type: 'n2-highmem-4',
            internal_ips: ['10.156.0.20'],
            external_ips: ['34.185.120.15'],
            zone: 'europe-west3-b',
          },
        },
        {
          name: 'rust-pve',
          status: 'RUNNING',
          port: '28015',
          preset: 'steamcmd-rust',
          vm: {
            name: 'vm-rust',
            id: '1824338635686187217',
            status: 'RUNNING',
            machine_type: 'e2-medium',
            internal_ips: ['10.156.0.21'],
            external_ips: ['34.185.120.16'],
            zone: 'europe-west3-c',
          },
        },
        {
          name: 'terraria-classic',
          status: 'TERMINATED',
          port: '7777',
          preset: 'steamcmd-terraria',
          vm: {
            name: 'vm-terraria',
            id: '4824338635686187217',
            status: 'TERMINATED',
            machine_type: 'e2-small',
            internal_ips: ['10.156.0.25'],
            external_ips: ['34.185.122.33'],
            zone: 'europe-west3-a',
          },
        },
        {
          name: 'factorio-industrial',
          status: 'RUNNING',
          port: '34197',
          preset: 'steamcmd-factorio',
          vm: {
            name: 'vm-factorio',
            id: '5524338635686187217',
            status: 'RUNNING',
            machine_type: 'c2-standard-4',
            internal_ips: ['10.156.0.5'],
            external_ips: ['34.185.130.50'],
            zone: 'europe-west3-c',
          },
        },
        {
          name: 'palworld-beta',
          status: 'REPAIRING',
          port: '8211',
          preset: 'steamcmd-palworld',
          vm: {
            name: 'vm-palworld',
            id: '6624338635686187217',
            status: 'REPAIRING',
            machine_type: 'e2-micro',
            internal_ips: ['10.156.0.30'],
            external_ips: ['34.185.125.10'],
            zone: 'europe-west3-b',
          },
        },
        {
          name: 'dayz-expedition',
          status: 'TERMINATED',
          port: '2302',
          preset: 'steamcmd-dayz',
          vm: {
            name: 'vm-dayz',
            id: '7324338635686187217',
            status: 'TERMINATED',
            machine_type: 'n2-standard-2',
            internal_ips: ['10.156.0.35'],
            external_ips: ['34.185.140.12'],
            zone: 'europe-west3-a',
          },
        },
        {
          name: '7-days-2-die-europe',
          status: 'RUNNING',
          port: '26900',
          preset: 'steamcmd-7days2die',
          vm: {
            name: 'vm-7days2die',
            id: '9924338635686187217',
            status: 'RUNNING',
            machine_type: 'c2-standard-8',
            internal_ips: ['10.156.0.40'],
            external_ips: ['34.185.150.99'],
            zone: 'europe-west3-c',
          },
        },
      ];

    default:
      return {
        status: 'error',
        code: 404,
        message: 'Nothing found for the given command',
      };
  }
};
