export const useRequest = async (command) => {
  await new Promise((resolve) => setTimeout(resolve, 300));

  switch (command) {
    case 'list':
      return {
        status: 'success',
        code: 200,
        data: [
          {
            name: 'minecraft-eu',
            id: '7824338635686187217',
            status: 'RUNNING',
            machine_type: 'e2-medium',
            internal_ips: ['10.156.0.12'],
            external_ips: ['34.185.100.42'],
            zone: 'zones/europe-west3-c',
          },
          {
            name: 'cs2-frankfurt',
            id: '3345882337651234411',
            status: 'RUNNING',
            machine_type: 'n2-standard-4',
            internal_ips: ['10.156.0.8'],
            external_ips: ['35.198.210.17'],
            zone: 'zones/europe-west3-b',
          },
          {
            name: 'valheim-nordic',
            id: '9192338641236187217',
            status: 'TERMINATED',
            machine_type: 'e2-highmem-2',
            internal_ips: ['10.156.0.15'],
            external_ips: ['34.185.101.11'],
            zone: 'zones/europe-west3-a',
          },
          {
            name: 'ark-survival',
            id: '7824338639991187217',
            status: 'RUNNING',
            machine_type: 'n2-highmem-4',
            internal_ips: ['10.156.0.20'],
            external_ips: ['34.185.120.15'],
            zone: 'zones/europe-west3-b',
          },
          {
            name: 'rust-pve',
            id: '1824338635686187217',
            status: 'RUNNING',
            machine_type: 'e2-medium',
            internal_ips: ['10.156.0.21'],
            external_ips: ['34.185.120.16'],
            zone: 'zones/europe-west3-c',
          },
          {
            name: 'terraria-classic',
            id: '4824338635686187217',
            status: 'TERMINATED',
            machine_type: 'e2-small',
            internal_ips: ['10.156.0.25'],
            external_ips: ['34.185.122.33'],
            zone: 'zones/europe-west3-a',
          },
          {
            name: 'factorio-industrial',
            id: '5524338635686187217',
            status: 'RUNNING',
            machine_type: 'c2-standard-4',
            internal_ips: ['10.156.0.5'],
            external_ips: ['34.185.130.50'],
            zone: 'zones/europe-west3-c',
          },
          {
            name: 'palworld-beta',
            id: '6624338635686187217',
            status: 'RUNNING',
            machine_type: 'e2-micro',
            internal_ips: ['10.156.0.30'],
            external_ips: ['34.185.125.10'],
            zone: 'zones/europe-west3-b',
          },
          {
            name: 'dayz-expedition',
            id: '7324338635686187217',
            status: 'TERMINATED',
            machine_type: 'n2-standard-2',
            internal_ips: ['10.156.0.35'],
            external_ips: ['34.185.140.12'],
            zone: 'zones/europe-west3-a',
          },
          {
            name: '7-days-2-die-europe',
            id: '9924338635686187217',
            status: 'RUNNING',
            machine_type: 'c2-standard-8',
            internal_ips: ['10.156.0.40'],
            external_ips: ['34.185.150.99'],
            zone: 'zones/europe-west3-c',
          },
        ],
      };

    default:
      return {
        status: 'error',
        code: 404,
        message: 'No instances found for the given command',
      };
  }
};
