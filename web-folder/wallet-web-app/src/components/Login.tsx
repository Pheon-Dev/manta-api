import {
  Box, Button, Card, Group, LoadingOverlay, PasswordInput, Text, TextInput, Stepper, Code
} from "@mantine/core";
import { useForm, zodResolver } from "@mantine/form";
import { IconCheck, IconX } from "@tabler/icons";
import { signIn, useSession } from "next-auth/react";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { z } from "zod";
import { useMantaStore } from '../pages/_app';
import { v4 as uuidv4 } from 'uuid';

const schema = z.object({
  username: z.string().min(2, { message: "User Name Missing" }),
  password: z.string().min(2, { message: "Password Missing" }),
});


const Login = (props: {}) => {
  const { status } = useSession();
  const router = useRouter();
  const [active, setActive] = useState(0);
  const form = useForm({
    // validate: zodResolver(schema),
    initialValues: {
      username: "",
      password: "",
      name: '',
      email: '',
    },
    validate: (values) => {
      if (active === 0) {
        return {
          username:
            values.username.trim().length < 4
              ? 'Username must include at least 4 characters'
              : null,
          password:
            values.password.length < 6 ? 'Password must include at least 6 characters' : null,
        };
      }

      if (active === 1) {
        return {
          name: values.name.trim().length < 2 ? 'Name must include at least 2 characters' : null,
          email: /^\S+@\S+$/.test(values.email) ? null : 'Invalid email',
        };
      }

      return {};
    },
  });


  const nextStep = () =>
    setActive((current) => {
      if (form.validate().hasErrors) {
        return current;
      }
      return current < 3 ? current + 1 : current;
    });

  const prevStep = () => setActive((current) => (current > 0 ? current - 1 : current));

  const id_value = uuidv4().slice(0, 8);
  const setID = useMantaStore((state) => state.setID);
  const setEmail = useMantaStore((state) => state.setEmail);
  const setName = useMantaStore((state) => state.setName);
  const setUsername = useMantaStore((state) => state.setUsername);
  const handleSubmit = async () => {
    try {
      if (form.values.username && form.values.password) {
        const res = await signIn("credentials", {
          username: form.values.username,
          password: form.values.password,
          redirect: false,
        });
        if (res?.ok) {
          setID(id_value)
          setEmail(form.values.email)
          setName(form.values.name)
          setUsername(form.values.username)
        }
      }
    } catch (error) {
      console.log(error)
    }
  };

  return (
    <>
      <Card
        sx={{ maxWidth: 500 }}
        mx="auto"
        shadow="sm"
        p="xl"
        radius="md"
        withBorder
        style={{ marginTop: "200px", position: "relative" }}
      >
        <Card.Section>
          <Stepper active={active} breakpoint="sm" m="xl">
            <Stepper.Step label="First step" description="Enter Login Credentials">
              <TextInput label="Username" placeholder="Username" {...form.getInputProps('username')} />
              <PasswordInput
                mt="md"
                label="Password"
                placeholder="Password"
                {...form.getInputProps('password')}
              />
            </Stepper.Step>

            <Stepper.Step label="Second step" description="More Information">
              <TextInput label="Name" placeholder="Name" {...form.getInputProps('name')} />
              <TextInput mt="md" label="Email" placeholder="Email" {...form.getInputProps('email')} />
            </Stepper.Step>

            <Stepper.Completed>
              Verify your login credentials:
              <Code block mt="xl">
                {JSON.stringify({ name: form.values.name, username: form.values.username, email: form.values.email }, null, 2)}
              </Code>
            </Stepper.Completed>
          </Stepper>

          <Group position="right" mt="xl">
            {active !== 0 && (
              <Button variant="default" onClick={prevStep}>
                Back
              </Button>
            )}
            {active !== 2 && <Button onClick={nextStep}>Next step</Button>}
            {active === 2 && (
              <Button onClick={() => handleSubmit()}>
                Login
              </Button>
            )}
          </Group>
        </Card.Section>
      </Card>
    </>
  );
}

export default Login;
