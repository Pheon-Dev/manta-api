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
}

const Deposit = ({ username }: Props) => {
  const { status, data } = useSession();
  const name = data?.user?.name;
  const account = trpc.card.list.useQuery({ method: "list_cards", id: 1, cookie: `${data?.user?.image}` });
  const cards = account?.data?.data?.result?.data && account?.data?.data?.result?.data?.map((card: Card) => `${card.id}: ${card.cname} [${card.cnumber.slice(0, 4)} **** ****]`) || []

  const card_idefault = cards[0] && cards[0].split(":")[0] || ""
  const form = useForm({
    initialValues: {
      cname: `${cards[0]}`,
      amount: 1,
    },

    validate: {
      cname: isNotEmpty('Select one of the cards'),
      amount: isInRange({ min: 1, max: 100000 }, 'Maximum amount is 100000, minimum amount is 1'),
    },
  });

  const method = "get_card"
  const cookie = `${data?.user?.image}`

  const deposit_money = trpc.account.deposit.useMutation({
    onSuccess: async () => {
      return notifications.update({
        id: "deposit",
        title: "Deposit Money",
        color: "green",
        icon: <IconCheck />,
        autoClose: 5000,
        message: `New ${form.values.amount} deposited successfully`,
      });
    }
  });

  // console.log(form.values.cname.split(":")[0])
  const card_id = form.values.cname.split(":")[0] && form.values.cname.split(":")[0] || card_idefault
  // console.log(cards[0])
  // const card = trpc.card.get.useQuery({ method: "get_card", id: 1, cookie: `${data?.user?.image}`, cid: `${card_id}` });
  // console.log(card?.data?.data?.result?.data?.cnumber)

  const handleSubmit = useCallback(() => {
    notifications.show({
      id: "deposit",
      title: "Loading Deposit Money",
      message: "Please wait...",
      loading: true,
    })
    try {
      if (
        form.values.amount !== 0 &&
        card_id !== "" &&
        username
      ) {
        deposit_money.mutate({
          cookie: cookie,
          method: method,
          id: 1,
          amount: form.values.amount,
          username: `${username}`,
          card_id: `${card_id}`,
        })
        if (deposit_money.isError) {
          notifications.update({
            id: "deposit",
            title: "Deposit Money",
            color: "red",
            icon: <IconX />,
            message: `Failed to create, please try again`,
          })
        }
      }

    } catch (error) {
      notifications.update({
        id: "deposit",
        title: "Deposit Money",
        color: "red",
        icon: <IconX />,
        message: `Error: ${error}`,
      })
    }
  }, [deposit_money, username, form.values.amount, card_id]);

  return (
    <Box component="form" maw={400} mx="auto" onSubmit={form.onSubmit(() => { })}>
      <NumberInput
        label="Enter amount"
        placeholder="Enter amount"
        withAsterisk
        mt="md"
        {...form.getInputProps('amount')}
      />
      <NativeSelect
        label="Pick a card"
        placeholder="Pick a card"
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

export default Deposit
