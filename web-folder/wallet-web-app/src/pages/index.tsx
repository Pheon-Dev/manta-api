import { trpc } from '../utils/trpc';
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
  Group,
  SegmentedControl,
  Box
} from '@mantine/core';
import { IconCash, IconBuildingBank, IconSend } from '@tabler/icons-react';
const HomePage = () => {
  const user = {
    image: "https://images.unsplash.com/photo-1508214751196-bcfd4ca60f91?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=255&q=80",
    name: "Harriette Spoonlicker",
    email: "hspoonlicker@outlook.com"
  }
  // const { data: login, fetchStatus: fetch_login_status } = trpc.login.useQuery({ username: 'demo1', password: "welcome" });
  const payments = trpc.payments.useQuery();

  if (!payments) {
    return <div>Loading...</div>;
  }
  // if (!login) {
  //   return <div>Loading...</div>;
  // }
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
                <Center>
                  <IconBuildingBank size={16} />
                  <Box ml={10}>Deposit</Box>
                </Center>
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
                  <IconSend size={16} />
                  <Box ml={10}>Send</Box>
                </Center>
              ),
            },
          ]}
        />
      </Center>
      <Center maw={600} h={300} mx="auto">
        <Card shadow="sm" padding="lg" radius="md" withBorder>

          <Group position="apart" w={400} mt="md" mb="md">
            <Text weight={500}>Token Balance</Text>
            <Badge color="blue" variant="light">
              Tks. 364.00
            </Badge>
          </Group>
          <Divider />
          <Group position="apart" w={400} mt="md" mb="md">
            <Text weight={500}>In Local Currency</Text>
            <Badge color="green" variant="light">
              KSHs. 23,634.00
            </Badge>
          </Group>
        </Card>
      </Center>
      <pre>{JSON.stringify(payments.data, undefined, 2)}</pre>
    </>
  );
}

export default HomePage;
