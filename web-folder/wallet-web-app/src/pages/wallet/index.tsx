import { useSession, signOut } from 'next-auth/react';
import {
  Text,
  Avatar,
  Divider,
  Center,
  Title,
  Drawer,
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
import Send from './Send';
import NewContact from './Contact';
import NewCard from './Card';
import Deposit from './Deposit';
import Withdraw from './Withdraw';
import { trpc } from '../../utils/trpc';
import { useMantaStore } from '../_app';
import { useEffect } from 'react';

interface Account {
  email: string,
  balance: string,
  id: number,
  username: string,
  aid: string,
}
const Wallet = () => {
  const { status, data } = useSession();
  // const id = useMantaStore((state) => state.id)
  const setID = useMantaStore((state) => state.setID)
  const name = data?.user?.name;
  const account = trpc.account.list.useQuery({ method: "list_accounts", id: 1, cookie: `${data?.user?.image}` });
  const res = account?.data?.data?.result?.data.find((account: Account) => account.email === data?.user?.email);

  useEffect(() => {
    let sub = true
    if (sub) { setID(res?.cid) }
    return () => { sub = false }
  }, [res?.cid])

  const user = {
    image: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTvc_M0Jo569OkceaKbg_bobTRQGfzXwYEWYgVi8DTwiw&s",
    name: name,
    username: res?.username,
    aid: res?.aid,
    id: res?.id,
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
        <Avatar src={user.image} alt="profile picture" color="indigo" radius="lg" size="xl" />
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
        labelProps={{ component: 'a', href: '#', variant: 'link', color: 'blue' }}
      />
      <Center maw={600} mx="auto">
        <SegmentedControl
          data={[
            {
              value: 'send',
              label: (
                <Group>
                  <Drawer opened={opened_send} onClose={close_send} title="Send Money" overlayProps={{ opacity: 0.5, blur: 1 }}>
                    <Send username={user.username} id={user.id} balance={user.balance} />
                  </Drawer>
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
                  <Drawer opened={opened_withdraw} onClose={close_withdraw} title="Withdraw Money" overlayProps={{ opacity: 0.5, blur: 1 }}>
                    <Withdraw username={user.username} id={user.id} balance={user.balance} />
                  </Drawer>
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
                  <Drawer opened={opened_deposit} onClose={close_deposit} title="Deposit Money" overlayProps={{ opacity: 0.5, blur: 1 }} >
                    <Deposit username={user.username} id={user.id} balance={user.balance} />
                  </Drawer>
                  <Center onClick={open_deposit}>
                    <IconBuildingBank size={16} />
                    <Box ml={10}>Deposit</Box>
                  </Center>
                </Group>
              ),
            },
          ]}
        />
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
                Actions
              </Text>
            </Box>
          </>
        }
        labelProps={{ component: 'a', href: '#', variant: 'link', color: 'blue' }}
      />
      <Center maw={600} mx="auto">
        <SegmentedControl
          data={[
            {
              value: 'card',
              label: (
                <Group>
                  <Drawer opened={opened_new_card} onClose={close_new_card} title="Add a new card" overlayProps={{ opacity: 0.5, blur: 1 }} position="right">
                    <NewCard username={user.username} />
                  </Drawer>
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
                  <Drawer opened={opened_new_contact} onClose={close_new_contact} title="Add a new contact" overlayProps={{ opacity: 0.5, blur: 1 }} position="right">
                    <NewContact username={user.username} />
                  </Drawer>
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
      <Center maw={600} h={200} mx="auto">
        <Card shadow="sm" padding="lg" radius="md" withBorder>

          <Group position="apart" w={400} mt="md" mb="md">
            <Text weight={500}>Account Balance</Text>
            <Badge color="blue" variant="light" size="lg">
              {`KES ${user.balance}`.replace(/\B(?=(\d{3})+(?!\d))/g, ",")}
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
      <pre>{JSON.stringify(account.data, undefined, 2)}</pre>
    </>
  );
}

export default Wallet;
