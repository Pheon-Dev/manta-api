import { notifications } from '@mantine/notifications';
import { useCallback } from 'react';
import { Button, Group, TextInput, NumberInput, Box, Textarea } from '@mantine/core';
import { useForm, isNotEmpty, isEmail, isInRange, hasLength, matches } from '@mantine/form';
import { IconCheck, IconX, IconCreditCard } from '@tabler/icons-react';
import { useSession } from 'next-auth/react';
import { trpc } from '../../utils/trpc';
import { NativeSelect } from '@mantine/core';

type Props = {
  username: string
}
const Withdraw = ({ username }: Props) => {
  const { status, data } = useSession();
  const name = data?.user?.name;
  const account = trpc.card.list.useQuery({ method: "list_cards", id: 1, cookie: `${data?.user?.image}` });

  const form = useForm({
    initialValues: {
      username: `${username}`,
      cnumber: '',
      amount: 0,
    },

    validate: {
      cnumber: hasLength({ min: 12, max: 12 }, 'Enter 12 Digit Card Number'),
      username: isNotEmpty('Card Owner should match the logged in user'),
      amount: isInRange({ min: 1, max: 100000 }, 'Maximum amount is 100000, minimum amount is 1'),
    },
  });

  const method = "get_card"
  const cookie = `${data?.user?.image}`

  const withdraw_money = trpc.account.withdraw.useMutation({
    onSuccess: async () => {
      return notifications.update({
        id: "withdraw",
        title: "Withdraw Money",
        color: "green",
        icon: <IconCheck />,
        autoClose: 5000,
        message: `New ${form.values.amount} withdrawn successfully`,
      });
    }
  });

  const handleSubmit = useCallback(() => {
    notifications.show({
      id: "withdraw",
      title: "Loading Withdraw Money",
      message: "Please wait...",
      loading: true,
    })
    try {
      if (
        form.values.cnumber !== "" &&
        form.values.amount !== 0 &&
        form.values.username !== ""
      ) {
        withdraw_money.mutate({
          cookie: cookie,
          method: method,
          id: 1,
          amount: form.values.amount,
          username: `${form.values.username}`,
          card_id: `${form.values.cnumber}`,
        })
        if (withdraw_money.isError) {
          notifications.update({
            id: "withdraw",
            title: "Withdraw Money",
            color: "red",
            icon: <IconX />,
            message: `Failed to create, please try again`,
          })
        }
      }

    } catch (error) {
      notifications.update({
        id: "withdraw",
        title: "Withdraw Money",
        color: "red",
        icon: <IconX />,
        message: `Error: ${error}`,
      })
    }
  }, [withdraw_money, form.values.username, form.values.amount, form.values.cnumber]);

  return (
    <Box component="form" maw={400} mx="auto" onSubmit={form.onSubmit(() => { })}>
      <NumberInput
        label="Enter amount"
        placeholder="Enter amount"
        withAsterisk
        mt="md"
        {...form.getInputProps('amount')}
      />
      <TextInput
        label="Enter Card Owner"
        placeholder="username"
        withAsterisk
        mt="md"
        {...form.getInputProps('username')}
      />
      <NativeSelect
        label="Pick a hashtag"
        placeholder="Pick a hashtag"
        data={['React', 'Angular', 'Svelte', 'Vue']}
        icon={<IconCreditCard size="1rem" />}
        {...form.getInputProps('cnumber')}
      />

      <Group position="right" mt="md">
        <Button type="submit" onClick={() => handleSubmit()}>Submit</Button>
      </Group>
    </Box>
  )
}

export default Withdraw
