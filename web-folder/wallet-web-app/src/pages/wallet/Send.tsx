import { useForm, isNotEmpty, isEmail, isInRange, hasLength, matches } from '@mantine/form';
import { Button, Group, TextInput, NumberInput, Box, Textarea } from '@mantine/core';
import { useMantaStore } from '../_app';
import { trpc } from '../../utils/trpc';
import { useCallback, useEffect } from 'react';
import { useSession } from 'next-auth/react';

const Send = () => {
  const cookie = useMantaStore((state) => state.cookie)
  const { status, data } = useSession();

  let name = ""
  const setCookie = useMantaStore((state) => state.setCookie);
  if (data?.user?.image) {
    const cookie_str = data?.user?.image.toString();
    useEffect(() => {
      setCookie(cookie_str);
    }, [cookie_str])
  }
  if (data?.user?.name) {
    name = data?.user?.name.toString();
  }

  const method = "create_payment";
  const uid = 1

  const form = useForm({
    initialValues: {
      sender: `${name}`,
      receiver: '',
      description: '',
      amount: 1,
    },

    validate: {
      sender: hasLength({ min: 2, max: 10 }, 'sender must be 2-10 characters long'),
      receiver: isNotEmpty('Enter your current receiver'),
      description: isNotEmpty('Enter your current receiver'),
      amount: isInRange({ min: 1, max: 99999 }, 'You must be 18-99 years old to register'),
    },
  });

  const send = useMantaStore((state) => state.send)

  const username = useMantaStore((state) => state.username)
  const send_money = trpc.send.useMutation({ onSuccess: async () => { return console.log("success") } });

  const handleSend = useCallback(() => {
    try {
      if (form.values.amount !== 0 && form.values.sender === username && form.values.receiver !== "" && form.values.description !== "") {
        send_money.mutate({
          cookie: cookie,
          method: method,
          id: uid,
          amount: `${form.values.amount}`,
          sender: form.values.sender,
          receiver: form.values.receiver,
          description: form.values.description
        })
        send(form.values.amount)
      }
    } catch (e) {
      console.log(e)
    }
  }, [send_money, form.values.amount, form.values.sender, form.values.receiver, form.values.description]);

  return (
    <Box component="form" maw={400} mx="auto" onSubmit={form.onSubmit(() => { })}>
      <NumberInput
        label="Enter amount"
        placeholder="Enter amount"
        withAsterisk
        mt="md"
        {...form.getInputProps('amount')}
      />
      <TextInput label="sender" placeholder="sender" withAsterisk {...form.getInputProps('sender')} />
      <TextInput
        label="Enter receiver"
        placeholder="Enter receiver"
        withAsterisk
        mt="md"
        {...form.getInputProps('receiver')}
      />
      <Textarea
        label="Enter description"
        placeholder="Enter description"
        withAsterisk
        mt="md"
        {...form.getInputProps('description')}
      />

      <Group position="right" mt="md">
        <Button type="submit" onClick={() => handleSend()}>Send</Button>
      </Group>
    </Box>
  );
}

export default Send;
