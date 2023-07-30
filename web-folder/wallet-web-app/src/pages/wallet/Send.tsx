import { useForm, isNotEmpty, isEmail, isInRange, hasLength, matches } from '@mantine/form';
import { Button, Group, TextInput, NumberInput, Box, Textarea } from '@mantine/core';
import { useMantaStore } from '../_app';

const Send = () => {
  const form = useForm({
    initialValues: {
      sender: 'demo1',
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
  const handleSend = () => {
      try {
        if (form.values.amount && form.values.sender && form.values.receiver && form.values.description)
       send(form.values.amount) 
      } catch (e) {
       console.log(e) 
      }
    }

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
