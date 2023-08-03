import { notifications } from '@mantine/notifications';
import { useCallback, useEffect } from 'react';
import { Button, Group, TextInput, NumberInput, Box, Textarea } from '@mantine/core';
import { useForm, isNotEmpty, isEmail, isInRange, hasLength, matches } from '@mantine/form';
import { IconCheck, IconX, IconCreditCard, IconInfoCircle } from '@tabler/icons-react';
import { useSession } from 'next-auth/react';
import { trpc } from '../../utils/trpc';
import { Select } from '@mantine/core';
import { useMantaStore } from '../_app';

type Props = {
  username: string
  id: number
  balance: string
}

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

const Withdraw = ({ username, id, balance }: Props) => {
  const cid = useMantaStore((state) => state.id)
  const { status, data } = useSession();
  const account = trpc.card.list.useQuery({ method: "list_cards", id: 1, cookie: `${data?.user?.image}` });
  const cards = account?.data?.data?.result?.data && account?.data?.data?.result?.data?.map((card: Card) => {
    return {
      value: card.id,
      label: `[KES ${card.cbalance.replace(/\B(?=(\d{3})+(?!\d))/g, ",")}]: ${card.cname} (${card.caccount})`,
      hidden: card.cid !== cid,
    }
  }) || []

  const form = useForm({
    initialValues: {
      cname: 0,
      amount: 1,
    },

    validate: {
      cname: isNotEmpty('Select one of the cards'),
      amount: isInRange({ min: 1, max: 100000 }, 'Maximum amount is 100000, minimum amount is 1'),
    },
  });

  const method = "get_card"
  const cookie = `${data?.user?.image}`

  const utils = trpc.useContext();
  const withdraw_money = trpc.account.withdraw.useMutation({
    onSuccess: async () => {
      await utils.account.invalidate();
      await utils.card.invalidate();
      return notifications.update({
        id: "withdraw",
        title: "Withdraw Money",
        color: "green",
        icon: <IconCheck />,
        autoClose: 5000,
        message: `New ${form.values.amount} withdrawed successfully`,
      });
    }
  });

  useEffect(() => {
    let sub = true
    if (sub) {
      notifications.update({
        id: "withdraw",
        title: "Withdraw Money",
        color: "blue",
        icon: <IconInfoCircle />,
        autoClose: 5000,
        message: `${withdraw_money?.data?.message}`,
      })
    }
    return () => { sub = false }

  }, [withdraw_money?.data?.message])

  const handleSubmit = useCallback(() => {
    notifications.show({
      id: "withdraw",
      title: "Loading Withdraw Money",
      message: "Please wait...",
      loading: true,
    })
    try {
      if (
        form.values.amount !== 0 &&
        form.values.cname !== 0 &&
        balance !== "" &&
        id !== 0 &&
        username
      ) {
        if (form.values.amount > +balance || +balance < 1) {
          return notifications.update({
            id: "withdraw",
            color: "red",
            icon: <IconX />,
            title: "Withdraw Money",
            message: `Insufficient balance`,
          })
        }
        withdraw_money.mutate({
          cookie: cookie,
          method: method,
          id: 1,
          amount: form.values.amount,
          username: `${username}`,
          acc_id: id,
          card_id: form.values.cname,
          balance: `${balance}`,
        })
        if (withdraw_money.isError) {
          notifications.update({
            id: "withdraw",
            title: "Withdraw Money",
            color: "red",
            icon: <IconX />,
            message: `Failed to withdraw, please try again after checking your connection`,
          })
        }
      }
      notifications.update({
        id: "withdraw",
        title: "Withdraw Money",
        color: "red",
        icon: <IconX />,
        message: `Something went wrong, please try again`,
      })

    } catch (error) {
      notifications.update({
        id: "withdraw",
        title: "Withdraw Money",
        color: "red",
        icon: <IconX />,
        message: `Error: ${error}`,
      })
    }
  }, [withdraw_money, username, form.values.amount, balance, id, form.values.cname]);

  return (
    <Box component="form" mx="auto" onSubmit={form.onSubmit(() => { })}>
      <NumberInput
        label="Enter amount"
        placeholder="Enter amount"
        withAsterisk
        mt="md"
        {...form.getInputProps('amount')}
      />
      <Select
        label="Pick a card"
        placeholder="Pick a card"
        searchable
        maxDropdownHeight={400}
        nothingFound="No results found"
        data={cards}
        icon={<IconCreditCard size="1rem" />}
        {...form.getInputProps('cname')}
      />

      <Group position="right" mt="md">
        <Button type="submit" onClick={() => handleSubmit()}>Submit</Button>
      </Group>
    </Box>
  )
}

export default Withdraw
