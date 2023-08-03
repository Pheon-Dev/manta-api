import { Card, Image, Text, Badge, Button, Group, Grid } from '@mantine/core';
import { useSession } from 'next-auth/react';
import { trpc } from '../../utils/trpc';
import { useMantaStore } from '../_app';

interface Card {
  cowner: string,
  cname: string,
  cbalance: string,
  ctype: string,
  caccount: string,
  cnumber: string,
  cvv: string,
  cvalid: string,
  cdescription: string,
  id: number,
  cid: number,
}

interface Account {
email: string,
balance: string,
id: number,
username: string,
aid: string,
}
const Cards = () => {
  const { status, data } = useSession();
  const id = useMantaStore((state) => state.id)
  const account = trpc.card.list.useQuery({ method: "list_cards", id: 1, cookie: `${data?.user?.image}` });
  const res = account?.data?.data?.result?.data

  return (
    <Grid>
      {res && res.map((card: Card) => card.cid === id && (
        <Grid.Col span={4} key={card.id}>
          <Card shadow="sm" padding="lg" radius="md" withBorder>
            <Group position="apart" mt="md" mb="xs">
              <Text weight={500}>{card.cname}</Text>
              <Badge color="violet" variant="light">
                {`KES ${card.cbalance}`.replace(/\B(?=(\d{3})+(?!\d))/g, ",")}
              </Badge>
            </Group>

            <Button variant="light" color="blue" fullWidth mt="md" radius="md">
              {card.cnumber.slice(0, 4)} **** **** ****
            </Button>
            <Text size="sm" color="dimmed">
              {card.cdescription}
            </Text>
            <Group position="apart">
              <Text size="sm" fs="italic" weight={100}>VALID: {card.cvalid}</Text>
              <Text fs="regular" size="sm" weight={100}>CVV: {card.cvv}</Text>
            </Group>
            <Group position="right">
              <Badge color="pink" variant="light">
                <Text size="sm" fz="xs" weight={500} fs="italic">{card.caccount}</Text>
              </Badge>
            </Group>

          </Card>
        </Grid.Col>
      )) || null}
    </Grid>
  )
}
export default Cards;
