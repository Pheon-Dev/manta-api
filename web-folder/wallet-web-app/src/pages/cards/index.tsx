import { Card, Image, Text, Badge, Button, Group, Grid } from '@mantine/core';
import { useSession } from 'next-auth/react';
import { trpc } from '../../utils/trpc';

const Cards = () => {
  const { status, data } = useSession();
  const name = data?.user?.name;
  const account = trpc.card.list.useQuery({ method: "list_cards", id: 1, cookie: `${data?.user?.image}` });
  const res = account?.data?.data?.result?.data[0]
  return (
    <Grid>
      <Grid.Col span={4}>
        <Card shadow="sm" padding="lg" radius="md" withBorder>
          <Group position="apart" mt="md" mb="xs">
            <Text weight={500}>M-PESA</Text>
            <Badge color="violet" variant="light">
              DEBIT
            </Badge>
          </Group>

          <Text size="sm" color="dimmed">
            M-PESA Global Pay Card
          </Text>

          <Button variant="light" color="blue" fullWidth mt="md" radius="md">
            4597-XXXX-XXXX-XXXX-XXXX
          </Button>
        </Card>
      </Grid.Col>
      <Grid.Col span={4}>
        <Card shadow="sm" padding="lg" radius="md" withBorder>
          <Group position="apart" mt="md" mb="xs">
            <Text weight={500}>Master Card</Text>
            <Badge color="pink" variant="light">
              CREDIT
            </Badge>
          </Group>

          <Text size="sm" color="dimmed">
            Master Card Credit Card
          </Text>

          <Button variant="light" color="blue" fullWidth mt="md" radius="md">
            4499-XXXX-XXXX-XXXX-XXXX
          </Button>
        </Card>
      </Grid.Col>
      <Grid.Col span={4}>
        <Card shadow="sm" padding="lg" radius="md" withBorder>
          <Group position="apart" mt="md" mb="xs">
            <Text weight={500}>VISA Card</Text>
            <Badge color="violet" variant="light">
              DEBIT
            </Badge>
          </Group>

          <Text size="sm" color="dimmed">
            VISA Debit Card
          </Text>

          <Button variant="light" color="blue" fullWidth mt="md" radius="md">
            4897-XXXX-XXXX-XXXX-XXXX
          </Button>
        </Card>
      </Grid.Col>
      <Grid.Col span={4}>
        <Card shadow="sm" padding="lg" radius="md" withBorder>
          <Group position="apart" mt="md" mb="xs">
            <Text weight={500}>KCB</Text>
            <Badge color="violet" variant="light">
              DEBIT
            </Badge>
          </Group>

          <Text size="sm" color="dimmed">
            Kenya Commercial Bank Debit Card
          </Text>

          <Button variant="light" color="blue" fullWidth mt="md" radius="md">
            4327-XXXX-XXXX-XXXX-XXXX
          </Button>
        </Card>
      </Grid.Col>
        <pre>{JSON.stringify(account.data, undefined, 2)}</pre>
    </Grid>
  )
}
export default Cards;
