import { useMantaStore } from '../_app';

import {
  Text,
  Divider,
  Center,
  Table,
  Box
} from '@mantine/core';
import { trpc } from '../../utils/trpc';
import { useEffect, useState } from 'react';
import { useSession } from "next-auth/react";

interface Payment {
  id: number,
  cid: number,
  mid: number,
  amount: string,
  receiver: string,
  sender: string,
  description: string
  ctime: string
  mtime: string
}

const Dashboard = () => {
  const id = useMantaStore((state) => state.id)
  const cookie = useMantaStore((state) => state.cookie)
  // const payments = trpc.payments.useQuery();
  const { status, data } = useSession();

  const setCookie = useMantaStore((state) => state.setCookie);
  if (data?.user?.image) {
    const cookie_str = data?.user?.image.toString();
    useEffect(() => {
      setCookie(cookie_str);
    }, [cookie_str])
  }

  const method = "list_payments";
  const uid = 1
  const rpc = trpc.list.useQuery({
    cookie: cookie,
    method: method,
    id: uid,
  });

  const send = useMantaStore((state) => state.send)

  const rows = rpc?.data?.payments?.result?.data?.map((element: Payment) => (
    <tr key={element.id}>
      <td>{`KES ${element.amount}`.replace(/\B(?=(\d{3})+(?!\d))/g, ",")}</td>
      <td>{element.receiver}</td>
      <td>{element.description}</td>
      <td>{element.ctime.toString().split('T')[1].slice(0, 5)}</td>
      <td>{element.ctime.toString().split('T')[0]}</td>
      <td>{id}</td>
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
              <th>AMOUNT</th>
              <th>RECEIVER</th>
              <th>DESCRIPTION</th>
              <th>TIME</th>
              <th>DATE</th>
              <th>SENDER ID</th>
            </tr>
          </thead>
          <tbody>{rows}</tbody>
        </Table>
      </Center>
    </>
  );
}
export default Dashboard;