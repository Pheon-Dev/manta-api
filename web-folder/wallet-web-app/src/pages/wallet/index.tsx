import { useSession } from 'next-auth/react';
import {
  Text,
  Avatar,
  Divider,
  Center,
  Title,
  Table,
  Card,
  Badge,
  Button,
  Modal,
  Group,
  SegmentedControl,
  Box
} from '@mantine/core';
import { IconCash, IconBuildingBank, IconSend } from '@tabler/icons-react';
import { useDisclosure } from '@mantine/hooks';
import { useMantaStore } from '../_app';
import { trpc} from '../../utils/trpc';
import Send from './Send';

const Wallet = () => {
  const balance = useMantaStore((state) => state.balance)
  const id = useMantaStore((state) => state.id)
  const email = useMantaStore((state) => state.email)
  const username = useMantaStore((state) => state.username)
  const name = useMantaStore((state) => state.name)
  const { status, data } = useSession();
  const account = trpc.account.accounts.useQuery({ method: "list_accounts", id: 1, cookie: `${data?.user?.image}` });

  const user = {
    image: "https://images.unsplash.com/photo-1508214751196-bcfd4ca60f91?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=255&q=80",
    name: name,
    email: email
  }
  const [opened_send, { open: open_send, close: close_send }] = useDisclosure(false);
  return (
    <>
      <Text
        variant="gradient"
        gradient={{ from: 'indigo', to: 'cyan', deg: 45 }}
        sx={{ fontFamily: 'Greycliff CF, sans-serif' }}
        ta="center" p="xs"
        fz="xl"
        fw={900}
      >
        {user.name}
      </Text>
      <Text
        variant="gradient"
        gradient={{ from: 'green', to: 'violet', deg: 45 }}
        sx={{ fontFamily: 'Greycliff CF, monospace' }}
        ta="center"
        fz="xs"
        fw={500}
      >
        {user.email}
      </Text>
      <Center mx="auto" maw={400} h={100}>
        <Avatar src={user.image} alt="profile picture" color="indigo" radius="xl" size="xl" />
      </Center>

      <Divider
        my="xs"
        variant="dashed"
        labelPosition="center"
        label={
          <>
            <Box ml={5}>
              <Text
                variant="gradient"
                gradient={{ from: 'indigo', to: 'cyan', deg: 45 }}
                sx={{ fontFamily: 'Greycliff CF, sans-serif' }}
                ta="center" p="xs"
                fz="xl"
                fw={900}
              >
                Services
              </Text>
            </Box>
          </>
        }
        labelProps={{ component: 'a', href: 'https://mantine.dev', variant: 'link', color: 'blue' }}
      />
      <Center maw={600} mx="auto">
        <SegmentedControl
          data={[
            {
              value: 'preview',
              label: (
                <Group>
                  <Modal opened={opened_send} onClose={close_send} title="Send Money" centered>
                    <Send />
                  </Modal>
                  <Center onClick={open_send}>
                    <IconSend size={16} />
                    <Box ml={10}>Send</Box>
                  </Center>
                </Group>
              ),
            },
            {
              value: 'code',
              label: (
                <Center>
                  <IconCash size={16} />
                  <Box ml={10}>Withdraw</Box>
                </Center>
              ),
            },
            {
              value: 'export',
              label: (
                <Center>
                  <IconBuildingBank size={16} />
                  <Box ml={10}>Deposit</Box>
                </Center>
              ),
            },
          ]}
        />
      </Center>
      <Center maw={600} h={300} mx="auto">
        <Card shadow="sm" padding="lg" radius="md" withBorder>

          <Group position="apart" w={400} mt="md" mb="md">
            <Text weight={500}>Account Balance</Text>
            <Badge color="blue" variant="light" size="lg">
              {`KES ${balance}`.replace(
                /\B(?=(\d{3})+(?!\d))/g,
                ","
              )}
            </Badge>
          </Group>
          <Divider />
          <Group position="apart" w={400} mt="md" mb="md">
            <Text weight={500}>Account ID</Text>
            <Badge color="blue" variant="light">
              {id}
            </Badge>
          </Group>
        </Card>
      </Center>
      <Center>
        <pre>{JSON.stringify(account.data, undefined, 2)}</pre>
      </Center>
    </>
  );
}

export default Wallet;
