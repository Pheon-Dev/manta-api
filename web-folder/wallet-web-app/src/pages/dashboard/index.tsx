// import { invoke } from "@tauri-apps/api/tauri";
import {
  Text,
  Divider,
  Center,
  Table,
  Box
} from '@mantine/core';
import { trpc } from '../../utils/trpc';
import { useState } from 'react';

interface Payment {
    id: number,
    cid: number,
    amount: string,
    receiver: string,
    sender: string,
    description: string
  }

const Dashboard = () => {
  const payments = trpc.payments.useQuery();

  if (!payments) {
    return <div>Loading...</div>;
  }
  const rows = payments?.data?.payments.map((element: Payment) => (
    <tr key={element.cid}>
      <td>{element.id}</td>
      <td>{element.amount}</td>
      <td>{element.receiver}</td>
      <td>{element.sender}</td>
      <td>{element.description}</td>
    </tr>
  ));

  return (
    <>
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
                Recent Transactions
              </Text>
            </Box>
          </>
        }
        labelProps={{ component: 'a', href: 'https://mantine.dev', variant: 'link', color: 'blue' }}
      />
      <Center maw={900} mx="auto" mt="xs">
        <Table horizontalSpacing="xs">
          <thead>
            <tr>
              <th>id</th>
              <th>amount</th>
              <th>sender</th>
              <th>receiver</th>
              <th>description</th>
            </tr>
          </thead>
          <tbody>{rows}</tbody>
        </Table>
      </Center>
      <Center>
        <pre>{JSON.stringify(payments.data, undefined, 2)}</pre>
      </Center>
    </>
  );
}
export default Dashboard;
