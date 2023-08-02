import { useForm, isNotEmpty, isEmail, isInRange, hasLength, matches } from '@mantine/form';
import { Button, Group, TextInput, NumberInput, Box, Textarea } from '@mantine/core';
// import { useMantaStore } from '../_app';
import { notifications } from '@mantine/notifications';
import { trpc } from '../../utils/trpc';
import { useCallback  } from 'react';
import { useSession } from 'next-auth/react';
import { IconCheck, IconX } from '@tabler/icons-react';

const Send = () => {
  const { status, data } = useSession();

  const form = useForm({
    initialValues: {
      sender: `${name}`,
      receiver: '',
      description: '',
      amount: 1,
    },

    validate: {
      sender: hasLength({ min: 2, max: 10 }, 'Sender must be 2-10 characters long'),
      receiver: isNotEmpty('Enter receiver'),
      description: isNotEmpty('Enter decription'),
      amount: isInRange({ min: 1, max: 100000 }, 'Maximum amount is 100000, minimum amount is 1'),
    },
  });

  // const send = useMantaStore((state) => state.send)

  const username = `${data?.user?.name}`
  const cookie = `${data?.user?.image}`
  const method = "create_payment";
  const uid = 1

  const send_money = trpc.payment.send.useMutation({
    onSuccess: async () => {
      return notifications.update({
        id: "send",
        color: "green",
        icon: <IconCheck />,
        title: "Sendin Money",
        autoClose: 5000,
        message: `KES ${form.values.amount} sent Successfully to: ${form.values.receiver}.\n Continue to send money or Press the X button to exit.`,
      });
    }
  });

  const handleSubmit = useCallback(() => {
    notifications.show({
      id: "send",
      title: "Loading",
      message: "Please wait...",
      loading: true,
    })
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
        // send(form.values.amount)
      }
    } catch (error) {
      notifications.update({
        id: "send",
        color: "red",
        icon: <IconX />,
        title: "Authentication",
        message: `Error: ${error}`,
      })
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
        <Button type="submit" onClick={() => handleSubmit()}>Send</Button>
      </Group>
    </Box>
  );
}

export default Send;
