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
import { IconCash, IconBuildingBank, IconSend, IconCreditCard, IconUser } from '@tabler/icons-react';
import { useDisclosure } from '@mantine/hooks';
import { trpc } from '../../utils/trpc';
import Send from './Send';
import NewContact from './Contact';
import NewCard from './Card';
import Deposit from './Deposit';
import Withdraw from './Withdraw';

const Wallet = () => {
  const { status, data } = useSession();
  const name = data?.user?.name;
  const account = trpc.account.list.useQuery({ method: "list_accounts", id: 1, cookie: `${data?.user?.image}` });
  const res = account?.data?.data?.result?.data[0]

  const user = {
    image: "https://images.unsplash.com/photo-1508214751196-bcfd4ca60f91?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=255&q=80",
    name: name,
    aid: res?.aid,
    balance: res?.balance,
    email: res?.email
  }
  const [opened_send, { open: open_send, close: close_send }] = useDisclosure(false);
  const [opened_withdraw, { open: open_withdraw, close: close_withdraw }] = useDisclosure(false);
  const [opened_deposit, { open: open_deposit, close: close_deposit }] = useDisclosure(false);
  const [opened_new_card, { open: open_new_card, close: close_new_card }] = useDisclosure(false);
  const [opened_new_contact, { open: open_new_contact, close: close_new_contact }] = useDisclosure(false);
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
              value: 'send',
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
              value: 'withdraw',
              label: (
                <Group>
                  <Modal opened={opened_withdraw} onClose={close_withdraw} title="Withdraw Money" centered>
                    <Withdraw />
                  </Modal>
                  <Center onClick={open_withdraw}>
                    <IconCash size={16} />
                    <Box ml={10}>Withdraw</Box>
                  </Center>
                </Group>
              ),
            },
            {
              value: 'deposit',
              label: (
                <Group>
                  <Modal opened={opened_deposit} onClose={close_deposit} title="Deposit Money" centered>
                    <Deposit />
                  </Modal>
                  <Center onClick={open_deposit}>
                    <IconBuildingBank size={16} />
                    <Box ml={10}>Deposit</Box>
                  </Center>
                </Group>
              ),
            },
            {
              value: 'card',
              label: (
                <Group>
                  <Modal opened={opened_new_card} onClose={close_new_card} title="Add a new card" centered>
                    <NewCard />
                  </Modal>
                  <Center onClick={open_new_card}>
                    <IconCreditCard size={16} />
                    <Box ml={10}>New Card</Box>
                  </Center>
                </Group>
              ),
            },
            {
              value: 'contact',
              label: (
                <Group>
                  <Modal opened={opened_new_contact} onClose={close_new_contact} title="Add a new card" centered>
                    <NewContact />
                  </Modal>
                  <Center onClick={open_new_contact}>
                  <IconUser size={16} />
                  <Box ml={10}>New Contact</Box>
                </Center>
                </Group>
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
              {`KES ${user.balance}`.replace(
                /\B(?=(\d{3})+(?!\d))/g,
                ","
              )}
            </Badge>
          </Group>
          <Divider />
          <Group position="apart" w={400} mt="md" mb="md">
            <Text weight={500}>Account ID</Text>
            <Badge color="blue" variant="light">
              {user.aid}
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
