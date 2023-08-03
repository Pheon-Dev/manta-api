import { useForm, isNotEmpty, isEmail, isInRange, hasLength, matches } from '@mantine/form';
import { Button, Select, Group, TextInput, NumberInput, Box, Textarea } from '@mantine/core';
// import { useMantaStore } from '../_app';
import { notifications } from '@mantine/notifications';
import { trpc } from '../../utils/trpc';
import { useCallback } from 'react';
import { useSession } from 'next-auth/react';
import { IconCheck, IconX } from '@tabler/icons-react';
import { useMantaStore } from '../_app';

interface Contact {
  username: string,
  ref_id: string,
  association: string,
  email: string,
  name: string,
  cid: number,
  id: number,
}

type Props = {
  username: string
  id: number
  balance: string
}
const Send = ({ username, id, balance }: Props) => {
  const { status, data } = useSession();
  const cid = useMantaStore((state) => state.id)
  const contacts_data = trpc.contact.list.useQuery({ method: "list_contacts", id: 1, cookie: `${data?.user?.image}` });
  const contacts = contacts_data?.data?.response?.result?.data && contacts_data?.data?.response?.result?.data?.map((contact: Contact) => {
    return {
      value: `${contact.id} ${contact.username}`,
      label: `[ID ${contact.ref_id.toUpperCase()}] - ${contact.name}`,
      hidden: contact.cid !== cid,
    }
  }) || []

  const form = useForm({
    initialValues: {
      sender: `${username}`,
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

  const cookie = `${data?.user?.image}`
  const method = "create_payment";

  const utils = trpc.useContext();
  const send_money = trpc.payment.send.useMutation({
    onSuccess: async () => {
      await utils.payment.invalidate();
      await utils.account.invalidate();
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
      if (form.values.amount !== 0 && form.values.sender === username && form.values.receiver !== "" && form.values.description !== "" && id !== 0) {
        const amount = form.values.amount
        const new_balance = Number(balance) - amount

        if (amount > +balance || new_balance < 1) {
          return notifications.update({
            id: "send",
            color: "red",
            icon: <IconX />,
            title: "Send Money",
            message: `Insufficient balance`,
          })
        }
        send_money.mutate({
          cookie: cookie,
          method: method,
          balance: balance,
          id: id,
          amount: `${form.values.amount}`,
          receiver: `${form.values.receiver}`,
          sender: form.values.sender,
          description: form.values.description
        })
        // send(form.values.amount)
      }
    } catch (error) {
      notifications.update({
        id: "send",
        color: "red",
        icon: <IconX />,
        title: "Send Money",
        message: `Error: ${error}`,
      })
    }
  }, [send_money, form.values.amount, form.values.sender, form.values.receiver, form.values.description, id, balance, cookie, method, username]);

  return (
    <Box component="form" maw={400} mx="auto" onSubmit={form.onSubmit(() => { })}>
      <NumberInput
        label="Enter amount"
        placeholder="Enter amount"
        withAsterisk
        mt="md"
        {...form.getInputProps('amount')}
      />
      <TextInput label="sender" placeholder="sender" withAsterisk {...form.getInputProps('sender')} disabled />
      <Select
        label="Enter receiver"
        placeholder="Enter receiver"
        data={contacts}
        searchable
        maxDropdownHeight={400}
        nothingFound="No results found"
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
