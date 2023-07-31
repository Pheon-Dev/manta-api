import { Grid, Avatar, Container, Text } from '@mantine/core';
import { IconBolt, IconCar, IconCash, IconCheckbox, IconNews, IconShoppingBag } from '@tabler/icons-react';
import Link from "next/link";

const Apps = () => {
  return (
    <Container p="xl">
      <Grid>
        <Grid.Col span={4}>
          <Avatar color="blue" radius="xs" size="xl" m="md" component={Link} href="/apps/task">
            <IconCheckbox size="3.5rem" />
          </Avatar>
          <Text size="md" weight={500} m="xs">
            Task Manager
          </Text>
        </Grid.Col>
        <Grid.Col span={4}>
          <Avatar color="yellow" radius="xs" size="xl" m="md" component={Link} href="/apps/kplc">
            <IconBolt size="3.5rem" />
          </Avatar>
          <Text size="md" weight={500} m="xs">
            KPLC App
          </Text>
        </Grid.Col>
        <Grid.Col span={4}>
          <Avatar color="green" radius="xs" size="xl" m="md" component={Link} href="/apps/taxi">
            <IconCar size="3.5rem" />
          </Avatar>
          <Text size="md" weight={500} m="xs">
            Taxi Mini
          </Text>
        </Grid.Col>
        <Grid.Col span={4}>
          <Avatar color="pink" radius="xs" size="xl" m="md" component={Link} href="/apps/soko">
            <IconShoppingBag size="3.5rem" />
          </Avatar>
          <Text size="md" weight={500} m="xs">
            Soko Bei Poa
          </Text>
        </Grid.Col>
        <Grid.Col span={4}>
          <Avatar color="violet" radius="xs" size="xl" m="md" component={Link} href="/apps/loan">
            <IconCash size="3.5rem" />
          </Avatar>
          <Text size="md" weight={500} m="xs">
            Loan Manager
          </Text>
        </Grid.Col>
        <Grid.Col span={4}>
          <Avatar color="gray" radius="xs" size="xl" m="md" component={Link} href="/apps/news">
            <IconNews size="3.5rem" />
          </Avatar>
          <Text size="md" weight={500} m="xs">
            Recent News
          </Text>
        </Grid.Col>
      </Grid>
    </Container>
  )
}

export default Apps
