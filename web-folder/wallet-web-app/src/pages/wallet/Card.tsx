import { notifications } from '@mantine/notifications';
import { useCallback } from 'react';
import { Select, Button, Group, TextInput, NumberInput, Box, Textarea } from '@mantine/core';
import { useForm, isNotEmpty, isEmail, isInRange, hasLength, matches } from '@mantine/form';
import { IconCheck, IconX } from '@tabler/icons-react';
import { useSession } from 'next-auth/react';
import { trpc } from '../../utils/trpc';

type Props = {
  username: string
}
const NewCard = ({ username }: Props) => {
  const { status, data } = useSession();

  const form = useForm({
    initialValues: {
      cbalance: 0,
      cowner: `${username}`,
      cnumber: '',
      cdescription: '',
      cvalid: '',
      cvv: "",
      cname: "",
      ctype: "",
      caccount: "",
    },

    validate: {
      cnumber: hasLength({ min: 12, max: 12 }, 'Enter 12 Digit Card Number'),
      cowner: isNotEmpty('Card Owner should match the logged in user'),
      cvv: isNotEmpty('Enter Card CVV number'),
      cdescription: isNotEmpty('Enter Card Description'),
      cvalid: hasLength({ min: 5, max: 5 }, 'Enter Card Validity Date [DD/YY]'),
      cname: isNotEmpty('Enter Card Name [M-PESA | KCB Card]'),
      cbalance: isInRange({ min: 1, max: 100000 }, 'Maximum amount is 100000, minimum amount is 1'),
    },
  });

  const method = "create_card"
  const cookie = `${data?.user?.image}`

  const create_card = trpc.card.create.useMutation({
    onSuccess: async () => {
      return notifications.update({
        id: "create-card",
        title: "New Card",
        color: "green",
        icon: <IconCheck />,
        autoClose: 5000,
        message: `New ${form.values.cname} created successfully`,
      });
    }
  });

  const handleSubmit = useCallback(() => {
    notifications.show({
      id: "create-card",
      title: "Loading New Card",
      message: "Please wait...",
      loading: true,
    })
    try {
      if (
        form.values.cbalance !== 0 &&
        form.values.cowner !== "" &&
        form.values.cnumber !== "" &&
        form.values.cdescription !== "" &&
        form.values.cvalid !== "" &&
        form.values.cvv !== "" &&
        form.values.cname !== "" &&
        form.values.ctype !== "" &&
        form.values.caccount !== ""
      ) {
        create_card.mutate({
          cookie: cookie,
          method: method,
          id: 1,
          cdescription: form.values.cdescription,
          cnumber: form.values.cnumber,
          cowner: form.values.cowner,
          cvv: form.values.cvv,
          cvalid: form.values.cvalid,
          cname: form.values.cname,
          ctype: form.values.ctype,
          caccount: form.values.caccount,
          cbalance: `${form.values.cbalance}`
        })
        if (create_card.isError) {
          notifications.update({
            id: "create-card",
            title: "New Card",
            color: "red",
            icon: <IconX />,
            message: `Failed to create, please try again`,
          })
        }
      }

    } catch (error) {
      notifications.update({
        id: "create-card",
        title: "New Card",
        color: "red",
        icon: <IconX />,
        message: `Error: ${error}`,
      })
    }
  }, [create_card, form.values.cbalance, form.values.cnumber, form.values.cowner, form.values.cdescription, form.values.cvalid, form.values.cvv, form.values.cname, form.values.ctype, form.values.caccount]);

  return (
    <Box component="form" maw={400} mx="auto" onSubmit={form.onSubmit(() => { })}>
      <TextInput
        label="Enter Card Name"
        placeholder="ABC Bank"
        withAsterisk
        mt="md"
        {...form.getInputProps('cname')}
      />
      <TextInput
        label="Enter Card Owner"
        placeholder="username"
        disabled
        withAsterisk
        mt="md"
        {...form.getInputProps('cowner')}
      />
      <NumberInput
        label="Enter Initial Deposit [1-100,000]"
        placeholder="000000"
        withAsterisk
        mt="md"
        {...form.getInputProps('cbalance')}
      />
      <Select
        label="Enter Card Account Type"
        placeholder="Debit | Credit"
        withAsterisk
        mt="md"
        data={[
          { value: 'debit', label: 'Debit' },
          { value: 'credit', label: 'Credit' },
        ]}
        {...form.getInputProps('caccount')}
      />
      <Select
        label="Enter Card Type"
        placeholder="VISA | Master Card"
        withAsterisk
        data={[
          { value: 'VISA', label: 'VISA' },
          { value: 'MasterCard', label: 'Master Card' },
          { value: 'UnionPay', label: 'Union Pay', disabled: true },
        ]}
        mt="md"
        {...form.getInputProps('ctype')}
      />
      <TextInput
        label="Enter Card Validity Period"
        placeholder="DD/YY"
        maxLength={5}
        minLength={5}
        withAsterisk
        mt="md"
        {...form.getInputProps('cvalid')}
      />
      <TextInput
        label="Enter Card CVV"
        placeholder="XXX"
        maxLength={3}
        minLength={3}
        withAsterisk
        mt="md"
        {...form.getInputProps('cvv')}
      />
      <TextInput
        label="Enter Card Number"
        maxLength={12}
        minLength={12}
        placeholder="0000000000000000"
        withAsterisk
        mt="md"
        {...form.getInputProps('cnumber')}
      />
      <Textarea
        label="Enter Card Description"
        placeholder="ABC Bank Savings Account"
        withAsterisk
        mt="md"
        {...form.getInputProps('cdescription')}
      />

      <Group position="left" mt="md">
        <Button type="submit" onClick={() => handleSubmit()}>Submit</Button>
      </Group>
    </Box>
  )
}

export default NewCard
