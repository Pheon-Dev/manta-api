import { notifications } from '@mantine/notifications';
import { useCallback  } from 'react';
import { Button, Group, TextInput, NumberInput, Box, Textarea } from '@mantine/core';
import { useForm, isNotEmpty, isEmail, isInRange, hasLength, matches } from '@mantine/form';
import { IconCheck, IconX } from '@tabler/icons-react';
import { useSession } from 'next-auth/react';
import { trpc } from '../../utils/trpc';

const NewCard = () => {
  const { status, data } = useSession();

  const form = useForm({
    initialValues: {
      cbalance: 0,
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
      cvv: isNotEmpty('Enter Card CVV number'),
      cdescription: isNotEmpty('Enter Card Description'),
      cvalid: isNotEmpty('Enter Card Validity Date [DD/YY]'),
      ctype: isNotEmpty('Enter Card Type [VISA | Master Card]'),
      cname: isNotEmpty('Enter Card Name [M-PESA | KCB Card]'),
      caccount: isNotEmpty('Enter Card Account [Debit | Credit]'),
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
  }, [create_card, form.values.cbalance, form.values.cnumber, form.values.cdescription, form.values.cvalid, form.values.cvv, form.values.cname, form.values.ctype, form.values.caccount]);

  return (
    <Box component="form" maw={400} mx="auto" onSubmit={form.onSubmit(() => { })}>
      <TextInput
        label="Enter Card Name"
        placeholder="ABC Bank"
        withAsterisk
        mt="md"
        {...form.getInputProps('cname')}
      />
      <NumberInput
        label="Enter Initial Deposit [1-100,000]"
        placeholder="000000"
        withAsterisk
        mt="md"
        {...form.getInputProps('cbalance')}
      />
      <TextInput
        label="Enter Card Account Type"
        placeholder="Debit | Credit"
        withAsterisk
        mt="md"
        {...form.getInputProps('caccount')}
      />
      <TextInput
        label="Enter Card Type"
        placeholder="VISA | Master Card"
        withAsterisk
        mt="md"
        {...form.getInputProps('ctype')}
      />
      <TextInput
        label="Enter Card Validity Period"
        placeholder="DD/YY"
        withAsterisk
        mt="md"
        {...form.getInputProps('cvalid')}
      />
      <TextInput
        label="Enter Card CVV"
        placeholder="XXX"
        withAsterisk
        mt="md"
        {...form.getInputProps('cvv')}
      />
      <TextInput
        label="Enter Card Number"
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

      <Group position="right" mt="md">
        <Button type="submit" onClick={() => handleSubmit()}>Submit</Button>
      </Group>
    </Box>
  )
}

export default NewCard
