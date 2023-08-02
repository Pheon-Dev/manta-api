import { notifications } from '@mantine/notifications';
import { useCallback, useEffect } from 'react';
import { Button, Group, TextInput, NumberInput, Box, Textarea } from '@mantine/core';
import { useForm, isNotEmpty, isEmail, isInRange, hasLength, matches } from '@mantine/form';
import { IconCheck, IconX, IconUser, IconInfoCircle } from '@tabler/icons-react';
import { useSession } from 'next-auth/react';
import { trpc } from '../../utils/trpc';
import { Select } from '@mantine/core';

type Props = {
  username: string
}
const Contact = ({ username }: Props) => {
  const { status, data } = useSession();
  const name = data?.user?.name;
  const account = trpc.contact.list.useQuery({ method: "list_contacts", id: 1, cookie: `${data?.user?.image}` });

  const form = useForm({
    initialValues: {
      ref_id: '',
      association: '',
      username: '',
      email: '',
      name: '',
    },

    validate: {
      ref_id: hasLength({ min: 8, max: 8 }, 'Name must be 8 characters long'),
      association: isNotEmpty('Enter your association with the contact'),
      username: isNotEmpty('Enter contact Manta user name'),
      name: isNotEmpty('Enter contact Name'),
      email: isEmail('Invalid email'),
    },
  });

  const method = "create_contact"
  const cookie = `${data?.user?.image}`

  const utils = trpc.useContext();
  const new_user = trpc.contact.create.useMutation({
    onSuccess: async () => {
      await utils.contact.invalidate();
      return notifications.update({
        id: "contact",
        title: "New Contact",
        color: "green",
        icon: <IconCheck />,
        autoClose: 5000,
        message: ``,
      });
    }
  });

  useEffect(() => {
      let sub = true
      if (sub) {
          notifications.update({
              id: "deposit",
              title: "Deposit Money",
              color: "blue",
              icon: <IconInfoCircle />,
              autoClose: 5000,
              message: `${new_user?.data?.message}`,
            })
        }
        return () => {sub = false}

    }, [new_user?.data?.message])
  const handleSubmit = useCallback(() => {
    notifications.show({
      id: "contact",
      title: "Loading New Contact",
      message: "Please wait...",
      loading: true,
    })
    try {
      if (
        form.values.name !== "" &&
        form.values.ref_id !== "" &&
        form.values.association !== "" &&
        form.values.email !== "" &&
        form.values.username !== ""
      ) {
        new_user.mutate({
          cookie: cookie,
          method: method,
          id: 1,
          username: `${form.values.username}`,
          name: `${form.values.name}`,
          association: `${form.values.association}`,
          ref_id: `${form.values.ref_id}`,
          email: `${form.values.email}`,
        })
        if (new_user.isError) {
          return notifications.update({
            id: "contact",
            title: "New Contact",
            color: "red",
            icon: <IconX />,
            message: `Failed to create, please try again`,
          })
        }
        if (new_user.isSuccess) {
          return notifications.update({
            id: "contact",
            title: "New Contact",
            color: "green",
            icon: <IconCheck />,
            autoClose: 5000,
            message: ``,
          })
        }
      }

    } catch (error) {
      notifications.update({
        id: "contact",
        title: "New Contact",
        color: "red",
        icon: <IconX />,
        message: `Error: ${error}`,
      })
    }
  }, [new_user, form.values.username, form.values.association, form.values.name, form.values.email, form.values.ref_id]);

  return (
    <Box component="form" maw={400} mx="auto" onSubmit={form.onSubmit(() => { })}>
      <TextInput
        label="Enter Contact Username"
        placeholder="username"
        withAsterisk
        mt="md"
        {...form.getInputProps('username')}
      />
      <TextInput
        label="Enter Contact Name"
        placeholder="name"
        withAsterisk
        mt="md"
        {...form.getInputProps('name')}
      />
      <TextInput
        label="Enter Contact Manta Reference ID"
        placeholder="XXXXXXXX"
        maxLength={8}
        minLength={8}
        withAsterisk
        mt="md"
        {...form.getInputProps('ref_id')}
      />
      <TextInput
        label="Enter Contact Email"
        placeholder="email"
        withAsterisk
        mt="md"
        {...form.getInputProps('email')}
      />
      <TextInput
        label="Enter Association with Contact "
        placeholder="association"
        withAsterisk
        mt="md"
        {...form.getInputProps('association')}
      />

      <Group position="left" mt="md">
        <Button type="submit" onClick={() => handleSubmit()}>Submit</Button>
      </Group>
    </Box>
  )
}

export default Contact
