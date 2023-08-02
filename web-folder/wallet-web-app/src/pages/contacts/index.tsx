import { Card, Image, Text, Badge, Button, Group, Grid } from '@mantine/core';
import { useSession } from 'next-auth/react';
import { trpc } from '../../utils/trpc';

interface Contact {
  username: string,
  ref_id: string,
  association: string,
  email: string,
  name: string,
}

const Contacts = () => {
  const { status, data } = useSession();
  const name = data?.user?.name;
  const account = trpc.contact.list.useQuery({ method: "list_contacts", id: 1, cookie: `${data?.user?.image}` });
  const res = account?.data?.response?.result?.data
  return (
    <Grid>
      {res && res.map((contact: Contact) => (
        <>
          <Grid.Col span={4}>
            <Card shadow="sm" padding="lg" radius="md" withBorder>
              <Group position="apart" mt="md" mb="xs">
                <Text weight={500}>{contact.username}</Text>
                <Badge color="violet" variant="light">
                  {contact.ref_id}
                </Badge>
              </Group>

              <Button variant="gradient" gradient={{ from: 'indigo', to: 'green', deg: 45 }} fullWidth mt="md" radius="md">
                {contact.name}
              </Button>
              <Text size="sm" color="dimmed">
                {contact.email}
              </Text>
            </Card>
          </Grid.Col>
        </>
      )) || null}
    </Grid>
  )
}
export default Contacts;

