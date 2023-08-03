import { notifications } from '@mantine/notifications';
import { useCallback, useEffect } from 'react';
import { Button, Group, TextInput, NumberInput, Box, Textarea, PasswordInput } from '@mantine/core';
import { useForm, isNotEmpty, isEmail, isInRange, hasLength, matches } from '@mantine/form';
import { IconCheck, IconX, IconUser, IconInfoCircle } from '@tabler/icons-react';
import { useSession } from 'next-auth/react';
import { trpc } from '../utils/trpc';

const NewUser = () => {
  const { status, data } = useSession();
  const name = data?.user?.name;
  const account = trpc.user.list.useQuery({ method: "list_users", id: 1, cookie: `${data?.user?.image}` });

  const form = useForm({
    initialValues: {
      password_clear: '',
      password_confirm: '',
      username: '',
      email: '',
    },

    validate: {
      password_clear: hasLength({ min: 8, max: 16 }, 'Name must be 8-16 characters long'),
      password_confirm: (value, values) =>
        value !== values.password_clear ? 'Passwords did not match' : null,
      username: isNotEmpty('Enter user Manta user name'),
      email: isEmail('Invalid email'),
    },
  });

  const method = "create_user"
  const cookie = `${data?.user?.image}`

  const utils = trpc.useContext();
  const new_user = trpc.user.create.useMutation({
    onSuccess: async () => {
      await utils.user.invalidate();
      return notifications.update({
        id: "user",
        title: "New New User",
        color: "green",
        icon: <IconCheck />,
        autoClose: 5000,
        message: ``,
      });
    }
  });

  useEffect(() => {
    let subscribe = true;
    if (subscribe) {
      if (
        +form.values.password_clear / 0 === 0 ||
        (+form.values.password_clear / 1 === +form.values.password_clear &&
          form.values.password_confirm.length > 0)
      ) {
        form.setFieldError("password_clear", "Please include letters | symbols");
      }
      let counter = 0;
      let nums = 0;
      while (counter < form.values.password_clear.length) {
        let f = form.values.password_clear[counter];
        if (f) {
          if (!isNaN(+f)) {
            nums += 1;
          }
        }
        counter++;
      }
      if (nums < 1 && form.values.password_confirm.length > 0)
        form.setFieldError("password_clear", "Please Include Numbers");

      if (form.values.password_clear.length < 9 && form.values.password_confirm.length > 0) {
        form.setFieldError(
          "password_clear",
          "password_clear length should be greater than 8"
        );
      }
    }
    return () => {
      subscribe = false;
    };
  }, [
    form.values.username,
    form.values.password_clear,
    form.values.password_confirm,
  ]);

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
    try {
      if (
        form.values.password_clear !== "" &&
        form.values.password_confirm !== "" &&
        form.values.email !== "" &&
        form.values.username !== ""
      ) {
    notifications.show({
      id: "user",
      title: "Loading New New User",
      message: "Please wait...",
      loading: true,
    })
        new_user.mutate({
          cookie: cookie,
          method: method,
          id: 1,
          username: `${form.values.username}`,
          password_clear: `${form.values.password_clear}`,
          email: `${form.values.email}`,
        })
        if (new_user.isError) {
          return notifications.update({
            id: "user",
            title: "New New User",
            color: "red",
            icon: <IconX />,
            message: `Failed to create, please try again`,
          })
        }
        if (new_user.isSuccess) {
          return notifications.update({
            id: "user",
            title: "New New User",
            color: "green",
            icon: <IconCheck />,
            autoClose: 5000,
            message: ``,
          })
        }
      }

    } catch (error) {
      notifications.update({
        id: "user",
        title: "New New User",
        color: "red",
        icon: <IconX />,
        message: `Error: ${error}`,
      })
    }
  }, [new_user, form.values.username, form.values.email, form.values.password_clear, form.values.password_confirm]);

  return (
    <Box component="form" maw={400} mx="auto" onSubmit={form.onSubmit(() => { })}>
      <TextInput
        label="Enter New User Username"
        placeholder="username"
        withAsterisk
        mt="md"
        {...form.getInputProps('username')}
      />
      <PasswordInput
        label="Enter New User Password"
        placeholder="XXXXXXXX"
        minLength={8}
        withAsterisk
        mt="md"
        {...form.getInputProps('password_clear')}
      />
      <PasswordInput
        label="Confurn New User Password"
        placeholder="XXXXXXXX"
        minLength={8}
        withAsterisk
        mt="md"
        {...form.getInputProps('password_confirm')}
      />
      <TextInput
        label="Enter New User Email"
        placeholder="email"
        withAsterisk
        mt="md"
        {...form.getInputProps('email')}
      />

      <Group position="left" mt="md">
        <Button type="submit" onClick={() => handleSubmit()}>Submit</Button>
      </Group>
    </Box>
  )
}

export default NewUser
